use crate::models::{Comment, LayoutMap, Selection, TextChunk};
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

/// Calculate which line within a chunk corresponds to a given Y position
/// Returns the precise line number based on interpolation within the chunk
fn calculate_line_from_y(
    line_start: usize,
    line_end: usize,
    chunk_y_start: f32,
    chunk_y_end: f32,
    y_pos: f32,
) -> usize {
    let chunk_height = chunk_y_end - chunk_y_start;
    let line_count = (line_end - line_start + 1).max(1);
    let y_offset = y_pos - chunk_y_start;
    let line_height = chunk_height / line_count as f32;
    let line_index = (y_offset / line_height).floor() as usize;
    line_start + line_index.min(line_count - 1)
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
    layout_map: &mut LayoutMap,
) {
    // Handle drag release
    if !ui.input(|i| i.pointer.any_down()) && selection.is_dragging {
        selection.end_drag();
    }

    for (idx, chunk) in chunks.iter_mut().enumerate() {
        let start_pos = ui.cursor().min;

        if let Some(image_path) = &chunk.image_path {
            // Check if in viewport using cached height
            let estimated_height = chunk.cached_height.unwrap_or(300.0);
            let approx_rect =
                egui::Rect::from_min_size(start_pos, egui::vec2(400.0, estimated_height));

            let before_y = ui.cursor().min.y;

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

            let after_y = ui.cursor().min.y;
            // Record position in layout map
            layout_map.record_chunk(chunk.line_start, chunk.line_end, before_y, after_y);

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

            let before_y = ui.cursor().min.y;

            if is_in_viewport(ui, approx_rect) {
                // Render and cache actual height
                code::render_code_block(ui, &chunk.text, lang, highlighter, theme);
                let after_y = ui.cursor().min.y;
                chunk.cached_height = Some(after_y - before_y);
            } else {
                // Use cached height for stable placeholder
                ui.add_space(estimated_height);
            }

            let after_y = ui.cursor().min.y;
            // Record position in layout map
            layout_map.record_chunk(chunk.line_start, chunk.line_end, before_y, after_y);

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

            let before_y = ui.cursor().min.y;

            if is_in_viewport(ui, approx_rect) {
                // Render and cache actual height
                table::render_table(ui, table_data, theme, idx);
                let after_y = ui.cursor().min.y;
                chunk.cached_height = Some(after_y - before_y);

                // Sense drag on the table area for selection
                let table_rect = egui::Rect::from_min_size(
                    start_pos,
                    egui::vec2(ui.available_width(), after_y - before_y),
                );
                let table_response = ui.interact(
                    table_rect,
                    ui.id().with(("table_sense", idx)),
                    egui::Sense::click_and_drag(),
                );

                if table_response.drag_started() {
                    // Calculate precise line where drag started
                    if let Some(interact_pos) = table_response.interact_pointer_pos() {
                        let precise_line = calculate_line_from_y(
                            chunk.line_start,
                            chunk.line_end,
                            before_y,
                            after_y,
                            interact_pos.y,
                        );
                        selection.start_drag(precise_line);
                    } else {
                        selection.start_drag(chunk.line_start);
                    }
                }
                // Don't call update_drag here - let the hover-based approach handle it
            } else {
                // Use cached height for stable placeholder
                ui.add_space(estimated_height);
            }

            let after_y = ui.cursor().min.y;
            // Record position in layout map
            layout_map.record_chunk(chunk.line_start, chunk.line_end, before_y, after_y);

            if chunk.newline_after {
                ui.add_space(theme.spacing.paragraph);
            }
        } else {
            // Text chunks: render with selection support
            let before_y = ui.cursor().min.y;

            // Render the text chunk
            let response = text::render_text_chunk(ui, chunk, theme);
            let after_y = ui.cursor().min.y;
            chunk.cached_height = Some(after_y - before_y);

            // Record position in layout map
            layout_map.record_chunk(chunk.line_start, chunk.line_end, before_y, after_y);

            // Handle selection via drag
            if response.drag_started() {
                // Calculate precise line where drag started
                if let Some(interact_pos) = response.interact_pointer_pos() {
                    let precise_line = calculate_line_from_y(
                        chunk.line_start,
                        chunk.line_end,
                        before_y,
                        after_y,
                        interact_pos.y,
                    );
                    selection.start_drag(precise_line);
                } else {
                    selection.start_drag(chunk.line_start);
                }
            }
            // Don't call update_drag here - let the hover-based approach handle it

            if chunk.newline_after {
                ui.add_space(theme.spacing.paragraph);
            }
        }
    }

    // Update selection based on hover (second pass after all chunks rendered)
    // This ensures bidirectional drag works (both up and down)
    if selection.is_dragging {
        if let Some(hover_pos) = ui.input(|i| i.pointer.hover_pos()) {
            // Use layout map to find which line we're hovering over
            for &(line_start, line_end, y_start, y_end) in &layout_map.chunks {
                if hover_pos.y >= y_start && hover_pos.y <= y_end {
                    let precise_line =
                        calculate_line_from_y(line_start, line_end, y_start, y_end, hover_pos.y);
                    selection.update_drag(precise_line);
                    break;
                }
            }
        }
    }

    // Draw selection bar using layout map
    if let (Some(start_line), Some(end_line)) = (selection.start_line, selection.end_line) {
        let (min_line, max_line) = if start_line <= end_line {
            (start_line, end_line)
        } else {
            (end_line, start_line)
        };

        if let Some((start_y, end_y)) = layout_map.get_y_range(min_line, max_line) {
            let bar_width = 4.0;
            let bar_x = ui.max_rect().right() - bar_width - 10.0; // 10px from right edge
            let bar_rect = egui::Rect::from_min_max(
                egui::pos2(bar_x, start_y),
                egui::pos2(bar_x + bar_width, end_y),
            );
            ui.painter()
                .rect_filled(bar_rect, 2.0, theme.colors.selection_highlight);
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
