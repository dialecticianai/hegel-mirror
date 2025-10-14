use crate::models::{Comment, LayoutMap, ReviewMode, Selection, TextChunk};
use crate::parsing::parse_markdown;
use crate::rendering::{render_comment_section, render_content};
use crate::storage::ReviewStorage;
use crate::syntax::SyntaxHighlighter;
use crate::theme::Theme;
use eframe::egui;
use std::collections::HashMap;
use std::path::PathBuf;

/// Extract text snippet from source for the given line range
fn extract_text_snippet(source: &str, start_line: usize, end_line: usize) -> String {
    let lines: Vec<&str> = source.lines().collect();
    let start_idx = start_line.saturating_sub(1); // Lines are 1-indexed
    let end_idx = end_line.min(lines.len());

    lines[start_idx..end_idx].join("\n")
}

pub struct MarkdownReviewApp {
    source: String,
    filename: String,
    base_path: PathBuf,
    chunks: Option<Vec<TextChunk>>,
    selection: Selection,
    comment_text: String,
    comments: Vec<Comment>,
    loaded_images: HashMap<String, egui::TextureHandle>,
    highlighter: SyntaxHighlighter,
    theme: Theme,
    layout_map: LayoutMap,
    review_mode: ReviewMode,
    storage: ReviewStorage,
}

impl MarkdownReviewApp {
    pub fn new(
        markdown: String,
        filename: String,
        base_path: PathBuf,
        out_dir: PathBuf,
        session_id: Option<String>,
    ) -> Self {
        let highlighter = SyntaxHighlighter::new();
        let storage = ReviewStorage::new(out_dir, filename.clone(), session_id);

        Self {
            source: markdown,
            filename,
            base_path,
            chunks: None, // Parse lazily on first frame
            selection: Selection::default(),
            comment_text: String::new(),
            comments: Vec::new(),
            loaded_images: HashMap::new(),
            highlighter,
            theme: Theme::default_theme(),
            layout_map: LayoutMap::new(),
            review_mode: ReviewMode::default(),
            storage,
        }
    }
}

impl eframe::App for MarkdownReviewApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Parse markdown on first frame (lazy initialization)
        if self.chunks.is_none() {
            self.chunks = Some(parse_markdown(&self.source, &self.base_path));
        }

        // Top menu bar for Submit Review button (batched mode)
        if self.review_mode == ReviewMode::Batched && !self.comments.is_empty() {
            egui::TopBottomPanel::top("review_actions").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("Review Mode");
                    ui.label(format!("({} comments queued)", self.comments.len()));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Submit Review").clicked() {
                            // Build comment tuples for batched write
                            let comment_data: Vec<_> = self
                                .comments
                                .iter()
                                .map(|c| {
                                    let text = extract_text_snippet(
                                        &self.source,
                                        c.line_start,
                                        c.line_end,
                                    );
                                    (
                                        text,
                                        c.text.clone(),
                                        c.line_start,
                                        c.col_start,
                                        c.line_end,
                                        c.col_end,
                                    )
                                })
                                .collect();

                            match self.storage.write_review(comment_data) {
                                Ok(path) => {
                                    println!("Review written to: {:?}", path);
                                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                }
                                Err(e) => {
                                    eprintln!("Failed to write review: {}", e);
                                }
                            }
                        }
                    });
                });
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // Apply page-level scroll area
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Apply page margins and max width
                let available_width = ui.available_width();
                let content_width = if let Some(max_width) = self.theme.layout.max_content_width {
                    max_width.min(
                        available_width
                            - self.theme.layout.page_margin_left
                            - self.theme.layout.page_margin_right,
                    )
                } else {
                    available_width
                        - self.theme.layout.page_margin_left
                        - self.theme.layout.page_margin_right
                };

                // Center content if max_width is set and we have room
                let left_margin = if self.theme.layout.max_content_width.is_some() {
                    ((available_width - content_width) / 2.0)
                        .max(self.theme.layout.page_margin_left)
                } else {
                    self.theme.layout.page_margin_left
                };

                ui.add_space(self.theme.layout.page_margin_top);

                ui.horizontal(|ui| {
                    ui.add_space(left_margin);
                    ui.vertical(|ui| {
                        ui.set_width(content_width);

                        // Show selection state in title
                        let title = if let (Some(start), Some(end)) =
                            (self.selection.start_line, self.selection.end_line)
                        {
                            let (min, max) = if start <= end {
                                (start, end)
                            } else {
                                (end, start)
                            };
                            format!(
                                "Markdown Review - Toy 2 (Bare Metal) | Selection: Lines {}-{}",
                                min, max
                            )
                        } else {
                            "Markdown Review - Toy 2 (Bare Metal)".to_string()
                        };
                        ui.heading(title);

                        // Clear layout map at start of frame
                        self.layout_map.clear();

                        // Only render if chunks are loaded
                        if let Some(chunks) = &mut self.chunks {
                            render_content(
                                ui,
                                ctx,
                                chunks,
                                &mut self.selection,
                                &mut self.loaded_images,
                                &self.highlighter,
                                &self.theme,
                                &mut self.layout_map,
                            );
                        } else {
                            ui.label("Loading...");
                        }
                    });
                });

                // Render comment UI as floating window (outside scroll area)
                render_comment_section(
                    ctx,
                    &self.layout_map,
                    &mut self.selection,
                    &mut self.comment_text,
                    &mut self.comments,
                    &self.theme,
                    &mut self.review_mode,
                    &self.storage,
                    &self.source,
                );

                ui.add_space(self.theme.layout.page_margin_bottom);
            });
        });
    }
}
