use crate::models::TextChunk;
use crate::theme::Theme;
use eframe::egui;

/// Render a plain text chunk with styling
pub fn render_text_chunk(ui: &mut egui::Ui, chunk: &TextChunk, theme: &Theme) -> egui::Response {
    let mut text = egui::RichText::new(&chunk.text);

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

    // Make label sense drags for selection
    ui.label(text).interact(egui::Sense::click_and_drag())
}
