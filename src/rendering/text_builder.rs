use crate::theme::Theme;
use eframe::egui;
use egui_twemoji::EmojiLabel;

/// Context for text rendering, determines which theme settings to apply
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextContext {
    /// Regular body text
    Body,
    /// Heading text with level (1-6)
    Heading(u8),
    /// Text inside table cells
    TableCell { is_header: bool },
    /// Inline code (currently unused, code styling handled by boolean flag)
    #[allow(dead_code)]
    InlineCode,
}

/// Build a styled RichText with consistent styling across all rendering contexts
///
/// This is the single source of truth for text styling in the application.
/// All text rendering (paragraphs, tables, headings) should use this function
/// to ensure consistent bold/italic/emoji support.
pub fn build_styled_text(
    text: &str,
    bold: bool,
    italic: bool,
    code: bool,
    context: TextContext,
    theme: &Theme,
) -> egui::RichText {
    let mut rich = egui::RichText::new(text);

    // Apply font family based on bold/italic combination
    // We now have actual bold fonts loaded, so use proper font families
    if bold && italic {
        rich = rich.family(egui::FontFamily::Name("BoldItalic".into()));
    } else if bold {
        rich = rich.family(egui::FontFamily::Name("Bold".into()));
    } else if italic {
        rich = rich.family(egui::FontFamily::Name("Italic".into()));
    }
    // Regular font (Proportional) is default, no need to set explicitly

    // Apply context-specific styling
    match context {
        TextContext::Body => {
            rich = rich
                .size(theme.typography.body_size)
                .color(theme.colors.text);
        }
        TextContext::Heading(level) => {
            let size = theme.typography.heading_sizes[(level - 1).min(5) as usize];
            rich = rich.heading().size(size).color(theme.colors.heading);
        }
        TextContext::TableCell { is_header } => {
            let text_color = if is_header {
                theme.colors.heading
            } else {
                theme.colors.text
            };
            rich = rich.size(theme.typography.body_size).color(text_color);
        }
        TextContext::InlineCode => {
            // Currently unused - inline code styling is handled by the `code` boolean below
            rich = rich
                .size(theme.typography.body_size)
                .color(theme.colors.text);
        }
    }

    // Apply inline code styling (overrides context sizing)
    if code {
        rich = rich
            .code()
            .size(theme.typography.code_size)
            .color(theme.colors.inline_code);
    }

    rich
}

/// Render styled text with emoji support
///
/// This wraps build_styled_text() and uses EmojiLabel to render colored emojis.
/// Use this instead of ui.label(build_styled_text(...)) for automatic emoji support.
pub fn render_styled_text(
    ui: &mut egui::Ui,
    text: &str,
    bold: bool,
    italic: bool,
    code: bool,
    context: TextContext,
    theme: &Theme,
    _id_source: impl std::hash::Hash,
) -> egui::Response {
    let styled_text = build_styled_text(text, bold, italic, code, context, theme);

    // Use EmojiLabel which automatically renders emojis as colored images
    // For non-emoji text, it behaves like a regular label
    // Important: Keep auto_inline enabled so emojis flow inline with text
    // Without this, EmojiLabel creates its own layout which breaks horizontal flow
    // Note: Widget ID collisions are handled by the caller using ui.interact() with a unique ID
    EmojiLabel::new(styled_text).auto_inline(true).show(ui)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_styled_text_body() {
        let theme = Theme::default_theme();
        let rich = build_styled_text("Hello", false, false, false, TextContext::Body, &theme);
        // Can't easily test RichText internals, but this ensures it compiles and runs
        assert_eq!(rich.text(), "Hello");
    }

    #[test]
    fn test_build_styled_text_heading() {
        let theme = Theme::default_theme();
        let rich = build_styled_text(
            "Heading",
            true,
            false,
            false,
            TextContext::Heading(1),
            &theme,
        );
        assert_eq!(rich.text(), "Heading");
    }

    #[test]
    fn test_build_styled_text_table() {
        let theme = Theme::default_theme();
        let rich = build_styled_text(
            "Cell",
            false,
            false,
            false,
            TextContext::TableCell { is_header: true },
            &theme,
        );
        assert_eq!(rich.text(), "Cell");
    }

    #[test]
    fn test_build_styled_text_inline_code() {
        let theme = Theme::default_theme();
        let rich = build_styled_text("code", false, false, true, TextContext::Body, &theme);
        assert_eq!(rich.text(), "code");
    }
}
