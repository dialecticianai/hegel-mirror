use super::Table;
use std::ops::Range;

/// Alignment for images and other block elements
#[derive(Clone, Debug, PartialEq)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

/// A rendered chunk of text with its source position
#[derive(Clone, Debug)]
pub struct TextChunk {
    pub text: String,
    /// Byte range in source markdown (for column precision feature)
    #[allow(dead_code)]
    pub byte_range: Range<usize>,
    /// Line/col range in source markdown (1-indexed)
    pub line_start: usize,
    /// Column start (for column precision feature)
    #[allow(dead_code)]
    pub col_start: usize,
    pub line_end: usize,
    /// Column end (for column precision feature)
    #[allow(dead_code)]
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
    /// Image alignment (for images)
    pub alignment: Option<Alignment>,
    /// Image width constraint (for images)
    pub image_width: Option<f32>,
    /// Actual image height (loaded during parsing)
    pub image_height: Option<f32>,
    /// Code block language (if this is a code block)
    pub code_block_lang: Option<String>,
    /// Table data (if this is a table)
    pub table: Option<Table>,
    /// Cached render height (for lazy loading without flicker)
    pub cached_height: Option<f32>,
}
