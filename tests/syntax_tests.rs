/// Unit tests for syntax highlighting
use mirror::syntax::SyntaxHighlighter;

#[test]
fn test_syntax_highlighter_new() {
    let highlighter = SyntaxHighlighter::new();

    // Should have loaded syntax and theme sets
    assert!(highlighter.syntax_set.syntaxes().len() > 0);
    assert!(highlighter.theme_set.themes.len() > 0);
}

#[test]
fn test_get_syntax_rust() {
    let highlighter = SyntaxHighlighter::new();
    let syntax = highlighter.get_syntax("rust");

    assert_eq!(syntax.name, "Rust");
}

#[test]
fn test_get_syntax_python() {
    let highlighter = SyntaxHighlighter::new();
    let syntax = highlighter.get_syntax("python");

    assert_eq!(syntax.name, "Python");
}

#[test]
fn test_get_syntax_javascript() {
    let highlighter = SyntaxHighlighter::new();
    let syntax = highlighter.get_syntax("javascript");

    assert_eq!(syntax.name, "JavaScript");
}

#[test]
fn test_get_syntax_unknown_falls_back_to_plain_text() {
    let highlighter = SyntaxHighlighter::new();
    let syntax = highlighter.get_syntax("nonexistent-language-xyz");

    assert_eq!(syntax.name, "Plain Text");
}

#[test]
fn test_get_theme() {
    let highlighter = SyntaxHighlighter::new();
    let theme = highlighter.get_theme();

    // Should load base16-ocean.dark theme
    assert!(theme.name.is_some());
}

#[test]
fn test_highlight_code_rust() {
    let highlighter = SyntaxHighlighter::new();
    let code = "fn main() {\n    println!(\"Hello\");\n}";

    let highlighted = highlighter.highlight_code(code, "rust");

    // Should have 3 lines (each line separately highlighted)
    assert_eq!(highlighted.len(), 3);

    // Each line should have style+text tuples
    for line in &highlighted {
        assert!(line.len() > 0, "Each line should have styled segments");
    }
}

#[test]
fn test_highlight_code_python() {
    let highlighter = SyntaxHighlighter::new();
    let code = "def hello():\n    print('world')";

    let highlighted = highlighter.highlight_code(code, "python");

    assert_eq!(highlighted.len(), 2);
    assert!(highlighted[0].len() > 0);
}

#[test]
fn test_highlight_code_empty_string() {
    let highlighter = SyntaxHighlighter::new();
    let code = "";

    let highlighted = highlighter.highlight_code(code, "rust");

    // Empty string has 0 lines
    assert_eq!(highlighted.len(), 0);
}

#[test]
fn test_highlight_code_single_line() {
    let highlighter = SyntaxHighlighter::new();
    let code = "let x = 5;";

    let highlighted = highlighter.highlight_code(code, "rust");

    assert_eq!(highlighted.len(), 1);

    // Should have multiple style segments for a statement
    let line = &highlighted[0];
    assert!(line.len() >= 1);

    // Concatenating all text should equal original (but with potential newline)
    let reconstructed: String = line.iter().map(|(_, text)| text.as_str()).collect();
    assert!(reconstructed.starts_with("let x = 5;"));
}

#[test]
fn test_highlight_code_with_whitespace() {
    let highlighter = SyntaxHighlighter::new();
    let code = "  fn test() {}";

    let highlighted = highlighter.highlight_code(code, "rust");

    assert_eq!(highlighted.len(), 1);
    assert!(highlighted[0].len() > 0);
}

#[test]
fn test_highlight_code_multiline_preserves_structure() {
    let highlighter = SyntaxHighlighter::new();
    let code = "line1\nline2\nline3";

    let highlighted = highlighter.highlight_code(code, "plain");

    // Should preserve 3 lines
    assert_eq!(highlighted.len(), 3);
}

#[test]
fn test_highlight_code_unknown_language() {
    let highlighter = SyntaxHighlighter::new();
    let code = "some random text";

    // Should not panic with unknown language, falls back to plain text
    let highlighted = highlighter.highlight_code(code, "unknown-lang");

    assert_eq!(highlighted.len(), 1);
}

#[test]
fn test_default_trait() {
    let highlighter = SyntaxHighlighter::default();

    // Default should work same as new()
    assert!(highlighter.syntax_set.syntaxes().len() > 0);
}

#[test]
fn test_highlight_code_with_unicode() {
    let highlighter = SyntaxHighlighter::new();
    let code = "// Comment with emoji 🦀\nlet x = \"日本語\";";

    let highlighted = highlighter.highlight_code(code, "rust");

    // Should handle Unicode without panicking
    assert_eq!(highlighted.len(), 2);
}

#[test]
fn test_highlight_preserves_content() {
    let highlighter = SyntaxHighlighter::new();
    let code = "fn test() { return 42; }";

    let highlighted = highlighter.highlight_code(code, "rust");

    // Reconstruct text from highlighted output
    let reconstructed: String = highlighted
        .iter()
        .flat_map(|line| line.iter().map(|(_, text)| text.as_str()))
        .collect();

    // Should preserve all original characters (might have added newline)
    assert!(reconstructed.contains("fn test"));
    assert!(reconstructed.contains("return 42"));
}
