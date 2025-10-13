use anyhow::Result;
use clap::Parser;

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

    // MVP placeholder: Just print args and exit successfully
    println!("Mirror v{}", env!("CARGO_PKG_VERSION"));
    println!("Files: {:?}", args.files);
    println!("Output directory: {}", args.out_dir);
    println!("JSON output: {}", args.json);
    println!("Headless: {}", args.headless);
    println!("\n⚠️  MVP not yet implemented - this is a placeholder");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        // Placeholder test to ensure test infrastructure works
        assert_eq!(2 + 2, 4);
    }
}
