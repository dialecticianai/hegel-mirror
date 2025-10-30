/// Common test helpers
use mirror::image_manager::ImageManager;
use mirror::models::TextChunk;
use mirror::parsing::parse_markdown;
use std::path::Path;

/// Parse markdown with default test setup (ImageManager with "." base path)
pub fn parse_test_markdown(markdown: &str) -> Vec<TextChunk> {
    let mut image_manager = ImageManager::new(Path::new("."));
    parse_markdown(markdown, Path::new("."), &mut image_manager)
}
