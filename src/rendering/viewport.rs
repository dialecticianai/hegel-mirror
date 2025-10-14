/// Viewport culling with height caching for stable lazy rendering
use crate::models::TextChunk;
use crate::theme::Theme;
use eframe::egui;

/// Manages viewport culling decisions and cached height rendering
pub struct ViewportCuller {
    viewport: egui::Rect,
    past_viewport: bool,
}

impl ViewportCuller {
    /// Create a new viewport culler for the current frame
    pub fn new(ui: &egui::Ui) -> Self {
        Self {
            viewport: ui.clip_rect(),
            past_viewport: false,
        }
    }

    /// Check if we should render a chunk at the given position with estimated height
    /// Returns true if the chunk is in viewport or might be visible
    pub fn should_render(&mut self, start_pos: egui::Pos2, _estimated_height: f32) -> bool {
        // If we haven't passed the viewport yet, always render
        if !self.past_viewport {
            // Check if we've now moved past the viewport bottom
            // Add 1000px buffer for smooth scrolling
            if start_pos.y > self.viewport.max.y + 1000.0 {
                self.past_viewport = true;
            }
            return true;
        }

        // Once past viewport, only render if we have a reason to
        // (e.g., no cached height means we need to measure it once)
        false
    }

    /// Render a chunk that's outside the viewport using cached or estimated height
    /// Returns the height used for spacing
    pub fn render_offscreen(
        &self,
        ui: &mut egui::Ui,
        chunk: &mut TextChunk,
        estimated_height: f32,
        theme: &Theme,
    ) -> f32 {
        let height = chunk.cached_height.unwrap_or(estimated_height);
        ui.add_space(height);

        // Add paragraph spacing if needed
        if chunk.newline_after {
            ui.add_space(theme.spacing.paragraph);
        }

        height
    }

    /// Check if a rect intersects with the viewport
    pub fn intersects_viewport(&self, rect: egui::Rect) -> bool {
        rect.intersects(self.viewport)
    }

    /// Create an approximate rect for viewport testing
    pub fn create_approx_rect(start_pos: egui::Pos2, width: f32, height: f32) -> egui::Rect {
        egui::Rect::from_min_size(start_pos, egui::vec2(width, height))
    }

    /// Get cached height or use estimated height
    pub fn get_height(chunk: &TextChunk, estimated: f32) -> f32 {
        chunk.cached_height.unwrap_or(estimated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_approx_rect() {
        let pos = egui::pos2(10.0, 20.0);
        let rect = ViewportCuller::create_approx_rect(pos, 100.0, 50.0);

        assert_eq!(rect.min, pos);
        assert_eq!(rect.width(), 100.0);
        assert_eq!(rect.height(), 50.0);
    }

    #[test]
    fn test_get_height_with_cache() {
        let mut chunk = create_test_chunk();
        chunk.cached_height = Some(42.0);

        assert_eq!(ViewportCuller::get_height(&chunk, 100.0), 42.0);
    }

    #[test]
    fn test_get_height_without_cache() {
        let chunk = create_test_chunk();
        assert_eq!(ViewportCuller::get_height(&chunk, 100.0), 100.0);
    }

    fn create_test_chunk() -> TextChunk {
        TextChunk {
            text: "test".to_string(),
            byte_range: 0..4,
            line_start: 1,
            col_start: 1,
            line_end: 1,
            col_end: 4,
            bold: false,
            italic: false,
            code: false,
            heading_level: None,
            newline_after: false,
            image_path: None,
            alignment: None,
            image_width: None,
            code_block_lang: None,
            table: None,
            cached_height: None,
        }
    }
}
