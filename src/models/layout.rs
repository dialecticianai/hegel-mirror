/// Layout information for selection bar rendering
/// Maps line numbers to Y coordinates in the rendered document
#[derive(Default)]
pub struct LayoutMap {
    /// Map from line number to (y_start, y_end) in screen coordinates
    /// This is computed during the render pass
    line_positions: Vec<(usize, f32, f32)>, // (line_number, y_start, y_end)
}

impl LayoutMap {
    pub fn new() -> Self {
        Self::default()
    }

    /// Clear all cached positions (call at start of frame)
    pub fn clear(&mut self) {
        self.line_positions.clear();
    }

    /// Record that lines [line_start..=line_end] occupy Y range [y_start, y_end]
    pub fn record_chunk(&mut self, line_start: usize, line_end: usize, y_start: f32, y_end: f32) {
        // For multi-line chunks, we store just the start and end
        // This is sufficient for selection bar rendering
        if line_start > 0 {
            self.line_positions.push((line_start, y_start, y_end));
        }
    }

    /// Get the Y range that spans the given line range
    /// Returns (min_y, max_y) for the selection bar
    pub fn get_y_range(&self, min_line: usize, max_line: usize) -> Option<(f32, f32)> {
        let mut min_y = None;
        let mut max_y = None;

        for &(line, y_start, y_end) in &self.line_positions {
            // Check if this chunk overlaps with the selection
            if line >= min_line && line <= max_line {
                min_y = Some(min_y.map_or(y_start, |current: f32| current.min(y_start)));
                max_y = Some(max_y.map_or(y_end, |current: f32| current.max(y_end)));
            }
        }

        match (min_y, max_y) {
            (Some(min), Some(max)) => Some((min, max)),
            _ => None,
        }
    }
}
