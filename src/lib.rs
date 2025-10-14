// Library exports for testing

pub mod models;
pub mod parsing;
pub mod rendering;
pub mod storage;
pub mod syntax;
pub mod theme;

// Re-export commonly used items for convenience
pub use models::{Comment, Document, LayoutMap, ReviewMode, Selection, TextChunk};
pub use parsing::parse_markdown;
pub use storage::ReviewStorage;
