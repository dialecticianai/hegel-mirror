use crate::models::{Document, ReviewMode};
use crate::parsing::parse_markdown;
use crate::rendering::{render_comment_section, render_content};
use crate::syntax::SyntaxHighlighter;
use crate::theme::Theme;
use eframe::egui;

/// Extract text snippet from source for the given line range
fn extract_text_snippet(source: &str, start_line: usize, end_line: usize) -> String {
    let lines: Vec<&str> = source.lines().collect();
    let start_idx = start_line.saturating_sub(1); // Lines are 1-indexed
    let end_idx = end_line.min(lines.len());

    lines[start_idx..end_idx].join("\n")
}

pub struct MarkdownReviewApp {
    documents: Vec<Document>,
    active_document_index: usize,
    highlighter: SyntaxHighlighter,
    theme: Theme,
    review_mode: ReviewMode,
}

impl MarkdownReviewApp {
    pub fn new(documents: Vec<Document>) -> Self {
        let highlighter = SyntaxHighlighter::new();

        Self {
            documents,
            active_document_index: 0,
            highlighter,
            theme: Theme::default_theme(),
            review_mode: ReviewMode::default(),
        }
    }

    fn active_document(&mut self) -> &mut Document {
        &mut self.documents[self.active_document_index]
    }
}

impl eframe::App for MarkdownReviewApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Parse markdown on first frame for active document (lazy initialization)
        {
            let doc = self.active_document();
            if doc.chunks.is_none() {
                doc.chunks = Some(parse_markdown(&doc.source, &doc.base_path));
            }
        }

        // Tab bar (if multiple documents)
        if self.documents.len() > 1 {
            egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    for (i, doc) in self.documents.iter().enumerate() {
                        let label = if doc.comment_count() > 0 {
                            format!("{} ({})", doc.filename, doc.comment_count())
                        } else {
                            doc.filename.clone()
                        };

                        if ui
                            .selectable_label(i == self.active_document_index, label)
                            .clicked()
                        {
                            self.active_document_index = i;
                        }
                    }
                });
            });
        }

        // Top menu bar for Submit Review button (batched mode)
        let total_comments: usize = self.documents.iter().map(|d| d.comment_count()).sum();
        if self.review_mode == ReviewMode::Batched && total_comments > 0 {
            egui::TopBottomPanel::top("review_actions").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("Review Mode");
                    ui.label(format!("({} comments queued)", total_comments));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Submit Review").clicked() {
                            // Write reviews for all documents with comments
                            let mut all_successful = true;
                            for doc in &self.documents {
                                if doc.comments.is_empty() {
                                    continue;
                                }

                                // Build comment tuples for batched write
                                let comment_data: Vec<_> = doc
                                    .comments
                                    .iter()
                                    .map(|c| {
                                        let text = extract_text_snippet(
                                            &doc.source,
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

                                match doc.storage.write_review(comment_data) {
                                    Ok(path) => {
                                        println!("Review written to: {:?}", path);
                                    }
                                    Err(e) => {
                                        eprintln!(
                                            "Failed to write review for {}: {}",
                                            doc.filename, e
                                        );
                                        all_successful = false;
                                    }
                                }
                            }

                            if all_successful {
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
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

                // Split borrows - need to get immutable refs before mutable borrow
                let highlighter = &self.highlighter;
                let theme = &self.theme;
                let doc = &mut self.documents[self.active_document_index];

                ui.horizontal(|ui| {
                    ui.add_space(left_margin);
                    ui.vertical(|ui| {
                        ui.set_width(content_width);

                        // Show selection state in title
                        let title = if let (Some(start), Some(end)) =
                            (doc.selection.start_line, doc.selection.end_line)
                        {
                            let (min, max) = if start <= end {
                                (start, end)
                            } else {
                                (end, start)
                            };
                            format!("{} | Selection: Lines {}-{}", doc.filename, min, max)
                        } else {
                            doc.filename.clone()
                        };
                        ui.heading(title);

                        // Clear layout map at start of frame
                        doc.layout_map.clear();

                        // Only render if chunks are loaded
                        if let Some(chunks) = &mut doc.chunks {
                            render_content(
                                ui,
                                ctx,
                                chunks,
                                &mut doc.selection,
                                &mut doc.loaded_images,
                                highlighter,
                                theme,
                                &mut doc.layout_map,
                            );
                        } else {
                            ui.label("Loading...");
                        }
                    });
                });

                // Render comment UI as floating window (outside scroll area)
                render_comment_section(
                    ctx,
                    &doc.layout_map,
                    &mut doc.selection,
                    &mut doc.comment_text,
                    &mut doc.comments,
                    theme,
                    &mut self.review_mode,
                    &doc.storage,
                    &doc.source,
                );

                ui.add_space(self.theme.layout.page_margin_bottom);
            });
        });
    }
}
