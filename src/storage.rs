use anyhow::{Context, Result};
use std::fs::{self, File};
use std::io::Write as IoWrite;
use std::path::{Path, PathBuf};

// Import reviews types from hegel-cli
pub use hegel::storage::reviews::{
    compute_relative_path, detect_project_type, detect_project_type_from, read_hegel_reviews,
    write_hegel_reviews, HegelReviewEntry, Position, ProjectType, ReviewComment, SelectionRange,
};

/// Storage manager for review files
pub struct ReviewStorage {
    out_dir: PathBuf,
    filename: String,
    pub session_id: Option<String>,
}

impl ReviewStorage {
    pub fn new(out_dir: PathBuf, filename: String, session_id: Option<String>) -> Self {
        Self {
            out_dir,
            filename,
            session_id,
        }
    }

    /// Get the base name without extension (e.g., "SPEC.md" -> "SPEC")
    fn base_name(&self) -> String {
        Path::new(&self.filename)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(&self.filename)
            .to_string()
    }

    /// Find the next review sequence number by scanning for existing .review.N files
    fn next_sequence_number(&self) -> Result<usize> {
        // Ensure output directory exists
        fs::create_dir_all(&self.out_dir).context(format!(
            "Failed to create output directory: {:?}",
            self.out_dir
        ))?;

        let base = self.base_name();
        let pattern = format!("{}.review.", base);

        let mut max_seq = 0;

        // Scan directory for matching files
        if let Ok(entries) = fs::read_dir(&self.out_dir) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.starts_with(&pattern) {
                        // Extract number after ".review."
                        if let Some(num_str) = name.strip_prefix(&pattern) {
                            if let Ok(num) = num_str.parse::<usize>() {
                                max_seq = max_seq.max(num);
                            }
                        }
                    }
                }
            }
        }

        Ok(max_seq + 1)
    }

    /// Get the current review file path (creates new sequence if none exists)
    pub fn review_file_path(&self) -> Result<PathBuf> {
        let seq = self.next_sequence_number()?;
        let base = self.base_name();
        Ok(self.out_dir.join(format!("{}.review.{}", base, seq)))
    }

    /// Write all comments atomically to a new review file (batched mode)
    pub fn write_review(
        &self,
        comments: Vec<(String, String, usize, usize, usize, usize)>,
    ) -> Result<PathBuf> {
        let review_path = self.review_file_path()?;
        let mut file = File::create(&review_path)
            .context(format!("Failed to create review file: {:?}", review_path))?;

        for (text, comment, line_start, col_start, line_end, col_end) in comments {
            let review_comment = ReviewComment::new(
                self.filename.clone(),
                self.session_id.clone(),
                text,
                comment,
                line_start,
                col_start,
                line_end,
                col_end,
            );

            let json_line = serde_json::to_string(&review_comment)
                .context("Failed to serialize comment to JSON")?;
            writeln!(file, "{}", json_line)
                .context(format!("Failed to write to review file: {:?}", review_path))?;
        }

        Ok(review_path)
    }

    /// Write approval (LGTM) to a new review file
    pub fn write_approval(&self) -> Result<PathBuf> {
        let review_path = self.review_file_path()?;
        let mut file = File::create(&review_path)
            .context(format!("Failed to create review file: {:?}", review_path))?;

        let timestamp = chrono::Utc::now().to_rfc3339();
        let message = if let Some(sid) = &self.session_id {
            format!("LGTM - {} (session: {})", timestamp, sid)
        } else {
            format!("LGTM - {}", timestamp)
        };

        writeln!(file, "{}", message)
            .context(format!("Failed to write to review file: {:?}", review_path))?;

        Ok(review_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_detect_hegel_project() {
        let temp_dir = TempDir::new().unwrap();
        let hegel_dir = temp_dir.path().join(".hegel");
        fs::create_dir(&hegel_dir).unwrap();

        let project_type = detect_project_type_from(Some(temp_dir.path().to_path_buf()));

        match project_type {
            ProjectType::Hegel { root } => {
                // Canonicalize both paths to handle macOS /var vs /private/var symlink
                let root_canonical = root.canonicalize().unwrap();
                let expected_canonical = hegel_dir.canonicalize().unwrap();
                assert_eq!(root_canonical, expected_canonical);
            }
            ProjectType::Standalone => {
                panic!("Expected Hegel project to be detected");
            }
        }
    }

    #[test]
    fn test_detect_standalone_project() {
        let temp_dir = TempDir::new().unwrap();

        let project_type = detect_project_type_from(Some(temp_dir.path().to_path_buf()));

        assert_eq!(project_type, ProjectType::Standalone);
    }

    #[test]
    fn test_hegel_project_stores_root_path() {
        let temp_dir = TempDir::new().unwrap();
        let hegel_dir = temp_dir.path().join(".hegel");
        fs::create_dir(&hegel_dir).unwrap();

        let project_type = detect_project_type_from(Some(temp_dir.path().to_path_buf()));

        if let ProjectType::Hegel { root } = project_type {
            assert!(root.ends_with(".hegel"));
            assert!(root.exists());
        } else {
            panic!("Expected Hegel project type");
        }
    }

    #[test]
    fn test_hegel_review_entry_serialization() {
        let comment = ReviewComment::new(
            "test.md".to_string(),
            Some("session123".to_string()),
            "selected text".to_string(),
            "test comment".to_string(),
            1,
            0,
            1,
            10,
        );

        let entry = HegelReviewEntry {
            comments: vec![comment],
            timestamp: "2025-01-01T00:00:00Z".to_string(),
            session_id: Some("session123".to_string()),
        };

        let json = serde_json::to_string(&entry).unwrap();
        let deserialized: HegelReviewEntry = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized, entry);
    }

    #[test]
    fn test_hegel_review_entry_empty_comments() {
        let entry = HegelReviewEntry {
            comments: vec![],
            timestamp: "2025-01-01T00:00:00Z".to_string(),
            session_id: Some("session123".to_string()),
        };

        let json = serde_json::to_string(&entry).unwrap();
        assert!(json.contains("\"comments\":[]"));
    }

    #[test]
    fn test_compute_relative_path() {
        let temp_dir = TempDir::new().unwrap();
        let hegel_dir = temp_dir.path().join(".hegel");
        fs::create_dir(&hegel_dir).unwrap();

        let file_path = temp_dir.path().join("src").join("test.md");
        fs::create_dir_all(file_path.parent().unwrap()).unwrap();
        fs::write(&file_path, "test").unwrap();

        let relative = compute_relative_path(&hegel_dir, &file_path).unwrap();
        assert_eq!(relative, "src/test.md");
    }

    #[test]
    fn test_read_hegel_reviews_empty() {
        let temp_dir = TempDir::new().unwrap();
        let hegel_dir = temp_dir.path().join(".hegel");
        fs::create_dir(&hegel_dir).unwrap();

        let reviews = read_hegel_reviews(&hegel_dir).unwrap();
        assert!(reviews.is_empty());
    }

    #[test]
    fn test_write_and_read_hegel_reviews() {
        let temp_dir = TempDir::new().unwrap();
        let hegel_dir = temp_dir.path().join(".hegel");

        let mut reviews = HashMap::new();
        let entry = HegelReviewEntry {
            comments: vec![],
            timestamp: "2025-01-01T00:00:00Z".to_string(),
            session_id: Some("session123".to_string()),
        };
        reviews.insert("test.md".to_string(), vec![entry.clone()]);

        write_hegel_reviews(&hegel_dir, &reviews).unwrap();

        let read_reviews = read_hegel_reviews(&hegel_dir).unwrap();
        assert_eq!(read_reviews.len(), 1);
        assert_eq!(read_reviews.get("test.md").unwrap()[0], entry);
    }

    #[test]
    fn test_write_hegel_reviews_creates_directory() {
        let temp_dir = TempDir::new().unwrap();
        let hegel_dir = temp_dir.path().join(".hegel");

        assert!(!hegel_dir.exists());

        let reviews = HashMap::new();
        write_hegel_reviews(&hegel_dir, &reviews).unwrap();

        assert!(hegel_dir.exists());
        assert!(hegel_dir.join("reviews.json").exists());
    }

    #[test]
    fn test_hegel_reviews_multiple_files() {
        let temp_dir = TempDir::new().unwrap();
        let hegel_dir = temp_dir.path().join(".hegel");

        let mut reviews = HashMap::new();

        let entry1 = HegelReviewEntry {
            comments: vec![],
            timestamp: "2025-01-01T00:00:00Z".to_string(),
            session_id: Some("session123".to_string()),
        };

        let entry2 = HegelReviewEntry {
            comments: vec![],
            timestamp: "2025-01-01T01:00:00Z".to_string(),
            session_id: Some("session123".to_string()),
        };

        reviews.insert("file1.md".to_string(), vec![entry1.clone()]);
        reviews.insert("file2.md".to_string(), vec![entry2.clone()]);

        write_hegel_reviews(&hegel_dir, &reviews).unwrap();

        let read_reviews = read_hegel_reviews(&hegel_dir).unwrap();
        assert_eq!(read_reviews.len(), 2);
        assert!(read_reviews.contains_key("file1.md"));
        assert!(read_reviews.contains_key("file2.md"));
    }

    #[test]
    fn test_hegel_reviews_multiple_entries_per_file() {
        let temp_dir = TempDir::new().unwrap();
        let hegel_dir = temp_dir.path().join(".hegel");

        let mut reviews = HashMap::new();

        let entry1 = HegelReviewEntry {
            comments: vec![],
            timestamp: "2025-01-01T00:00:00Z".to_string(),
            session_id: Some("session1".to_string()),
        };

        let entry2 = HegelReviewEntry {
            comments: vec![],
            timestamp: "2025-01-01T01:00:00Z".to_string(),
            session_id: Some("session2".to_string()),
        };

        reviews.insert("test.md".to_string(), vec![entry1.clone(), entry2.clone()]);

        write_hegel_reviews(&hegel_dir, &reviews).unwrap();

        let read_reviews = read_hegel_reviews(&hegel_dir).unwrap();
        let entries = read_reviews.get("test.md").unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0], entry1);
        assert_eq!(entries[1], entry2);
    }

    // Integration tests for dual-mode storage routing
    mod integration {
        use super::*;
        use crate::models::Document;

        #[test]
        fn test_document_write_review_hegel_mode() {
            let temp_dir = TempDir::new().unwrap();
            let hegel_dir = temp_dir.path().join(".hegel");
            fs::create_dir(&hegel_dir).unwrap();

            let file_path = temp_dir.path().join("test.md");
            fs::write(&file_path, "# Test").unwrap();

            let project_type = ProjectType::Hegel {
                root: hegel_dir.clone(),
            };

            let doc = Document::new(
                "test.md".to_string(),
                "# Test".to_string(),
                temp_dir.path().to_path_buf(),
                file_path.clone(),
                temp_dir.path().to_path_buf(),
                Some("session123".to_string()),
                project_type,
            );

            let comments = vec![
                ("text1".to_string(), "comment1".to_string(), 1, 0, 1, 5),
                ("text2".to_string(), "comment2".to_string(), 2, 0, 2, 5),
            ];

            let result = doc.write_review(comments);
            assert!(result.is_ok());

            // Verify reviews.json was created
            let reviews_json = hegel_dir.join("reviews.json");
            assert!(reviews_json.exists());

            // Verify content
            let reviews = read_hegel_reviews(&hegel_dir).unwrap();
            assert_eq!(reviews.len(), 1);
            assert!(reviews.contains_key("test.md"));

            let entries = reviews.get("test.md").unwrap();
            assert_eq!(entries.len(), 1);
            assert_eq!(entries[0].comments.len(), 2);
        }

        #[test]
        fn test_document_write_approval_hegel_mode() {
            let temp_dir = TempDir::new().unwrap();
            let hegel_dir = temp_dir.path().join(".hegel");
            fs::create_dir(&hegel_dir).unwrap();

            let file_path = temp_dir.path().join("test.md");
            fs::write(&file_path, "# Test").unwrap();

            let project_type = ProjectType::Hegel {
                root: hegel_dir.clone(),
            };

            let doc = Document::new(
                "test.md".to_string(),
                "# Test".to_string(),
                temp_dir.path().to_path_buf(),
                file_path.clone(),
                temp_dir.path().to_path_buf(),
                Some("session123".to_string()),
                project_type,
            );

            let result = doc.write_approval();
            assert!(result.is_ok());

            // Verify reviews.json was created
            let reviews_json = hegel_dir.join("reviews.json");
            assert!(reviews_json.exists());

            // Verify LGTM entry has empty comments
            let reviews = read_hegel_reviews(&hegel_dir).unwrap();
            assert_eq!(reviews.len(), 1);

            let entries = reviews.get("test.md").unwrap();
            assert_eq!(entries.len(), 1);
            assert_eq!(entries[0].comments.len(), 0);
        }

        #[test]
        fn test_document_write_review_standalone_mode() {
            let temp_dir = TempDir::new().unwrap();
            let file_path = temp_dir.path().join("test.md");
            fs::write(&file_path, "# Test").unwrap();

            let project_type = ProjectType::Standalone;

            let doc = Document::new(
                "test.md".to_string(),
                "# Test".to_string(),
                temp_dir.path().to_path_buf(),
                file_path.clone(),
                temp_dir.path().to_path_buf(),
                Some("session123".to_string()),
                project_type,
            );

            let comments = vec![("text1".to_string(), "comment1".to_string(), 1, 0, 1, 5)];

            let result = doc.write_review(comments);
            assert!(result.is_ok());

            // Verify .review.1 file was created
            let review_file = temp_dir.path().join("test.review.1");
            assert!(review_file.exists());

            // Verify no reviews.json was created
            assert!(!temp_dir.path().join("reviews.json").exists());
        }

        #[test]
        fn test_multi_file_hegel_reviews() {
            let temp_dir = TempDir::new().unwrap();
            let hegel_dir = temp_dir.path().join(".hegel");
            fs::create_dir(&hegel_dir).unwrap();

            let project_type = ProjectType::Hegel {
                root: hegel_dir.clone(),
            };

            // Create two documents
            let file1 = temp_dir.path().join("file1.md");
            let file2 = temp_dir.path().join("file2.md");
            fs::write(&file1, "# File 1").unwrap();
            fs::write(&file2, "# File 2").unwrap();

            let doc1 = Document::new(
                "file1.md".to_string(),
                "# File 1".to_string(),
                temp_dir.path().to_path_buf(),
                file1.clone(),
                temp_dir.path().to_path_buf(),
                Some("session123".to_string()),
                project_type.clone(),
            );

            let doc2 = Document::new(
                "file2.md".to_string(),
                "# File 2".to_string(),
                temp_dir.path().to_path_buf(),
                file2.clone(),
                temp_dir.path().to_path_buf(),
                Some("session123".to_string()),
                project_type.clone(),
            );

            // Write reviews for both
            doc1.write_review(vec![(
                "text1".to_string(),
                "comment1".to_string(),
                1,
                0,
                1,
                5,
            )])
            .unwrap();
            doc2.write_review(vec![(
                "text2".to_string(),
                "comment2".to_string(),
                1,
                0,
                1,
                5,
            )])
            .unwrap();

            // Verify both are in reviews.json
            let reviews = read_hegel_reviews(&hegel_dir).unwrap();
            assert_eq!(reviews.len(), 2);
            assert!(reviews.contains_key("file1.md"));
            assert!(reviews.contains_key("file2.md"));
        }
    }
}
