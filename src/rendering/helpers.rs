/// Helper utilities for selection line calculation
use eframe::egui;

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
}
