use crate::models::{Table, TextChunk};
use crate::parsing::chunks::{
    push_break_chunk, push_code_chunk, push_image_chunk, push_image_chunk_with_alignment,
    push_table_chunk, push_text_chunk,
};
use crate::parsing::html::parse_html_image;
use crate::parsing::position::LineOffsets;
use pulldown_cmark::{CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd};
use std::ops::Range;
use std::path::Path;

/// Parse markdown into chunks with position tracking
pub fn parse_markdown(source: &str, base_path: &Path) -> Vec<TextChunk> {
    let mut chunks: Vec<TextChunk> = Vec::new();

    // Build line offset table once for O(log n) lookups
    let line_offsets = LineOffsets::new(source);

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

    // Track HTML block state (for centered images)
    let mut in_html_block = false;
    let mut html_block_content = String::new();
    let mut html_block_range: Option<Range<usize>> = None;

    for (event, range) in parser.into_offset_iter() {
        match event {
            Event::Start(Tag::HtmlBlock) => {
                in_html_block = true;
                html_block_content.clear();
                html_block_range = Some(range.clone());
            }
            Event::Html(html) | Event::InlineHtml(html) if in_html_block => {
                html_block_content.push_str(&html);
            }
            Event::End(TagEnd::HtmlBlock) => {
                in_html_block = false;
                // Try to parse centered image from HTML
                if let Some((img_src, alignment, width)) = parse_html_image(&html_block_content) {
                    let html_range = html_block_range.take().unwrap_or(range.clone());
                    push_image_chunk_with_alignment(
                        &mut chunks,
                        &img_src,
                        source,
                        base_path,
                        &line_offsets,
                        &html_range,
                        alignment,
                        width,
                    );
                }
                html_block_content.clear();
            }
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
                    push_table_chunk(&mut chunks, table, source, &line_offsets, &table_range);
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
                        &line_offsets,
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
                        &line_offsets,
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
                        &line_offsets,
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
    line_offsets: &LineOffsets,
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
                push_image_chunk(
                    chunks,
                    url.as_ref(),
                    source,
                    base_path,
                    &line_offsets,
                    range,
                );
            }
        }
        _ => {}
    }
}

// HTML image parsing extracted to parsing/html.rs
