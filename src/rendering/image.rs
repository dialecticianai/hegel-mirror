use crate::image_manager::ImageManager;
use eframe::egui;

/// Render an image chunk with optional alignment and width constraint
pub fn render_image(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    image_path: &str,
    image_manager: &mut ImageManager,
    alignment: Option<crate::models::Alignment>,
    width: Option<f32>,
) -> Option<egui::Response> {
    if let Some(texture) = image_manager.get_or_load_texture(ctx, image_path) {
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
