// Library exports for testing

pub mod models;
pub mod parsing;
pub mod rendering;
pub mod syntax;
pub mod theme;

// Re-export commonly used items for convenience
pub use models::{Comment, LayoutMap, Selection, TextChunk};
pub use parsing::parse_markdown;
