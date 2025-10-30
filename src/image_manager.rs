use eframe::egui;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Metadata for an image (dimensions without full texture)
#[derive(Debug, Clone)]
struct ImageMetadata {
    width: u32,
    height: u32,
    full_path: PathBuf,
}

/// Centralized image manager - loads metadata during parsing, textures on-demand
pub struct ImageManager {
    /// Image metadata cache (dimensions loaded during parsing)
    metadata: HashMap<String, ImageMetadata>,
    /// Lazy-loaded texture cache (created during rendering)
    textures: HashMap<String, egui::TextureHandle>,
    /// Base path for resolving relative image paths
    base_path: PathBuf,
}

impl ImageManager {
    /// Create a new ImageManager with the given base path
    pub fn new(base_path: impl AsRef<Path>) -> Self {
        Self {
            metadata: HashMap::new(),
            textures: HashMap::new(),
            base_path: base_path.as_ref().to_path_buf(),
        }
    }

    /// Load image metadata (dimensions only) - fast, no texture creation
    /// Returns (width, height) if successful, None if image cannot be loaded
    pub fn load_metadata(&mut self, path: &str) -> Option<(u32, u32)> {
        // Check cache first
        if let Some(metadata) = self.metadata.get(path) {
            return Some((metadata.width, metadata.height));
        }

        // Resolve full path
        let full_path = self.base_path.join(path);

        // Try to load image dimensions
        if let Ok(image_data) = std::fs::read(&full_path) {
            if let Ok(image) = image::load_from_memory(&image_data) {
                let width = image.width();
                let height = image.height();

                // Cache metadata
                self.metadata.insert(
                    path.to_string(),
                    ImageMetadata {
                        width,
                        height,
                        full_path,
                    },
                );

                return Some((width, height));
            }
        }

        None
    }

    /// Get dimensions for a previously loaded image
    /// Returns None if image metadata not loaded
    pub fn get_dimensions(&self, path: &str) -> Option<(u32, u32)> {
        self.metadata.get(path).map(|m| (m.width, m.height))
    }

    /// Get or load texture for rendering (lazy loading)
    /// First call loads the texture, subsequent calls return cached texture
    pub fn get_or_load_texture(
        &mut self,
        ctx: &egui::Context,
        path: &str,
    ) -> Option<&egui::TextureHandle> {
        // Check texture cache first
        if self.textures.contains_key(path) {
            return self.textures.get(path);
        }

        // Get metadata (must be loaded first)
        let metadata = self.metadata.get(path)?;

        // Load full image and create texture
        if let Ok(image_data) = std::fs::read(&metadata.full_path) {
            if let Ok(image) = image::load_from_memory(&image_data) {
                let size = [image.width() as _, image.height() as _];
                let rgba = image.to_rgba8();
                let pixels = rgba.as_flat_samples();
                let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
                let texture = ctx.load_texture(path, color_image, egui::TextureOptions::default());

                self.textures.insert(path.to_string(), texture);
                return self.textures.get(path);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    fn create_test_image(path: &Path, width: u32, height: u32) {
        use image::{ImageBuffer, Rgb};
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);
        img.save(path).unwrap();
    }

    #[test]
    fn test_load_metadata_valid_image() {
        let temp_dir = std::env::temp_dir().join("image_manager_test_1");
        fs::create_dir_all(&temp_dir).unwrap();

        let image_path = temp_dir.join("test.png");
        create_test_image(&image_path, 100, 50);

        let mut manager = ImageManager::new(&temp_dir);
        let result = manager.load_metadata("test.png");

        assert_eq!(result, Some((100, 50)));

        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_load_metadata_caching() {
        let temp_dir = std::env::temp_dir().join("image_manager_test_2");
        fs::create_dir_all(&temp_dir).unwrap();

        let image_path = temp_dir.join("test.png");
        create_test_image(&image_path, 200, 100);

        let mut manager = ImageManager::new(&temp_dir);

        // First load
        let result1 = manager.load_metadata("test.png");
        assert_eq!(result1, Some((200, 100)));

        // Delete the file
        fs::remove_file(&image_path).unwrap();

        // Second load should return cached value
        let result2 = manager.load_metadata("test.png");
        assert_eq!(result2, Some((200, 100)));

        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_load_metadata_missing_file() {
        let temp_dir = std::env::temp_dir().join("image_manager_test_3");
        fs::create_dir_all(&temp_dir).unwrap();

        let mut manager = ImageManager::new(&temp_dir);
        let result = manager.load_metadata("nonexistent.png");

        assert_eq!(result, None);

        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_load_metadata_corrupt_file() {
        let temp_dir = std::env::temp_dir().join("image_manager_test_4");
        fs::create_dir_all(&temp_dir).unwrap();

        let corrupt_path = temp_dir.join("corrupt.png");
        let mut file = fs::File::create(&corrupt_path).unwrap();
        file.write_all(b"not an image").unwrap();

        let mut manager = ImageManager::new(&temp_dir);
        let result = manager.load_metadata("corrupt.png");

        assert_eq!(result, None);

        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_get_dimensions() {
        let temp_dir = std::env::temp_dir().join("image_manager_test_5");
        fs::create_dir_all(&temp_dir).unwrap();

        let image_path = temp_dir.join("test.png");
        create_test_image(&image_path, 150, 75);

        let mut manager = ImageManager::new(&temp_dir);
        manager.load_metadata("test.png");

        let dims = manager.get_dimensions("test.png");
        assert_eq!(dims, Some((150, 75)));

        let missing = manager.get_dimensions("notloaded.png");
        assert_eq!(missing, None);

        fs::remove_dir_all(&temp_dir).ok();
    }
}
