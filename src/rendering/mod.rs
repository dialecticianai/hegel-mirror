mod chunk;
pub(crate) mod chunk_renderer;
pub mod code;
pub mod comments;
pub(crate) mod helpers;
pub mod image;
pub(crate) mod inline_batcher;
pub(crate) mod selection_manager;
pub mod table;
pub mod text;
pub(crate) mod text_builder;
pub mod ui;
pub(crate) mod viewport;

pub use comments::render_comment_section;
pub use ui::render_content;
