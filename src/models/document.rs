use crate::image_manager::ImageManager;
use crate::models::{Comment, LayoutMap, Selection, TextChunk};
use crate::storage::ReviewStorage;
use std::path::PathBuf;

/// A single document being reviewed
pub struct Document {
    pub filename: String,
    pub source: String,
    pub base_path: PathBuf,
    pub chunks: Option<Vec<TextChunk>>,
    pub selection: Selection,
    pub comment_text: String,
    pub comments: Vec<Comment>,
    pub image_manager: ImageManager,
    pub layout_map: LayoutMap,
    pub storage: ReviewStorage,
    pub approved: bool,
}

impl Document {
    pub fn new(
        filename: String,
        source: String,
        base_path: PathBuf,
        out_dir: PathBuf,
        session_id: Option<String>,
    ) -> Self {
        let storage = ReviewStorage::new(out_dir, filename.clone(), session_id);
        let image_manager = ImageManager::new(&base_path);

        Self {
            filename,
            source,
            base_path,
            chunks: None, // Parse lazily on first render
            selection: Selection::default(),
            comment_text: String::new(),
            comments: Vec::new(),
            image_manager,
            layout_map: LayoutMap::new(),
            storage,
            approved: false,
        }
    }

    /// Count of comments for this document (for tab label)
    pub fn comment_count(&self) -> usize {
        self.comments.len()
    }
}
