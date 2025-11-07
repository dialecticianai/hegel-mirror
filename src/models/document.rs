use crate::image_manager::ImageManager;
use crate::models::{Comment, LayoutMap, Selection, TextChunk};
use crate::storage::{ProjectType, ReviewStorage};
use std::path::PathBuf;

/// A single document being reviewed
pub struct Document {
    pub filename: String,
    pub source: String,
    pub base_path: PathBuf,
    pub file_path: PathBuf,
    pub chunks: Option<Vec<TextChunk>>,
    pub selection: Selection,
    pub comment_text: String,
    pub comments: Vec<Comment>,
    pub image_manager: ImageManager,
    pub layout_map: LayoutMap,
    pub storage: ReviewStorage,
    pub project_type: ProjectType,
    pub approved: bool,
}

impl Document {
    pub fn new(
        filename: String,
        source: String,
        base_path: PathBuf,
        file_path: PathBuf,
        out_dir: PathBuf,
        session_id: Option<String>,
        project_type: ProjectType,
    ) -> Self {
        let storage = ReviewStorage::new(out_dir, filename.clone(), session_id);
        let image_manager = ImageManager::new(&base_path);

        Self {
            filename,
            source,
            base_path,
            file_path,
            chunks: None, // Parse lazily on first render
            selection: Selection::default(),
            comment_text: String::new(),
            comments: Vec::new(),
            image_manager,
            layout_map: LayoutMap::new(),
            storage,
            project_type,
            approved: false,
        }
    }

    /// Count of comments for this document (for tab label)
    pub fn comment_count(&self) -> usize {
        self.comments.len()
    }

    /// Write review comments (routes to appropriate backend)
    pub fn write_review(
        &self,
        comments: Vec<(String, String, usize, usize, usize, usize)>,
    ) -> anyhow::Result<PathBuf> {
        use crate::storage::{
            compute_relative_path, read_hegel_reviews, write_hegel_reviews, HegelReviewEntry,
            ReviewComment,
        };

        match &self.project_type {
            ProjectType::Hegel { root } => {
                // Compute relative path for this file
                let relative_path = compute_relative_path(root, &self.file_path)?;

                // Read existing reviews
                let mut reviews_map = read_hegel_reviews(root)?;

                // Create review comments
                let review_comments: Vec<ReviewComment> = comments
                    .into_iter()
                    .map(
                        |(text, comment, line_start, col_start, line_end, col_end)| {
                            ReviewComment::new(
                                relative_path.clone(),
                                self.storage.session_id.clone(),
                                text,
                                comment,
                                line_start,
                                col_start,
                                line_end,
                                col_end,
                            )
                        },
                    )
                    .collect();

                // Create new entry
                let entry = HegelReviewEntry {
                    comments: review_comments,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                    session_id: self.storage.session_id.clone(),
                };

                // Append to file's review list
                reviews_map
                    .entry(relative_path.clone())
                    .or_insert_with(Vec::new)
                    .push(entry);

                // Write back to disk
                write_hegel_reviews(root, &reviews_map)?;

                // Return path to reviews.json
                Ok(root.join("reviews.json"))
            }
            ProjectType::Standalone => {
                // Use existing ReviewStorage logic
                self.storage.write_review(comments)
            }
        }
    }

    /// Write approval (LGTM) (routes to appropriate backend)
    pub fn write_approval(&self) -> anyhow::Result<PathBuf> {
        use crate::storage::{
            compute_relative_path, read_hegel_reviews, write_hegel_reviews, HegelReviewEntry,
        };

        match &self.project_type {
            ProjectType::Hegel { root } => {
                // Compute relative path for this file
                let relative_path = compute_relative_path(root, &self.file_path)?;

                // Read existing reviews
                let mut reviews_map = read_hegel_reviews(root)?;

                // Create LGTM entry with empty comments
                let entry = HegelReviewEntry {
                    comments: vec![],
                    timestamp: chrono::Utc::now().to_rfc3339(),
                    session_id: self.storage.session_id.clone(),
                };

                // Append to file's review list
                reviews_map
                    .entry(relative_path.clone())
                    .or_insert_with(Vec::new)
                    .push(entry);

                // Write back to disk
                write_hegel_reviews(root, &reviews_map)?;

                // Return path to reviews.json
                Ok(root.join("reviews.json"))
            }
            ProjectType::Standalone => {
                // Use existing ReviewStorage logic
                self.storage.write_approval()
            }
        }
    }
}
