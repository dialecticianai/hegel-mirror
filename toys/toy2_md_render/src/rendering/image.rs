use eframe::egui;
use std::collections::HashMap;
use std::fs;

/// Load an image from disk and convert to texture
pub fn load_image_texture(
    ctx: &egui::Context,
    image_path: &str,
    loaded_images: &mut HashMap<String, egui::TextureHandle>,
) {
    if loaded_images.contains_key(image_path) {
        return;
    }

    if let Ok(image_data) = fs::read(image_path) {
        if let Ok(image) = image::load_from_memory(&image_data) {
            let size = [image.width() as _, image.height() as _];
            let rgba = image.to_rgba8();
            let pixels = rgba.as_flat_samples();
            let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
            let texture =
                ctx.load_texture(image_path, color_image, egui::TextureOptions::default());
            loaded_images.insert(image_path.to_string(), texture);
        }
    }
}

/// Render an image chunk
pub fn render_image(
    ui: &mut egui::Ui,
    image_path: &str,
    loaded_images: &HashMap<String, egui::TextureHandle>,
) -> Option<egui::Response> {
    if let Some(texture) = loaded_images.get(image_path) {
        Some(ui.image(texture))
    } else {
        ui.label(format!("[Failed to load image: {}]", image_path));
        None
    }
}
