use crate::models::{Comment, LayoutMap, Selection, TextChunk};
use crate::parsing::parse_markdown;
use crate::rendering::{render_comment_section, render_content};
use crate::syntax::SyntaxHighlighter;
use crate::theme::Theme;
use eframe::egui;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct MarkdownReviewApp {
    _source: String,
    chunks: Vec<TextChunk>,
    selection: Selection,
    comment_text: String,
    comments: Vec<Comment>,
    _base_path: PathBuf,
    loaded_images: HashMap<String, egui::TextureHandle>,
    highlighter: SyntaxHighlighter,
    theme: Theme,
    layout_map: LayoutMap,
}

impl MarkdownReviewApp {
    pub fn new(markdown: String, base_path: PathBuf) -> Self {
        let chunks = parse_markdown(&markdown, &base_path);
        let highlighter = SyntaxHighlighter::new();

        Self {
            _source: markdown,
            chunks,
            selection: Selection::default(),
            comment_text: String::new(),
            comments: Vec::new(),
            _base_path: base_path,
            loaded_images: HashMap::new(),
            highlighter,
            theme: Theme::default_theme(),
            layout_map: LayoutMap::new(),
        }
    }
}

impl eframe::App for MarkdownReviewApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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

                        render_content(
                            ui,
                            ctx,
                            &mut self.chunks,
                            &mut self.selection,
                            &mut self.loaded_images,
                            &self.highlighter,
                            &self.theme,
                            &mut self.layout_map,
                        );
                    });
                });

                // Render comment UI as floating window (outside scroll area)
                render_comment_section(
                    ctx,
                    &self.layout_map,
                    &self.selection,
                    &mut self.comment_text,
                    &mut self.comments,
                    &self.theme,
                );

                ui.add_space(self.theme.layout.page_margin_bottom);
            });
        });
    }
}
