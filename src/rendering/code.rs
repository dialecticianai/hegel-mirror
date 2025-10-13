use crate::syntax::SyntaxHighlighter;
use crate::theme::Theme;
use eframe::egui;

/// Render a syntax-highlighted code block
pub fn render_code_block(
    ui: &mut egui::Ui,
    code: &str,
    lang: &str,
    highlighter: &SyntaxHighlighter,
    theme: &Theme,
) {
    let highlighted_lines = highlighter.highlight_code(code, lang);

    egui::Frame::NONE
        .fill(theme.colors.code_block_bg)
        .inner_margin(theme.spacing.code_block_padding)
        .corner_radius(theme.spacing.corner_radius)
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
                        ui.label(
                            egui::RichText::new(text)
                                .color(color)
                                .monospace()
                                .size(theme.typography.code_block_size),
                        );
                    }
                });
            }
        });
}
