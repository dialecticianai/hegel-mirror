/// Helper utilities for viewport culling and height caching
use crate::models::TextChunk;
use crate::theme::Theme;
use eframe::egui;

/// Check if a widget rect is within the visible viewport
pub fn is_in_viewport(ui: &egui::Ui, widget_rect: egui::Rect) -> bool {
    let viewport = ui.clip_rect();
    widget_rect.intersects(viewport)
}

/// Calculate which line within a chunk corresponds to a given Y position
/// Returns the precise line number based on interpolation within the chunk
pub fn calculate_line_from_y(
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

/// Estimate height for a text chunk based on line count
pub fn estimate_text_height(chunk: &TextChunk, theme: &Theme) -> f32 {
    let line_count = chunk.text.lines().count().max(1);
    line_count as f32 * theme.spacing.min_line_height
}

/// Estimate height for a code block based on line count
pub fn estimate_code_height(chunk: &TextChunk, theme: &Theme) -> f32 {
    let line_count = chunk.text.lines().count().max(1);
    (line_count as f32 * theme.spacing.min_line_height) + theme.spacing.code_block_padding * 2.0
}

/// Estimate height for a table based on row count
pub fn estimate_table_height(row_count: usize, theme: &Theme) -> f32 {
    (row_count as f32 + 1.0) * theme.spacing.min_line_height
}

/// Get or estimate cached height for a chunk
pub fn get_cached_or_estimated_height(chunk: &TextChunk, estimated: f32) -> f32 {
    chunk.cached_height.unwrap_or(estimated)
}

/// Create an approximate rect for viewport culling
pub fn create_approx_rect(start_pos: egui::Pos2, width: f32, height: f32) -> egui::Rect {
    egui::Rect::from_min_size(start_pos, egui::vec2(width, height))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_line_from_y() {
        // Chunk spanning lines 10-15 at Y positions 100.0-200.0
        // Each line is 100.0 / 6 = 16.67 pixels high

        // Click at very start (Y=100.0) should be line 10
        assert_eq!(calculate_line_from_y(10, 15, 100.0, 200.0, 100.0), 10);

        // Click in middle (Y=150.0) should be around line 13
        let mid_line = calculate_line_from_y(10, 15, 100.0, 200.0, 150.0);
        assert!(mid_line >= 12 && mid_line <= 13);

        // Click at end (Y=199.0) should be line 15
        assert_eq!(calculate_line_from_y(10, 15, 100.0, 200.0, 199.0), 15);
    }

    #[test]
    fn test_calculate_line_single_line_chunk() {
        // Single line chunk
        assert_eq!(calculate_line_from_y(5, 5, 50.0, 70.0, 60.0), 5);
    }

    #[test]
    fn test_estimate_text_height() {
        let theme = Theme::default_theme();
        let mut chunk = TextChunk {
            text: "line1\nline2\nline3".to_string(),
            byte_range: 0..5,
            line_start: 1,
            col_start: 1,
            line_end: 3,
            col_end: 5,
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
        };

        let height = estimate_text_height(&chunk, &theme);
        assert!(height > 0.0);
        assert_eq!(height, 3.0 * theme.spacing.min_line_height);
    }

    #[test]
    fn test_get_cached_height_when_present() {
        let mut chunk = TextChunk {
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
            cached_height: Some(42.0),
        };

        assert_eq!(get_cached_or_estimated_height(&chunk, 100.0), 42.0);
    }

    #[test]
    fn test_get_cached_height_falls_back_to_estimate() {
        let mut chunk = TextChunk {
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
        };

        assert_eq!(get_cached_or_estimated_height(&chunk, 100.0), 100.0);
    }
}
