use crate::models::TextChunk;
use crate::theme::Theme;
use eframe::egui;

/// Render a plain text chunk with styling and drag sensing
pub fn render_text_chunk(ui: &mut egui::Ui, chunk: &TextChunk, theme: &Theme) -> egui::Response {
    let mut text = egui::RichText::new(&chunk.text);

    // Apply bold styling - egui's strong() makes text use stronger/brighter color
    // Note: egui doesn't have native font-weight support for true bold rendering
    // To get actual bold fonts, we'd need to load and use a bold font family
    if chunk.bold {
        text = text.strong();
    }
    if chunk.italic {
        text = text.italics();
    }
    if chunk.code {
        text = text
            .code()
            .size(theme.typography.code_size)
            .color(theme.colors.inline_code);
    }
    if let Some(level) = chunk.heading_level {
        let size = theme.typography.heading_sizes[(level - 1).min(5) as usize];
        text = text.heading().size(size).color(theme.colors.heading);
    } else {
        text = text
            .size(theme.typography.body_size)
            .color(theme.colors.text);
    }

    // Render label with selection disabled, then sense drags on the rect
    let label = ui.add(egui::Label::new(text).selectable(false));
    ui.interact(label.rect, label.id, egui::Sense::click_and_drag())
}
