use crate::models::TextChunk;
use eframe::egui;

/// Render a plain text chunk with styling
pub fn render_text_chunk(ui: &mut egui::Ui, chunk: &TextChunk) -> egui::Response {
    let mut text = egui::RichText::new(&chunk.text);

    if chunk.bold {
        text = text.strong();
    }
    if chunk.italic {
        text = text.italics();
    }
    if chunk.code {
        text = text.code();
    }
    if let Some(level) = chunk.heading_level {
        text = text.heading().size(24.0 - (level as f32 * 2.0));
    }

    ui.label(text)
}
