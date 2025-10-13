use crate::syntax::SyntaxHighlighter;
use eframe::egui;

/// Render a syntax-highlighted code block
pub fn render_code_block(
    ui: &mut egui::Ui,
    code: &str,
    lang: &str,
    highlighter: &SyntaxHighlighter,
) {
    let highlighted_lines = highlighter.highlight_code(code, lang);

    egui::Frame::NONE
        .fill(egui::Color32::from_rgb(43, 48, 59))
        .inner_margin(10.0)
        .corner_radius(4.0)
        .show(ui, |ui| {
            for line_ranges in highlighted_lines {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    for (style, text) in line_ranges {
                        let color = egui::Color32::from_rgb(
                            style.foreground.r,
                            style.foreground.g,
                            style.foreground.b,
                        );
                        ui.label(egui::RichText::new(text).color(color).monospace());
                    }
                });
            }
        });
}
