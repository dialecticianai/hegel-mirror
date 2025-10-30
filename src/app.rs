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

        // Tab bar (if multiple documents, showing only non-approved docs)
        let unapproved_docs: Vec<usize> = self
            .documents
            .iter()
            .enumerate()
            .filter(|(_, doc)| !doc.approved)
            .map(|(i, _)| i)
            .collect();

        if unapproved_docs.len() > 1 {
            egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    for &i in &unapproved_docs {
                        let doc = &self.documents[i];
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

        // Top menu bar for LGTM and Submit Review buttons (per-document)
        let active_has_comments = self.documents[self.active_document_index].comment_count() > 0;
        let active_approved = self.documents[self.active_document_index].approved;
        let show_top_bar =
            self.review_mode == ReviewMode::Batched && active_has_comments || !active_has_comments;

        if show_top_bar && !active_approved {
            egui::TopBottomPanel::top("review_actions").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if self.review_mode == ReviewMode::Batched && active_has_comments {
                        ui.heading("Review Mode");
                        ui.label(format!(
                            "({} comments queued)",
                            self.documents[self.active_document_index].comment_count()
                        ));
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // LGTM button - only show if current document has no comments
                        if !active_has_comments {
                            if ui.button("Approve (LGTM)").clicked() {
                                // Write approval for current document
                                match self.documents[self.active_document_index]
                                    .storage
                                    .write_approval()
                                {
                                    Ok(path) => {
                                        println!("Approval written to: {:?}", path);
                                        // Mark document as approved
                                        let doc = &mut self.documents[self.active_document_index];
                                        doc.approved = true;

                                        // Check if all documents are done (all approved)
                                        let all_done = self.documents.iter().all(|d| d.approved);
                                        if all_done {
                                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                        } else {
                                            // Switch to next unapproved document
                                            if let Some(&next_idx) = unapproved_docs
                                                .iter()
                                                .find(|&&i| i != self.active_document_index)
                                            {
                                                self.active_document_index = next_idx;
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to write approval: {}", e);
                                    }
                                }
                            }
                        }

                        // Submit Review button - only show in batched mode with comments
                        if self.review_mode == ReviewMode::Batched && active_has_comments {
                            if ui.button("Submit Review").clicked() {
                                // Build comment tuples for current document
                                let doc = &self.documents[self.active_document_index];
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

                                match doc.storage.write_review(comment_data.clone()) {
                                    Ok(path) => {
                                        println!("Review written to: {:?}", path);
                                        println!();
                                        // Print the full review content
                                        for (
                                            text,
                                            comment,
                                            line_start,
                                            _col_start,
                                            line_end,
                                            _col_end,
                                        ) in &comment_data
                                        {
                                            println!("Lines {}-{}:", line_start, line_end);
                                            println!(
                                                "  Selected: {}",
                                                text.lines().next().unwrap_or("")
                                            );
                                            if text.lines().count() > 1 {
                                                println!("  ...");
                                            }
                                            println!("  Comment: {}", comment);
                                            println!();
                                        }
                                        // Mark document as approved (review submitted)
                                        let doc = &mut self.documents[self.active_document_index];
                                        doc.approved = true;

                                        // Check if all documents are done
                                        let all_done = self.documents.iter().all(|d| d.approved);
                                        if all_done {
                                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                        } else {
                                            // Switch to next unapproved document
                                            if let Some(&next_idx) = unapproved_docs
                                                .iter()
                                                .find(|&&i| i != self.active_document_index)
                                            {
                                                self.active_document_index = next_idx;
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to write review: {}", e);
                                    }
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
