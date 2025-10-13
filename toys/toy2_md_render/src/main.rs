mod app;
mod models;
mod parsing;
mod rendering;
mod syntax;

use app::MarkdownReviewApp;
use eframe::egui;
use std::fs;
use std::path::Path;

fn main() -> Result<(), eframe::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <markdown-file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let markdown_content = fs::read_to_string(file_path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", file_path, e));

    let base_path = Path::new(file_path)
        .parent()
        .unwrap_or(Path::new("."))
        .to_path_buf();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Hegel Mirror - Toy 2",
        options,
        Box::new(move |cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            Ok(Box::new(MarkdownReviewApp::new(
                markdown_content,
                base_path,
            )))
        }),
    )
}
