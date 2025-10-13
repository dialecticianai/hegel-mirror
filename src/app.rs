use crate::models::{Comment, Selection, TextChunk};
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
        }
    }
}

impl eframe::App for MarkdownReviewApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Markdown Review - Toy 2 (Bare Metal)");

            render_content(
                ui,
                ctx,
                &mut self.chunks,
                &mut self.selection,
                &mut self.loaded_images,
                &self.highlighter,
                &self.theme,
            );

            render_comment_section(
                ui,
                &self.chunks,
                &self.selection,
                &mut self.comment_text,
                &mut self.comments,
                &self.theme,
            );
        });
    }
}
