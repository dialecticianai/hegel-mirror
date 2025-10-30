mod app;
mod image_manager;
mod models;
mod parsing;
mod rendering;
mod storage;
mod syntax;
mod theme;

use anyhow::Result;
use app::MarkdownReviewApp;
use clap::Parser;
use eframe::egui;
use models::Document;
use std::fs;
use std::path::Path;
use std::sync::Arc;

/// Ephemeral Markdown review UI for Dialectic-Driven Development
#[derive(Parser, Debug)]
#[command(name = "mirror")]
#[command(version, about, long_about = None)]
struct Args {
    /// Files to review
    files: Vec<String>,

    /// Output directory for review files
    #[arg(long, default_value = ".ddd")]
    out_dir: String,

    /// Emit JSON with review file paths on exit
    #[arg(long)]
    json: bool,

    /// Headless mode (no-op, for testing)
    #[arg(long)]
    headless: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.files.is_empty() {
        anyhow::bail!("No files specified. Usage: mirror FILE1.md [FILE2.md ...]");
    }

    if args.headless {
        // Headless mode: just exit successfully
        return Ok(());
    }

    // Get session ID from environment
    let session_id = std::env::var("HEGEL_SESSION_ID").ok();

    // Parse out_dir as PathBuf
    let out_dir = Path::new(&args.out_dir).to_path_buf();

    // Load all files into Document structs
    let mut documents = Vec::new();
    for file_path in &args.files {
        let markdown_content = fs::read_to_string(file_path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", file_path, e));

        let base_path = Path::new(file_path)
            .parent()
            .unwrap_or(Path::new("."))
            .to_path_buf();

        let filename = Path::new(file_path)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown.md")
            .to_string();

        documents.push(Document::new(
            filename,
            markdown_content,
            base_path,
            out_dir.clone(),
            session_id.clone(),
        ));
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Hegel Mirror",
        options,
        Box::new(move |cc| {
            // Set light mode visuals
            cc.egui_ctx.set_visuals(egui::Visuals::light());

            // Install image loaders for emoji support
            egui_extras::install_image_loaders(&cc.egui_ctx);

            // Load custom fonts (Inter with bold/italic variants)
            let mut fonts = egui::FontDefinitions::default();

            // Load font data from embedded files
            fonts.font_data.insert(
                "Inter-Regular".to_owned(),
                Arc::new(egui::FontData::from_static(include_bytes!(
                    "../fonts/Inter-Regular.ttf"
                ))),
            );
            fonts.font_data.insert(
                "Inter-Bold".to_owned(),
                Arc::new(egui::FontData::from_static(include_bytes!(
                    "../fonts/Inter-Bold.ttf"
                ))),
            );
            fonts.font_data.insert(
                "Inter-Italic".to_owned(),
                Arc::new(egui::FontData::from_static(include_bytes!(
                    "../fonts/Inter-Italic.ttf"
                ))),
            );
            fonts.font_data.insert(
                "Inter-BoldItalic".to_owned(),
                Arc::new(egui::FontData::from_static(include_bytes!(
                    "../fonts/Inter-BoldItalic.ttf"
                ))),
            );

            // Set up font families
            fonts.families.insert(
                egui::FontFamily::Proportional,
                vec!["Inter-Regular".to_owned()],
            );
            fonts.families.insert(
                egui::FontFamily::Name("Bold".into()),
                vec!["Inter-Bold".to_owned()],
            );
            fonts.families.insert(
                egui::FontFamily::Name("Italic".into()),
                vec!["Inter-Italic".to_owned()],
            );
            fonts.families.insert(
                egui::FontFamily::Name("BoldItalic".into()),
                vec!["Inter-BoldItalic".to_owned()],
            );

            // Apply font definitions
            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(MarkdownReviewApp::new(documents)))
        }),
    )
    .map_err(|e| anyhow::anyhow!("eframe error: {}", e))
}
