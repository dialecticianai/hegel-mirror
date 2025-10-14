/// Unit tests for parsing layer
use mirror::parsing::parse_markdown;
use std::path::Path;

#[test]
fn test_parse_empty_string() {
    let markdown = "";
    let chunks = parse_markdown(markdown, Path::new("."));
    assert_eq!(chunks.len(), 0);
}

#[test]
fn test_parse_plain_text() {
    let markdown = "Hello world";
    let chunks = parse_markdown(markdown, Path::new("."));

    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0].text, "Hello world");
    assert_eq!(chunks[0].line_start, 1);
    assert!(!chunks[0].bold);
    assert!(!chunks[0].italic);
    assert!(!chunks[0].code);
}

#[test]
fn test_parse_heading() {
    let markdown = "# Heading 1";
    let chunks = parse_markdown(markdown, Path::new("."));

    assert!(!chunks.is_empty());
    let heading_chunk = chunks.iter().find(|c| c.text == "Heading 1").unwrap();
    assert_eq!(heading_chunk.heading_level, Some(1));
    assert_eq!(heading_chunk.line_start, 1);
}

#[test]
fn test_parse_bold_text() {
    let markdown = "This is **bold** text";
    let chunks = parse_markdown(markdown, Path::new("."));

    let bold_chunk = chunks.iter().find(|c| c.text == "bold").unwrap();
    assert!(bold_chunk.bold);
    assert!(!bold_chunk.italic);
}

#[test]
fn test_parse_italic_text() {
    let markdown = "This is *italic* text";
    let chunks = parse_markdown(markdown, Path::new("."));

    let italic_chunk = chunks.iter().find(|c| c.text == "italic").unwrap();
    assert!(italic_chunk.italic);
    assert!(!italic_chunk.bold);
}

#[test]
fn test_parse_inline_code() {
    let markdown = "Use `println!()` to print";
    let chunks = parse_markdown(markdown, Path::new("."));

    let code_chunk = chunks.iter().find(|c| c.text == "println!()").unwrap();
    assert!(code_chunk.code);
    assert_eq!(code_chunk.code_block_lang, None);
}

