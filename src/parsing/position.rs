/// Precomputed line offset table for fast byte-to-line conversion
pub struct LineOffsets {
    offsets: Vec<usize>, // Byte offset of each line start
}

impl LineOffsets {
    /// Build line offset table in O(n) time
    pub fn new(source: &str) -> Self {
        let mut offsets = vec![0]; // Line 1 starts at byte 0
        for (idx, ch) in source.char_indices() {
            if ch == '\n' {
                offsets.push(idx + 1); // Next line starts after newline
            }
        }
        Self { offsets }
    }

    /// Convert byte offset to (line, col) in O(log n) time - both 1-indexed
    pub fn byte_to_line_col(&self, source: &str, byte_offset: usize) -> (usize, usize) {
        // Binary search to find line number
        let line = match self.offsets.binary_search(&byte_offset) {
            Ok(idx) => idx + 1, // Exact match on line start
            Err(idx) => idx,    // Insert position gives line number
        };

        // Calculate column by scanning from line start
        let line_start = if line > 0 { self.offsets[line - 1] } else { 0 };
        let col = source[line_start..byte_offset].chars().count() + 1;

        (line, col)
    }
}

/// Convert byte offset to (line, col) - both 1-indexed (legacy function for tests)
#[allow(dead_code)]
pub fn byte_to_line_col(source: &str, byte_offset: usize) -> (usize, usize) {
    let offsets = LineOffsets::new(source);
    offsets.byte_to_line_col(source, byte_offset)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_to_line_col() {
        let source = "line 1\nline 2\nline 3";
        assert_eq!(byte_to_line_col(source, 0), (1, 1));
        assert_eq!(byte_to_line_col(source, 7), (2, 1));
        assert_eq!(byte_to_line_col(source, 14), (3, 1));
    }
}
