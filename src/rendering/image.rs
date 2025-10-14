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

/// Render an image chunk with optional alignment and width constraint
pub fn render_image(
    ui: &mut egui::Ui,
    image_path: &str,
    loaded_images: &HashMap<String, egui::TextureHandle>,
    alignment: Option<crate::models::Alignment>,
    width: Option<f32>,
) -> Option<egui::Response> {
    if let Some(texture) = loaded_images.get(image_path) {
        // Calculate display size (respecting width constraint if provided)
        let original_size = texture.size_vec2();
        let display_size = if let Some(desired_width) = width {
            // Maintain aspect ratio
            let aspect_ratio = original_size.y / original_size.x;
            egui::vec2(desired_width, desired_width * aspect_ratio)
        } else {
            original_size
        };

        // Apply alignment and render the image
        let response = match alignment {
            Some(crate::models::Alignment::Center) => {
                // Center the image by wrapping in a horizontal layout
                ui.horizontal(|ui| {
                    // Calculate left padding to center the image
                    let available_width = ui.available_width();
                    let left_padding = (available_width - display_size.x) / 2.0;
                    if left_padding > 0.0 {
                        ui.add_space(left_padding);
                    }
                    ui.add(egui::Image::new(texture).fit_to_exact_size(display_size))
                })
                .inner
            }
            Some(crate::models::Alignment::Right) => {
                // Right-align the image
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    ui.add(egui::Image::new(texture).fit_to_exact_size(display_size))
                })
                .inner
            }
            Some(crate::models::Alignment::Left) | None => {
                // Left-align (default)
                ui.add(egui::Image::new(texture).fit_to_exact_size(display_size))
            }
        };

        Some(response)
    } else {
        ui.label(format!("[Failed to load image: {}]", image_path));
        None
    }
}
