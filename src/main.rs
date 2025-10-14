mod app;
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
use std::fs;
use std::path::Path;

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

    // For now, only support single file (multi-file tabs coming later)
    let file_path = &args.files[0];
    let markdown_content = fs::read_to_string(file_path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", file_path, e));

    let base_path = Path::new(file_path)
        .parent()
        .unwrap_or(Path::new("."))
        .to_path_buf();

    // Extract filename for review file naming
    let filename = Path::new(file_path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown.md")
        .to_string();

    // Get session ID from environment
    let session_id = std::env::var("HEGEL_SESSION_ID").ok();

    // Parse out_dir as PathBuf
    let out_dir = Path::new(&args.out_dir).to_path_buf();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Hegel Mirror",
        options,
        Box::new(move |cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            Ok(Box::new(MarkdownReviewApp::new(
                markdown_content,
                filename,
                base_path,
                out_dir,
                session_id,
            )))
        }),
    )
    .map_err(|e| anyhow::anyhow!("eframe error: {}", e))
}
