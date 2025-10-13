use crate::models::TextChunk;
use crate::parsing::position::byte_to_line_col;
use pulldown_cmark::{CodeBlockKind, CowStr, Event, Parser, Tag, TagEnd};
use std::ops::Range;
use std::path::Path;

/// Parse markdown into chunks with position tracking
pub fn parse_markdown(source: &str, base_path: &Path) -> Vec<TextChunk> {
    let mut chunks: Vec<TextChunk> = Vec::new();
    let parser = Parser::new(source);

    // Track style state
    let mut bold = false;
    let mut italic = false;
    let mut heading_level = None;
    let mut current_image_url: Option<CowStr> = None;
    let mut in_code_block = false;
    let mut code_block_lang: Option<String> = None;

    for (event, range) in parser.into_offset_iter() {
        match event {
            Event::Start(tag) => handle_start_tag(
                tag,
                &mut bold,
                &mut italic,
                &mut heading_level,
                &mut current_image_url,
                &mut in_code_block,
                &mut code_block_lang,
            ),
            Event::End(tag) => handle_end_tag(
                tag,
                &mut bold,
                &mut italic,
                &mut heading_level,
                &mut current_image_url,
                &mut in_code_block,
                &mut code_block_lang,
                &mut chunks,
                source,
                base_path,
                &range,
            ),
            Event::Text(text) => {
                if current_image_url.is_none() {
                    push_text_chunk(
                        &mut chunks,
                        text.to_string(),
                        source,
                        &range,
                        bold,
                        italic,
                        in_code_block,
                        heading_level,
                        &code_block_lang,
                    );
                }
            }
            Event::Code(text) => {
                push_code_chunk(
                    &mut chunks,
                    text.to_string(),
                    source,
                    &range,
                    bold,
                    italic,
                    heading_level,
                );
            }
            Event::SoftBreak => {
                push_break_chunk(&mut chunks, " ".to_string(), &range, false);
            }
            Event::HardBreak => {
                push_break_chunk(&mut chunks, "\n".to_string(), &range, true);
            }
            _ => {}
        }
    }

    chunks
}

fn handle_start_tag<'a>(
    tag: Tag<'a>,
    bold: &mut bool,
    italic: &mut bool,
    heading_level: &mut Option<u8>,
    current_image_url: &mut Option<CowStr<'a>>,
    in_code_block: &mut bool,
    code_block_lang: &mut Option<String>,
) {
    match tag {
        Tag::Strong => *bold = true,
        Tag::Emphasis => *italic = true,
        Tag::CodeBlock(kind) => {
            *in_code_block = true;
            *code_block_lang = match kind {
                CodeBlockKind::Fenced(lang) => Some(lang.to_string()),
                CodeBlockKind::Indented => None,
            };
        }
        Tag::Heading { level, .. } => *heading_level = Some(level as u8),
        Tag::Image { dest_url, .. } => {
            *current_image_url = Some(dest_url);
        }
        _ => {}
    }
}

fn handle_end_tag(
    tag: TagEnd,
    bold: &mut bool,
    italic: &mut bool,
    heading_level: &mut Option<u8>,
    current_image_url: &mut Option<CowStr>,
    in_code_block: &mut bool,
    code_block_lang: &mut Option<String>,
    chunks: &mut Vec<TextChunk>,
    source: &str,
    base_path: &Path,
    range: &Range<usize>,
) {
    match tag {
        TagEnd::Strong => *bold = false,
        TagEnd::Emphasis => *italic = false,
        TagEnd::CodeBlock => {
            *in_code_block = false;
            *code_block_lang = None;
        }
        TagEnd::Heading(_) => {
            *heading_level = None;
            if let Some(last) = chunks.last_mut() {
                last.newline_after = true;
            }
        }
        TagEnd::Paragraph => {
            if let Some(last) = chunks.last_mut() {
                last.newline_after = true;
            }
        }
        TagEnd::Image => {
            if let Some(url) = current_image_url.take() {
                push_image_chunk(chunks, url.as_ref(), source, base_path, range);
            }
        }
        _ => {}
    }
}

fn push_text_chunk(
    chunks: &mut Vec<TextChunk>,
    text: String,
    source: &str,
    range: &Range<usize>,
    bold: bool,
    italic: bool,
    in_code_block: bool,
    heading_level: Option<u8>,
    code_block_lang: &Option<String>,
) {
    let (line_start, col_start) = byte_to_line_col(source, range.start);
    let (line_end, col_end) = byte_to_line_col(source, range.end);

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
        code_block_lang: if in_code_block {
            code_block_lang.clone()
        } else {
            None
        },
        cached_height: None,
    });
}

fn push_code_chunk(
    chunks: &mut Vec<TextChunk>,
    text: String,
    source: &str,
    range: &Range<usize>,
    bold: bool,
    italic: bool,
    heading_level: Option<u8>,
) {
    let (line_start, col_start) = byte_to_line_col(source, range.start);
    let (line_end, col_end) = byte_to_line_col(source, range.end);

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
        code_block_lang: None,
        cached_height: None,
    });
}

fn push_break_chunk(
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
        code_block_lang: None,
        cached_height: None,
    });
}

fn push_image_chunk(
    chunks: &mut Vec<TextChunk>,
    url: &str,
    source: &str,
    base_path: &Path,
    range: &Range<usize>,
) {
    let (line_start, col_start) = byte_to_line_col(source, range.start);
    let (line_end, col_end) = byte_to_line_col(source, range.end);
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
        code_block_lang: None,
        cached_height: None,
    });
}
