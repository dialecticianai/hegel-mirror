use std::ops::Range;

/// A rendered chunk of text with its source position
#[derive(Clone, Debug)]
pub struct TextChunk {
    pub text: String,
    /// Byte range in source markdown
    pub byte_range: Range<usize>,
    /// Line/col range in source markdown (1-indexed)
    pub line_start: usize,
    pub col_start: usize,
    pub line_end: usize,
    pub col_end: usize,
    /// Styling
    pub bold: bool,
    pub italic: bool,
    pub code: bool,
    pub heading_level: Option<u8>,
    /// Layout hints
    pub newline_after: bool,
    /// Image path (if this is an image)
    pub image_path: Option<String>,
    /// Code block language (if this is a code block)
    pub code_block_lang: Option<String>,
    /// Cached render height (for lazy loading without flicker)
    pub cached_height: Option<f32>,
}

impl TextChunk {
    pub fn is_image(&self) -> bool {
        self.image_path.is_some()
    }

    pub fn is_code_block(&self) -> bool {
        self.code_block_lang.is_some()
    }
}
