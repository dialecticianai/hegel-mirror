use crate::models::TextChunk;
use crate::rendering::text_builder::{render_styled_text, TextContext};
use crate::theme::Theme;
use eframe::egui;

/// Render a plain text chunk with styling and drag sensing
pub fn render_text_chunk(ui: &mut egui::Ui, chunk: &TextChunk, theme: &Theme) -> egui::Response {
    // Determine rendering context
    let context = if let Some(level) = chunk.heading_level {
        TextContext::Heading(level)
    } else {
        TextContext::Body
    };

    // Render using centralized builder with emoji support
    // Note: EmojiLabel doesn't support .selectable(false), but that's OK
    // We handle selection via interact() below
    // Use line_start and byte_range as unique ID to avoid widget ID collisions
    let unique_id = ui.id().with((chunk.line_start, chunk.byte_range.start));
    let label = render_styled_text(
        ui,
        &chunk.text,
        chunk.bold,
        chunk.italic,
        chunk.code,
        context,
        theme,
        (chunk.line_start, chunk.byte_range.start),
    );

    // Sense drags on the rect for selection using our unique ID, not the label's ID
    ui.interact(label.rect, unique_id, egui::Sense::click_and_drag())
}
