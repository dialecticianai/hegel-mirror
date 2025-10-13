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

/// Handle common selection interactions (drag start, click) for any response
/// Returns true if clicked (for click-to-clear detection)
fn handle_selection_interaction(
    response: &egui::Response,
    chunk: &TextChunk,
    before_y: f32,
    after_y: f32,
    selection: &mut Selection,
) -> bool {
    let mut was_clicked = false;

    // Handle drag start with precise line calculation
    if response.drag_started() {
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

    // Track clicks for click-to-clear
    if response.clicked() {
        was_clicked = true;
    }

    was_clicked
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

    // Track if any chunk was clicked (to distinguish click from drag)
    let mut chunk_was_clicked = false;

    // Only build layout map if we have an active selection or are dragging
    let need_layout_map = selection.is_active() || selection.is_dragging;

    // Get viewport bounds for early exit optimization
    let viewport = ui.clip_rect();
    let mut past_viewport = false;

    for (idx, chunk) in chunks.iter_mut().enumerate() {
        let start_pos = ui.cursor().min;

        // Early exit if we're well past the viewport and have cached heights
        // This is safe because we maintain Y positions via add_space
        if past_viewport && chunk.cached_height.is_some() {
            // Skip processing this chunk entirely
            let estimated_height = chunk.cached_height.unwrap();
            ui.add_space(estimated_height);
            if chunk.newline_after {
                ui.add_space(theme.spacing.paragraph);
            }
            continue;
        }

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
            // Record position in layout map (only if needed for selection)
            if need_layout_map {
                layout_map.record_chunk(chunk.line_start, chunk.line_end, before_y, after_y);
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
            // Record position in layout map (only if needed for selection)
            if need_layout_map {
                layout_map.record_chunk(chunk.line_start, chunk.line_end, before_y, after_y);
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

                // Handle selection interactions (unified with text chunks)
                if handle_selection_interaction(
                    &table_response,
                    chunk,
                    before_y,
                    after_y,
                    selection,
                ) {
                    chunk_was_clicked = true;
                }
            } else {
                // Use cached height for stable placeholder
                ui.add_space(estimated_height);
            }

            let after_y = ui.cursor().min.y;
            // Record position in layout map (only if needed for selection)
            if need_layout_map {
                layout_map.record_chunk(chunk.line_start, chunk.line_end, before_y, after_y);
            }

            if chunk.newline_after {
                ui.add_space(theme.spacing.paragraph);
            }
        } else {
            // Text chunks: render with selection support
            let before_y = ui.cursor().min.y;

            // Estimate height based on line count
            let line_count = chunk.text.lines().count().max(1);
            let estimated_height = line_count as f32 * theme.spacing.min_line_height;
            let approx_rect =
                egui::Rect::from_min_size(start_pos, egui::vec2(600.0, estimated_height));

            if is_in_viewport(ui, approx_rect) {
                // In viewport - render and cache actual height
                let response = text::render_text_chunk(ui, chunk, theme);
                let after_y = ui.cursor().min.y;
                chunk.cached_height = Some(after_y - before_y);

                // Record position in layout map (only if needed for selection)
                if need_layout_map {
                    layout_map.record_chunk(chunk.line_start, chunk.line_end, before_y, after_y);
                }

                // Handle selection interactions (unified with table chunks)
                if handle_selection_interaction(&response, chunk, before_y, after_y, selection) {
                    chunk_was_clicked = true;
                }
            } else {
                // Outside viewport - use cached height if available, otherwise estimate
                let height = chunk.cached_height.unwrap_or(estimated_height);
                ui.add_space(height);
                let after_y = ui.cursor().min.y;

                // Record position in layout map (only if needed for selection)
                if need_layout_map {
                    layout_map.record_chunk(chunk.line_start, chunk.line_end, before_y, after_y);
                }
            }

            if chunk.newline_after {
                ui.add_space(theme.spacing.paragraph);
            }
        }

        // Check if we've moved past the viewport bottom
        // Once past, we can start skipping expensive processing
        if !past_viewport && start_pos.y > viewport.max.y + 1000.0 {
            past_viewport = true;
        }
    }

    // Clear selection if clicked without dragging
    if chunk_was_clicked && !selection.is_dragging {
        selection.clear();
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

/// Render the comment UI section as a floating panel in the right margin
pub fn render_comment_section(
    ctx: &egui::Context,
    layout_map: &LayoutMap,
    selection: &Selection,
    comment_text: &mut String,
    comments: &mut Vec<Comment>,
    theme: &Theme,
) {
    // Only show if there's an active selection
    if let (Some(start_line), Some(end_line)) = (selection.start_line, selection.end_line) {
        let (min_line, max_line) = if start_line <= end_line {
            (start_line, end_line)
        } else {
            (end_line, start_line)
        };

        // Get the Y position for the selection start and end
        if let Some(selection_y_start) = layout_map.get_line_y(min_line) {
            let selection_y_end = layout_map.get_line_y(max_line).unwrap_or(selection_y_start);

            // Position the comment box at the selection Y position, in the right margin
            let screen_width = ctx.screen_rect().width();
            let screen_height = ctx.screen_rect().height();
            let window_x = screen_width
                - theme.layout.comment_box_width
                - theme.layout.comment_box_margin_right;

            // Clamp comment box to viewport with some padding
            let viewport_padding = 20.0;
            let clamped_y = selection_y_start
                .max(viewport_padding)
                .min(screen_height - theme.layout.comment_box_height - viewport_padding);

            // Determine if selection is off-screen
            let selection_above_viewport = selection_y_end < viewport_padding;
            let selection_below_viewport = selection_y_start > screen_height - viewport_padding;

            egui::Window::new("Add Comment")
                .fixed_pos(egui::pos2(window_x, clamped_y))
                .fixed_size(egui::vec2(
                    theme.layout.comment_box_width,
                    theme.layout.comment_box_height,
                ))
                .resizable(false)
                .collapsible(false)
                .title_bar(false)
                .frame(egui::Frame::window(&ctx.style()).inner_margin(10.0))
                .show(ctx, |ui| {
                    // Show scroll indicator if selection is off-screen
                    if selection_above_viewport {
                        ui.horizontal(|ui| {
                            ui.label("↑");
                            ui.label(egui::RichText::new("Scroll up to see selection").weak());
                        });
                        ui.separator();
                    } else if selection_below_viewport {
                        ui.horizontal(|ui| {
                            ui.label("↓");
                            ui.label(egui::RichText::new("Scroll down to see selection").weak());
                        });
                        ui.separator();
                    }

                    ui.label(format!("Selection: Lines {}-{}", min_line, max_line));
                    ui.add_space(5.0);

                    ui.label("Comment:");
                    ui.text_edit_multiline(comment_text);
                    ui.add_space(5.0);

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
    }

    // Show existing comments list at the bottom (could be made floating too)
    if !comments.is_empty() {
        let screen_height = ctx.screen_rect().height();
        egui::Window::new("Comments")
            .fixed_pos(egui::pos2(
                theme.layout.comments_list_margin_left,
                screen_height - theme.layout.comments_list_margin_bottom,
            ))
            .fixed_size(egui::vec2(
                theme.layout.comments_list_width,
                theme.layout.comments_list_height,
            ))
            .resizable(false)
            .collapsible(true)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for comment in comments {
                        ui.label(comment.format());
                        ui.separator();
                    }
                });
            });
    }
}