#[test]
fn test_parse_code_block() {
    let markdown = r#"```rust
fn main() {}
```"#;
    let chunks = parse_markdown(markdown, Path::new("."));

    let code_chunk = chunks.iter().find(|c| c.text.contains("fn main")).unwrap();
    assert!(code_chunk.code);
    assert_eq!(code_chunk.code_block_lang, Some("rust".to_string()));
}

#[test]
fn test_parse_code_block_no_language() {
    let markdown = r#"```
plain code
```"#;
    let chunks = parse_markdown(markdown, Path::new("."));

    let code_chunk = chunks
        .iter()
        .find(|c| c.text.contains("plain code"))
        .unwrap();
    assert!(code_chunk.code);
    // Fenced block without language has empty string, not None
    assert_eq!(code_chunk.code_block_lang, Some("".to_string()));
}

#[test]
fn test_parse_multiline_document() {
    let markdown = include_str!("fixtures/basic.md");
    let chunks = parse_markdown(markdown, Path::new("."));

    // Should have multiple chunks
    assert!(chunks.len() > 5);

    // Check heading exists
    let heading = chunks.iter().find(|c| c.text == "Heading 1").unwrap();
    assert_eq!(heading.heading_level, Some(1));
    assert_eq!(heading.line_start, 1);

    // Check bold text exists
    let bold = chunks.iter().find(|c| c.text == "bold").unwrap();
    assert!(bold.bold);

    // Check italic text exists
    let italic = chunks.iter().find(|c| c.text == "italic").unwrap();
    assert!(italic.italic);

    // Check inline code exists
    let inline_code = chunks.iter().find(|c| c.text == "inline code").unwrap();
    assert!(inline_code.code);

    // Check code block exists
    let code_block = chunks
        .iter()
        .find(|c| c.code && c.text.contains("println"))
        .unwrap();
    assert_eq!(code_block.code_block_lang, Some("rust".to_string()));
}

#[test]
fn test_parse_table() {
    let markdown = include_str!("fixtures/tables.md");
    let chunks = parse_markdown(markdown, Path::new("."));

    // Should find table chunks
    let table_chunks: Vec<_> = chunks.iter().filter(|c| c.table.is_some()).collect();
    assert!(table_chunks.len() > 0, "Should parse at least one table");

    let first_table = table_chunks[0].table.as_ref().unwrap();
    // Tables should have alignments (columns)
    assert!(
        first_table.alignments.len() > 0,
        "Table should have columns"
    );
}

#[test]
fn test_parse_unicode() {
    let markdown = include_str!("fixtures/unicode.md");
    let chunks = parse_markdown(markdown, Path::new("."));

    // Should parse without panicking
    assert!(chunks.len() > 0);

    // Check that emoji is parsed
    let emoji_chunk = chunks.iter().find(|c| c.text.contains("🦀"));
    assert!(emoji_chunk.is_some(), "Should parse emoji");

    // Check multi-byte characters
    let japanese_chunk = chunks.iter().find(|c| c.text.contains("日本語"));
    assert!(japanese_chunk.is_some(), "Should parse Japanese characters");
}

#[test]
fn test_position_tracking_single_line() {
    let markdown = "Hello world";
    let chunks = parse_markdown(markdown, Path::new("."));

    assert_eq!(chunks[0].line_start, 1);
    assert_eq!(chunks[0].line_end, 1);
    assert_eq!(chunks[0].col_start, 1);
}

#[test]
fn test_position_tracking_multiple_lines() {
    let markdown = "Line 1\nLine 2\nLine 3";
    let chunks = parse_markdown(markdown, Path::new("."));

    // Find chunks on different lines
    let line1 = chunks.iter().find(|c| c.line_start == 1);
    let line2 = chunks.iter().find(|c| c.line_start == 2);
    let line3 = chunks.iter().find(|c| c.line_start == 3);

    assert!(line1.is_some(), "Should have chunk on line 1");
    assert!(line2.is_some(), "Should have chunk on line 2");
    assert!(line3.is_some(), "Should have chunk on line 3");
}

#[test]
fn test_newline_after_paragraph() {
    let markdown = "Paragraph 1\n\nParagraph 2";
    let chunks = parse_markdown(markdown, Path::new("."));

    // Check that paragraph chunks have newline_after set
    let para_chunks: Vec<_> = chunks.iter().filter(|c| !c.text.is_empty()).collect();
    assert!(para_chunks.len() >= 2);

    // First paragraph should have newline after
    let first_para = &para_chunks[0];
    assert!(first_para.newline_after || para_chunks[1].newline_after);
}

#[test]
fn test_soft_break_vs_hard_break() {
    let markdown_soft = "Line 1\nLine 2";
    let markdown_hard = "Line 1  \nLine 2"; // Two spaces before newline = hard break

    let chunks_soft = parse_markdown(markdown_soft, Path::new("."));
    let chunks_hard = parse_markdown(markdown_hard, Path::new("."));

    // Both should parse successfully
    assert!(chunks_soft.len() > 0);
    assert!(chunks_hard.len() > 0);
}

#[test]
fn test_image_path_resolution() {
    let markdown = "![Alt text](image.png)";
    let base_path = Path::new("/test/dir");
    let chunks = parse_markdown(markdown, base_path);

    let image_chunk = chunks.iter().find(|c| c.image_path.is_some());
    assert!(image_chunk.is_some(), "Should parse image");

    let img = image_chunk.unwrap();
    assert!(img.image_path.as_ref().unwrap().contains("image.png"));
}

#[test]
fn test_html_centered_image() {
    let markdown = r#"<p align="center">
<img src="test.png" width="400">
</p>"#;
    let chunks = parse_markdown(markdown, Path::new("."));

    let image_chunk = chunks.iter().find(|c| c.image_path.is_some());
    assert!(image_chunk.is_some(), "Should parse HTML image");

    let img = image_chunk.unwrap();
    assert!(img.alignment.is_some(), "Should detect center alignment");
    assert_eq!(img.image_width, Some(400.0), "Should parse width");
}

#[test]
fn test_mixed_formatting() {
    let markdown = "This has ***bold and italic*** text";
    let chunks = parse_markdown(markdown, Path::new("."));

    // Should parse the bold+italic chunk
    let formatted = chunks.iter().find(|c| c.bold && c.italic);
    assert!(formatted.is_some(), "Should parse bold+italic text");
}
