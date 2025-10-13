use crate::models::{Comment, Selection, TextChunk};
use crate::rendering::{code, image, table, text};
use crate::syntax::SyntaxHighlighter;
use crate::theme::Theme;
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
    theme: &Theme,
) {
    // Handle drag release
    if !ui.input(|i| i.pointer.any_down()) && selection.is_dragging {
        selection.end_drag();
    }

    for (idx, chunk) in chunks.iter_mut().enumerate() {
        let start_pos = ui.cursor().min;

        // Check if this chunk's lines are in the selected range
        let chunk_selected =
            (chunk.line_start..=chunk.line_end).any(|line| selection.contains_line(line));

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
                    // Images can be selected by clicking (single line selection)
                    if response.clicked() {
                        selection.start_drag(chunk.line_start);
                        selection.end_drag();
                    }
                }
            } else {
                // Use cached height for stable placeholder
                ui.add_space(estimated_height);
            }
            if chunk.newline_after {
                ui.add_space(theme.spacing.paragraph);
            }
        } else if let Some(lang) = &chunk.code_block_lang {
            // Check if in viewport using cached height
            let line_count = chunk.text.lines().count().max(1);
            let estimated_height = chunk.cached_height.unwrap_or(
                (line_count as f32 * theme.spacing.min_line_height)
                    + theme.spacing.code_block_padding * 2.0,
            );
            let approx_rect =
                egui::Rect::from_min_size(start_pos, egui::vec2(600.0, estimated_height));

            if is_in_viewport(ui, approx_rect) {
                // Render and cache actual height
                let before_y = ui.cursor().min.y;
                code::render_code_block(ui, &chunk.text, lang, highlighter, theme);
                let after_y = ui.cursor().min.y;
                chunk.cached_height = Some(after_y - before_y);
            } else {
                // Use cached height for stable placeholder
                ui.add_space(estimated_height);
            }
            if chunk.newline_after {
                ui.add_space(theme.spacing.paragraph);
            }
        } else if let Some(table_data) = &chunk.table {
            // Check if in viewport using cached height
            let estimated_height = chunk
                .cached_height
                .unwrap_or((table_data.rows.len() as f32 + 1.0) * theme.spacing.min_line_height);
            let approx_rect =
                egui::Rect::from_min_size(start_pos, egui::vec2(600.0, estimated_height));

            if is_in_viewport(ui, approx_rect) {
                // Render and cache actual height
                let before_y = ui.cursor().min.y;
                table::render_table(ui, table_data, theme, idx);
                let after_y = ui.cursor().min.y;
                chunk.cached_height = Some(after_y - before_y);
            } else {
                // Use cached height for stable placeholder
                ui.add_space(estimated_height);
            }
            if chunk.newline_after {
                ui.add_space(theme.spacing.paragraph);
            }
        } else {
            // Text chunks: render with selection support
            let before_y = ui.cursor().min.y;

            // Paint selection highlight if this chunk is selected
            if chunk_selected {
                let estimated_height = chunk.cached_height.unwrap_or(theme.spacing.min_line_height);
                let highlight_rect = egui::Rect::from_min_size(
                    start_pos,
                    egui::vec2(ui.available_width(), estimated_height),
                );
                ui.painter()
                    .rect_filled(highlight_rect, 0.0, theme.colors.selection_highlight);
            }

            // Render the text chunk
            let response = text::render_text_chunk(ui, chunk, theme);
            let after_y = ui.cursor().min.y;
            chunk.cached_height = Some(after_y - before_y);

            // Handle selection via drag
            if response.drag_started() {
                selection.start_drag(chunk.line_start);
            } else if response.dragged() {
                selection.update_drag(chunk.line_start);
            }

            if chunk.newline_after {
                ui.add_space(theme.spacing.paragraph);
            }
        }
    }
}

/// Render the comment UI section
pub fn render_comment_section(
    ui: &mut egui::Ui,
    _chunks: &[TextChunk],
    selection: &Selection,
    comment_text: &mut String,
    comments: &mut Vec<Comment>,
    _theme: &Theme,
) {
    ui.separator();

    // Show current selection info
    if let (Some(start_line), Some(end_line)) = (selection.start_line, selection.end_line) {
        let (min_line, max_line) = if start_line <= end_line {
            (start_line, end_line)
        } else {
            (end_line, start_line)
        };

        ui.label(format!("Selection: Lines {}-{}", min_line, max_line));

        ui.horizontal(|ui| {
            ui.label("Comment:");
            ui.text_edit_singleline(comment_text);
            if ui.button("Add Comment").clicked() && !comment_text.is_empty() {
                comments.push(Comment::new(
                    comment_text.clone(),
                    min_line,
                    0, // col_start: 0 (beginning of line)
                    max_line,
                    0, // col_end: 0 (simplified for MVP)
                ));
                comment_text.clear();
            }
        });
    }

    ui.separator();
    ui.heading("Comments");
    for comment in comments {
        ui.label(comment.format());
    }
}
