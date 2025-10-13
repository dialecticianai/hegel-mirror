pub mod default;

use eframe::egui;

/// Complete theme specification for the application
#[derive(Clone, Debug)]
pub struct Theme {
    pub typography: Typography,
    pub spacing: Spacing,
    pub colors: Colors,
}

/// Typography settings (font sizes, line heights, etc.)
#[derive(Clone, Debug)]
pub struct Typography {
    /// Base body text size
    pub body_size: f32,
    /// Heading sizes [h1, h2, h3, h4, h5, h6]
    pub heading_sizes: [f32; 6],
    /// Inline code size
    pub code_size: f32,
    /// Code block size
    pub code_block_size: f32,
    /// Line height multiplier
    pub line_height: f32,
}

/// Spacing and layout settings
#[derive(Clone, Debug)]
pub struct Spacing {
    /// Space after paragraphs
    pub paragraph: f32,
    /// Space after headings
    pub heading: f32,
    /// Padding inside code blocks
    pub code_block_padding: f32,
    /// Inner margin for frames
    pub inner_margin: f32,
    /// Corner radius for rounded elements
    pub corner_radius: f32,
    /// Minimum line height (in pixels)
    pub min_line_height: f32,
}

/// Color scheme
#[derive(Clone, Debug)]
pub struct Colors {
    /// Background color for code blocks
    pub code_block_bg: egui::Color32,
    /// Text color for body text
    pub text: egui::Color32,
    /// Text color for headings
    pub heading: egui::Color32,
    /// Text color for inline code
    pub inline_code: egui::Color32,
}

impl Theme {
    /// Get the default theme
    pub fn default_theme() -> Self {
        default::THEME
    }
}
