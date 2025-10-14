use crate::models::{Alignment, Table, TextChunk};
use crate::parsing::position::LineOffsets;
use std::ops::Range;
use std::path::Path;

/// Push a text chunk to the chunks vector
pub fn push_text_chunk(
    chunks: &mut Vec<TextChunk>,
    text: String,
    source: &str,
    line_offsets: &LineOffsets,
    range: &Range<usize>,
    bold: bool,
    italic: bool,
    in_code_block: bool,
    heading_level: Option<u8>,
    code_block_lang: &Option<String>,
) {
    let (line_start, col_start) = line_offsets.byte_to_line_col(source, range.start);
    let (line_end, col_end) = line_offsets.byte_to_line_col(source, range.end);

    chunks.push(TextChunk {
        text,
        byte_range: range.clone(),
        line_start,
        col_start,
        line_end,
        col_end,
        bold,
        italic,
        code: in_code_block,
        heading_level,
        newline_after: false,
        image_path: None,
        alignment: None,
        image_width: None,
        code_block_lang: if in_code_block {
            code_block_lang.clone()
        } else {
            None
        },
        table: None,
        cached_height: None,
    });
}

/// Push an inline code chunk to the chunks vector
pub fn push_code_chunk(
    chunks: &mut Vec<TextChunk>,
    text: String,
    source: &str,
    line_offsets: &LineOffsets,
    range: &Range<usize>,
    bold: bool,
    italic: bool,
    heading_level: Option<u8>,
) {
    let (line_start, col_start) = line_offsets.byte_to_line_col(source, range.start);
    let (line_end, col_end) = line_offsets.byte_to_line_col(source, range.end);

    chunks.push(TextChunk {
        text,
        byte_range: range.clone(),
        line_start,
        col_start,
        line_end,
        col_end,
        bold,
        italic,
        code: true,
        heading_level,
        newline_after: false,
        image_path: None,
        alignment: None,
        image_width: None,
        code_block_lang: None,
        table: None,
        cached_height: None,
    });
}

/// Push a break chunk (soft or hard break) to the chunks vector
pub fn push_break_chunk(
    chunks: &mut Vec<TextChunk>,
    text: String,
    range: &Range<usize>,
    newline: bool,
) {
    chunks.push(TextChunk {
        text,
        byte_range: range.clone(),
        line_start: 0,
        col_start: 0,
        line_end: 0,
        col_end: 0,
        bold: false,
        italic: false,
        code: false,
        heading_level: None,
        newline_after: newline,
        image_path: None,
        alignment: None,
        image_width: None,
        code_block_lang: None,
        table: None,
        cached_height: None,
    });
}

/// Push an image chunk to the chunks vector
pub fn push_image_chunk(
    chunks: &mut Vec<TextChunk>,
    url: &str,
    source: &str,
    base_path: &Path,
    line_offsets: &LineOffsets,
    range: &Range<usize>,
) {
    push_image_chunk_with_alignment(
        chunks,
        url,
        source,
        base_path,
        line_offsets,
        range,
        None,
        None,
    );
}

/// Push an image chunk with specific alignment and width
pub fn push_image_chunk_with_alignment(
    chunks: &mut Vec<TextChunk>,
    url: &str,
    source: &str,
    base_path: &Path,
    line_offsets: &LineOffsets,
    range: &Range<usize>,
    alignment: Option<Alignment>,
    width: Option<f32>,
) {
    let (line_start, col_start) = line_offsets.byte_to_line_col(source, range.start);
    let (line_end, col_end) = line_offsets.byte_to_line_col(source, range.end);
    let image_path = base_path.join(url).to_string_lossy().to_string();

    chunks.push(TextChunk {
        text: format!("[Image: {}]", url),
        byte_range: range.clone(),
        line_start,
        col_start,
        line_end,
        col_end,
        bold: false,
        italic: false,
        code: false,
        heading_level: None,
        newline_after: true,
        image_path: Some(image_path),
        alignment,
        image_width: width,
        code_block_lang: None,
        table: None,
        cached_height: None,
    });
}

/// Push a table chunk to the chunks vector
pub fn push_table_chunk(
    chunks: &mut Vec<TextChunk>,
    table: Table,
    source: &str,
    line_offsets: &LineOffsets,
    range: &Range<usize>,
) {
    let (line_start, col_start) = line_offsets.byte_to_line_col(source, range.start);
    let (line_end, col_end) = line_offsets.byte_to_line_col(source, range.end);

    chunks.push(TextChunk {
        text: "[Table]".to_string(),
        byte_range: range.clone(),
        line_start,
        col_start,
        line_end,
        col_end,
        bold: false,
        italic: false,
        code: false,
        heading_level: None,
        newline_after: true,
        image_path: None,
        alignment: None,
        image_width: None,
        code_block_lang: None,
        table: Some(table),
        cached_height: None,
    });
}
