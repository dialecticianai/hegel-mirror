use crate::models::{Comment, Selection, TextChunk};
use crate::rendering::{code, image, text};
use crate::syntax::SyntaxHighlighter;
use eframe::egui;
use std::collections::HashMap;

/// Check if a widget rect is within the visible viewport
fn is_in_viewport(ui: &egui::Ui, widget_rect: egui::Rect) -> bool {
    let viewport = ui.clip_rect();
    widget_rect.intersects(viewport)
}

/// Render the main UI with markdown content (with stable lazy loading)
pub fn render_content(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    chunks: &mut [TextChunk],
    selection: &mut Selection,
    loaded_images: &mut HashMap<String, egui::TextureHandle>,
    highlighter: &SyntaxHighlighter,
) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        for (idx, chunk) in chunks.iter_mut().enumerate() {
            let start_pos = ui.cursor().min;

            if let Some(image_path) = &chunk.image_path {
                // Check if in viewport using cached height
                let estimated_height = chunk.cached_height.unwrap_or(300.0);
                let approx_rect =
                    egui::Rect::from_min_size(start_pos, egui::vec2(400.0, estimated_height));

                if is_in_viewport(ui, approx_rect) {
                    // Load and render, cache actual height
                    image::load_image_texture(ctx, image_path, loaded_images);
                    if let Some(response) = image::render_image(ui, image_path, loaded_images) {
                        chunk.cached_height = Some(response.rect.height());
                        if response.clicked() {
                            selection.set_single_chunk(idx, chunk.text.len());
                        }
                    }
                } else {
                    // Use cached height for stable placeholder
                    ui.add_space(estimated_height);
                }
                if chunk.newline_after {
                    ui.add_space(4.0);
                }
            } else if let Some(lang) = &chunk.code_block_lang {
                // Check if in viewport using cached height
                let line_count = chunk.text.lines().count().max(1);
                let estimated_height = chunk
                    .cached_height
                    .unwrap_or((line_count as f32 * 16.0) + 20.0);
                let approx_rect =
                    egui::Rect::from_min_size(start_pos, egui::vec2(600.0, estimated_height));

                if is_in_viewport(ui, approx_rect) {
                    // Render and cache actual height
                    let before_y = ui.cursor().min.y;
                    code::render_code_block(ui, &chunk.text, lang, highlighter);
                    let after_y = ui.cursor().min.y;
                    chunk.cached_height = Some(after_y - before_y);
                } else {
                    // Use cached height for stable placeholder
                    ui.add_space(estimated_height);
                }
                if chunk.newline_after {
                    ui.add_space(4.0);
                }
            } else {
                // Text is cheap, always render
                let response = text::render_text_chunk(ui, chunk);
                if response.clicked() {
                    selection.set_single_chunk(idx, chunk.text.len());
                }
                if chunk.newline_after {
                    ui.add_space(4.0);
                }
            }
        }
    });
}

/// Render the comment UI section
pub fn render_comment_section(
    ui: &mut egui::Ui,
    chunks: &[TextChunk],
    selection: &Selection,
    comment_text: &mut String,
    comments: &mut Vec<Comment>,
) {
    ui.separator();

    // Show current selection info
    if let (Some(start), Some(end)) = (selection.start_chunk, selection.end_chunk) {
        if start < chunks.len() && end < chunks.len() {
            let start_chunk = &chunks[start];
            let end_chunk = &chunks[end];

            ui.label(format!(
                "Selection: L{}:C{} → L{}:C{}",
                start_chunk.line_start,
                start_chunk.col_start,
                end_chunk.line_end,
                end_chunk.col_end
            ));

            ui.horizontal(|ui| {
                ui.label("Comment:");
                ui.text_edit_singleline(comment_text);
                if ui.button("Add Comment").clicked() && !comment_text.is_empty() {
                    comments.push(Comment::new(
                        comment_text.clone(),
                        start_chunk.line_start,
                        start_chunk.col_start,
                        end_chunk.line_end,
                        end_chunk.col_end,
                    ));
                    comment_text.clear();
                }
            });
        }
    }

    ui.separator();
    ui.heading("Comments");
    for comment in comments {
        ui.label(comment.format());
    }
}
