use super::{Colors, Spacing, Theme, Typography};
use eframe::egui;

/// Default theme - clean, readable, professional
pub const THEME: Theme = Theme {
    typography: Typography {
        body_size: 14.0,
        heading_sizes: [32.0, 28.0, 24.0, 20.0, 16.0, 14.0],
        code_size: 13.0,
        code_block_size: 13.0,
        line_height: 1.5,
    },
    spacing: Spacing {
        paragraph: 4.0,
        heading: 8.0,
        code_block_padding: 10.0,
        inner_margin: 10.0,
        corner_radius: 4.0,
        min_line_height: 16.0,
    },
    colors: Colors {
        // Dark code block background (like GitHub)
        code_block_bg: egui::Color32::from_rgb(43, 48, 59),
        text: egui::Color32::from_rgb(50, 50, 50),
        heading: egui::Color32::from_rgb(30, 30, 30),
        inline_code: egui::Color32::from_rgb(214, 73, 108),
    },
};
