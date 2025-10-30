/// Batch consecutive inline text chunks into horizontal_wrapped layouts
use crate::models::TextChunk;

/// Determines if chunks should be batched together in horizontal_wrapped layout
pub struct InlineTextBatcher;

impl InlineTextBatcher {
    /// Check if a chunk should be batched with others (is inline text)
    pub fn is_inline_chunk(chunk: &TextChunk) -> bool {
        // Inline chunks are: text, not image, not code block, not table,
        // not ending with newline, and not starting with a list bullet
        if chunk.image_path.is_some()
            || chunk.code_block_lang.is_some()
            || chunk.table.is_some()
            || chunk.heading_level.is_some()
            || chunk.newline_after
        {
            return false;
        }

        // Check if starts with list bullet
        !Self::starts_with_bullet(&chunk.text)
    }

    /// Check if text starts with a Markdown list bullet
    fn starts_with_bullet(text: &str) -> bool {
        let trimmed = text.trim_start();
        trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("+ ")
    }

    /// Find the range of consecutive inline chunks starting at the given index
    /// Returns (start_idx, end_idx_exclusive)
    pub fn find_inline_batch(chunks: &[TextChunk], start_idx: usize) -> Option<(usize, usize)> {
        if start_idx >= chunks.len() {
            return None;
        }

        // Check if first chunk would be inline (ignoring newline_after for this check)
        let first_chunk = &chunks[start_idx];
        if first_chunk.image_path.is_some()
            || first_chunk.code_block_lang.is_some()
            || first_chunk.table.is_some()
            || first_chunk.heading_level.is_some()
            || Self::starts_with_bullet(&first_chunk.text)
        {
            return None;
        }

        // Find where the batch ends
        let mut end_idx = start_idx + 1;
        while end_idx < chunks.len() {
            let chunk = &chunks[end_idx];

            // Check if chunk is batchable (similar check but allow newline_after temporarily)
            if chunk.image_path.is_some()
                || chunk.code_block_lang.is_some()
                || chunk.table.is_some()
                || chunk.heading_level.is_some()
                || Self::starts_with_bullet(&chunk.text)
            {
                break;
            }

            end_idx += 1;

            // If this chunk ends with newline, include it and stop
            if chunk.newline_after {
                break;
            }
        }

        Some((start_idx, end_idx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_text_chunk(text: &str, newline_after: bool) -> TextChunk {
        TextChunk {
            text: text.to_string(),
            byte_range: 0..text.len(),
            line_start: 1,
            col_start: 1,
            line_end: 1,
            col_end: text.len(),
            bold: false,
            italic: false,
            code: false,
            heading_level: None,
            newline_after,
            image_path: None,
            alignment: None,
            image_width: None,
            image_height: None,
            code_block_lang: None,
            table: None,
            cached_height: None,
        }
    }

    #[test]
    fn test_is_inline_chunk_basic_text() {
        let chunk = create_text_chunk("Hello world", false);
        assert!(InlineTextBatcher::is_inline_chunk(&chunk));
    }

    #[test]
    fn test_is_inline_chunk_with_newline() {
        let chunk = create_text_chunk("Hello world", true);
        assert!(!InlineTextBatcher::is_inline_chunk(&chunk));
    }

    #[test]
    fn test_is_inline_chunk_with_bullet() {
        let chunk = create_text_chunk("- List item", false);
        assert!(!InlineTextBatcher::is_inline_chunk(&chunk));
    }

    #[test]
    fn test_is_inline_chunk_with_image() {
        let mut chunk = create_text_chunk("Alt text", false);
        chunk.image_path = Some("/path/to/image.png".to_string());
        assert!(!InlineTextBatcher::is_inline_chunk(&chunk));
    }

    #[test]
    fn test_is_inline_chunk_with_code_block() {
        let mut chunk = create_text_chunk("fn main() {}", false);
        chunk.code_block_lang = Some("rust".to_string());
        assert!(!InlineTextBatcher::is_inline_chunk(&chunk));
    }

    #[test]
    fn test_is_inline_chunk_with_heading() {
        let mut chunk = create_text_chunk("Heading", false);
        chunk.heading_level = Some(1);
        assert!(!InlineTextBatcher::is_inline_chunk(&chunk));
    }

    #[test]
    fn test_find_inline_batch_simple() {
        let chunks = vec![
            create_text_chunk("Hello ", false),
            create_text_chunk("world", true),
        ];

        let result = InlineTextBatcher::find_inline_batch(&chunks, 0);
        assert_eq!(result, Some((0, 2)));
    }

    #[test]
    fn test_find_inline_batch_stops_at_non_inline() {
        let chunks = vec![
            create_text_chunk("Hello ", false),
            create_text_chunk("world", false),
            {
                let mut chunk = create_text_chunk("Image", false);
                chunk.image_path = Some("img.png".to_string());
                chunk
            },
        ];

        let result = InlineTextBatcher::find_inline_batch(&chunks, 0);
        assert_eq!(result, Some((0, 2)));
    }

    #[test]
    fn test_find_inline_batch_none_for_non_inline() {
        let chunks = vec![{
            let mut chunk = create_text_chunk("Code", false);
            chunk.code_block_lang = Some("rust".to_string());
            chunk
        }];

        let result = InlineTextBatcher::find_inline_batch(&chunks, 0);
        assert_eq!(result, None);
    }

    #[test]
    fn test_starts_with_bullet_variations() {
        assert!(InlineTextBatcher::starts_with_bullet("- item"));
        assert!(InlineTextBatcher::starts_with_bullet("* item"));
        assert!(InlineTextBatcher::starts_with_bullet("+ item"));
        assert!(InlineTextBatcher::starts_with_bullet("  - item"));
        assert!(!InlineTextBatcher::starts_with_bullet("Not a list"));
        assert!(!InlineTextBatcher::starts_with_bullet("-no space"));
    }
}
