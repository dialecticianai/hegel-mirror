use anyhow::{Context, Result};
use serde::Serialize;
use std::fs::{self, File};
use std::io::Write as IoWrite;
use std::path::{Path, PathBuf};

/// Review comment with full metadata for JSONL serialization
#[derive(Serialize, Clone)]
pub struct ReviewComment {
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    pub file: String,
    pub selection: SelectionRange,
    pub text: String,
    pub comment: String,
}

#[derive(Serialize, Clone)]
pub struct SelectionRange {
    pub start: Position,
    pub end: Position,
}

#[derive(Serialize, Clone)]
pub struct Position {
    pub line: usize,
    pub col: usize,
}

impl ReviewComment {
    pub fn new(
        file: String,
        session_id: Option<String>,
        text: String,
        comment: String,
        line_start: usize,
        col_start: usize,
        line_end: usize,
        col_end: usize,
    ) -> Self {
        Self {
            timestamp: chrono::Utc::now().to_rfc3339(),
            session_id,
            file,
            selection: SelectionRange {
                start: Position {
                    line: line_start,
                    col: col_start,
                },
                end: Position {
                    line: line_end,
                    col: col_end,
                },
            },
            text,
            comment,
        }
    }
}

/// Storage manager for review files
pub struct ReviewStorage {
    out_dir: PathBuf,
    filename: String,
    session_id: Option<String>,
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
