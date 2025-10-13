/// Layout information for selection bar rendering
/// Maps line numbers to Y coordinates in the rendered document
#[derive(Default)]
pub struct LayoutMap {
    /// Chunks with their line ranges and Y positions
    /// Each entry: (line_start, line_end, y_start, y_end)
    pub chunks: Vec<(usize, usize, f32, f32)>,

    /// Legacy field for compatibility during refactor
    pub line_positions: Vec<(usize, f32, f32)>,
}

/// Calculate the height of a single line within a chunk
fn calculate_line_height(line_start: usize, line_end: usize, chunk_height: f32) -> f32 {
    let line_count = (line_end - line_start + 1).max(1);
    chunk_height / line_count as f32
}

impl LayoutMap {
    pub fn new() -> Self {
        Self::default()
    }

    /// Clear all cached positions (call at start of frame)
    pub fn clear(&mut self) {
        self.chunks.clear();
        self.line_positions.clear();
    }

    /// Record that lines [line_start..=line_end] occupy Y range [y_start, y_end]
    pub fn record_chunk(&mut self, line_start: usize, line_end: usize, y_start: f32, y_end: f32) {
        if line_start > 0 {
            self.chunks.push((line_start, line_end, y_start, y_end));
            // Keep legacy format for backward compat
            self.line_positions.push((line_start, y_start, y_end));
        }
    }

    /// Get the Y position for a specific line number
    /// For multi-line chunks, interpolates within the chunk
    pub fn get_line_y(&self, line: usize) -> Option<f32> {
        for &(line_start, line_end, y_start, y_end) in &self.chunks {
            if line >= line_start && line <= line_end {
                let chunk_height = y_end - y_start;
                let line_height = calculate_line_height(line_start, line_end, chunk_height);
                let line_offset = (line - line_start) as f32;
                return Some(y_start + line_offset * line_height);
            }
        }
        None
    }

    /// Get the Y range that spans the given line range
    /// Returns (min_y, max_y) for the selection bar with line-precise boundaries
    pub fn get_y_range(&self, min_line: usize, max_line: usize) -> Option<(f32, f32)> {
        let start_y = self.get_line_y(min_line)?;
        let end_y = self.get_line_y(max_line).map(|y| {
            // Find the chunk containing max_line to get its line height
            for &(line_start, line_end, y_start, y_end) in &self.chunks {
                if max_line >= line_start && max_line <= line_end {
                    let chunk_height = y_end - y_start;
                    let line_height = calculate_line_height(line_start, line_end, chunk_height);
                    return y + line_height;
                }
            }
            y
        })?;

        Some((start_y, end_y))
    }
}
