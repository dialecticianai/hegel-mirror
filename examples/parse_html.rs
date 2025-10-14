use pulldown_cmark::{Event, Options, Parser};

fn main() {
    let markdown = r#"<p align="center">
  <img src="logo.png" alt="Test Logo" width="200">
</p>

# Test Document

This is a test.
"#;

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(markdown, options);

    for (i, event) in parser.enumerate() {
        println!("{}: {:?}", i, event);
    }
}
