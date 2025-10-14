use crate::models::{LayoutMap, Selection, TextChunk};
use crate::rendering::chunk;
use crate::rendering::helpers::calculate_line_from_y;
use crate::syntax::SyntaxHighlighter;
use crate::theme::Theme;
use eframe::egui;
use std::collections::HashMap;

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

        // Render the chunk using unified rendering logic
        let was_clicked = chunk::render_chunk(
            ui,
            ctx,
            chunk,
            idx,
            selection,
            loaded_images,
            highlighter,
            theme,
            layout_map,
            need_layout_map,
        );

        if was_clicked {
            chunk_was_clicked = true;
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
