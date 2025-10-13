use crate::models::{Table, TextChunk};
use crate::parsing::position::byte_to_line_col;
use pulldown_cmark::{CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd};
use std::ops::Range;
use std::path::Path;

/// Parse markdown into chunks with position tracking
pub fn parse_markdown(source: &str, base_path: &Path) -> Vec<TextChunk> {
    let mut chunks: Vec<TextChunk> = Vec::new();
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(source, options);

    // Track style state
    let mut bold = false;
    let mut italic = false;
    let mut heading_level = None;
    let mut current_image_url: Option<CowStr> = None;
    let mut in_code_block = false;
    let mut code_block_lang: Option<String> = None;

    // Track table state
    let mut in_table = false;
    let mut in_table_head = false;
    let mut current_table: Option<Table> = None;
    let mut current_row: Vec<String> = Vec::new();
    let mut current_cell: String = String::new();
    let mut table_start_range: Option<Range<usize>> = None;

    for (event, range) in parser.into_offset_iter() {
        match event {
            Event::Start(Tag::Table(alignments)) => {
                in_table = true;
                current_table = Some(Table::new(alignments));
                table_start_range = Some(range.clone());
            }
            Event::Start(Tag::TableHead) => {
                in_table_head = true;
            }
            Event::Start(Tag::TableRow) => {
                current_row.clear();
            }
            Event::Start(Tag::TableCell) => {
                current_cell.clear();
            }
            Event::End(TagEnd::TableCell) => {
                current_row.push(current_cell.clone());
            }
            Event::End(TagEnd::TableRow) => {
                if let Some(table) = &mut current_table {
                    if in_table_head {
                        table.header = current_row.clone();
                    } else {
                        table.rows.push(current_row.clone());
                    }
                }
            }
            Event::End(TagEnd::TableHead) => {
                in_table_head = false;
            }
            Event::End(TagEnd::Table) => {
                if let Some(table) = current_table.take() {
                    let table_range = table_start_range.take().unwrap_or(range.clone());
                    push_table_chunk(&mut chunks, table, source, &table_range);
                }
                in_table = false;
            }
            Event::Text(text) => {
                if in_table {
                    current_cell.push_str(&text);
                } else if current_image_url.is_none() {
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
                if in_table {
                    current_cell.push_str(&text);
                } else {
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
            }
            Event::SoftBreak => {
                if !in_table {
                    push_break_chunk(&mut chunks, " ".to_string(), &range, false);
                }
            }
            Event::HardBreak => {
                if !in_table {
                    push_break_chunk(&mut chunks, "\n".to_string(), &range, true);
                }
            }
            Event::Start(tag) => {
                if !in_table {
                    handle_start_tag(
                        tag,
                        &mut bold,
                        &mut italic,
                        &mut heading_level,
                        &mut current_image_url,
                        &mut in_code_block,
                        &mut code_block_lang,
                    );
                }
            }
            Event::End(tag) => {
                if !in_table {
                    handle_end_tag(
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
                    );
                }
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
        table: None,
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
        table: None,
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
        table: None,
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
        table: None,
        cached_height: None,
    });
}

fn push_table_chunk(chunks: &mut Vec<TextChunk>, table: Table, source: &str, range: &Range<usize>) {
    let (line_start, col_start) = byte_to_line_col(source, range.start);
    let (line_end, col_end) = byte_to_line_col(source, range.end);

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
        code_block_lang: None,
        table: Some(table),
        cached_height: None,
    });
}
