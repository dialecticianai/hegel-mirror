/// Convert byte offset to (line, col) - both 1-indexed
pub fn byte_to_line_col(source: &str, byte_offset: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;

    for (idx, ch) in source.char_indices() {
        if idx >= byte_offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }

    (line, col)
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
