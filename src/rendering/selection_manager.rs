/// Centralized selection handling for all chunk types
use crate::models::{LayoutMap, Selection, TextChunk};
use crate::rendering::helpers::calculate_line_from_y;
use crate::theme::Theme;
use eframe::egui;

/// Manages selection interactions, updates, and visualization
pub struct SelectionManager<'a> {
    selection: &'a mut Selection,
    layout_map: &'a LayoutMap,
}

impl<'a> SelectionManager<'a> {
    /// Create a new selection manager
    pub fn new(selection: &'a mut Selection, layout_map: &'a LayoutMap) -> Self {
        Self {
            selection,
            layout_map,
        }
    }

    /// Handle selection interaction for a chunk (drag start, click)
    /// Returns true if the chunk was clicked (for click-to-clear detection)
    pub fn handle_interaction(
        &mut self,
        response: &egui::Response,
        chunk: &TextChunk,
        before_y: f32,
        after_y: f32,
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
                self.selection.start_drag(precise_line);
            } else {
                self.selection.start_drag(chunk.line_start);
            }
        }

        // Track clicks for click-to-clear
        if response.clicked() {
            was_clicked = true;
        }

        was_clicked
    }

    /// Handle selection interaction for single-line elements (images)
    /// Automatically completes the drag after starting
    pub fn handle_single_line_interaction(&mut self, response: &egui::Response, line: usize) {
        if response.clicked() {
            self.selection.start_drag(line);
            self.selection.end_drag();
        }
    }

    /// Update selection based on hover position during drag
    /// Call this once per frame after all chunks are rendered
    pub fn update_from_hover(&mut self, ui: &egui::Ui) {
        if !self.selection.is_dragging {
            return;
        }

        if let Some(hover_pos) = ui.input(|i| i.pointer.hover_pos()) {
            // Use layout map to find which line we're hovering over
            for &(line_start, line_end, y_start, y_end) in &self.layout_map.chunks {
                if hover_pos.y >= y_start && hover_pos.y <= y_end {
                    let precise_line =
                        calculate_line_from_y(line_start, line_end, y_start, y_end, hover_pos.y);
                    self.selection.update_drag(precise_line);
                    break;
                }
            }
        }
    }

    /// End drag if pointer is released
    /// Call this once per frame before rendering
    pub fn handle_drag_release(&mut self, ui: &egui::Ui) {
        if !ui.input(|i| i.pointer.any_down()) && self.selection.is_dragging {
            self.selection.end_drag();
        }
    }

    /// Draw the selection highlight bar
    pub fn draw_selection_bar(&self, ui: &mut egui::Ui, theme: &Theme) {
        if let (Some(start_line), Some(end_line)) =
            (self.selection.start_line, self.selection.end_line)
        {
            let (min_line, max_line) = if start_line <= end_line {
                (start_line, end_line)
            } else {
                (end_line, start_line)
            };

            if let Some((start_y, end_y)) = self.layout_map.get_y_range(min_line, max_line) {
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
}
