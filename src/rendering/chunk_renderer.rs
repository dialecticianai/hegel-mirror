/// Trait-based chunk rendering system to eliminate duplication
use crate::models::{LayoutMap, Selection, TextChunk};
use crate::rendering::selection_manager::SelectionManager;
use crate::rendering::viewport::ViewportCuller;
use crate::syntax::SyntaxHighlighter;
use crate::theme::Theme;
use eframe::egui;
use std::collections::HashMap;

/// Shared context for rendering chunks
pub struct RenderContext<'a> {
    pub ui: &'a mut egui::Ui,
    pub ctx: &'a egui::Context,
    pub chunk: &'a mut TextChunk,
    pub idx: usize,
    pub selection: &'a mut Selection,
    pub loaded_images: &'a mut HashMap<String, egui::TextureHandle>,
    pub highlighter: &'a SyntaxHighlighter,
    pub theme: &'a Theme,
    pub layout_map: &'a mut LayoutMap,
    pub need_layout_map: bool,
}

/// Trait for rendering different chunk types with consistent viewport culling
pub trait ChunkRenderer {
    /// Estimate the height of this chunk for viewport culling
    fn estimate_height(&self, chunk: &TextChunk, theme: &Theme) -> f32;

    /// Render the chunk when it's visible in the viewport
    /// Returns (actual_height, was_clicked)
    fn render_visible(&self, ctx: &mut RenderContext) -> (f32, bool);

    /// Get the default width estimate for this chunk type
    fn estimated_width(&self) -> f32 {
        600.0
    }

    /// Render a chunk with full viewport culling and caching logic
    fn render(&self, ctx: &mut RenderContext, culler: &ViewportCuller) -> bool {
        let start_pos = ctx.ui.cursor().min;
        let estimated_height = self.estimate_height(ctx.chunk, ctx.theme);

        let approx_rect =
            ViewportCuller::create_approx_rect(start_pos, self.estimated_width(), estimated_height);
        let before_y = ctx.ui.cursor().min.y;

        let mut was_clicked = false;

        if culler.intersects_viewport(approx_rect) {
            // In viewport - render and cache actual height
            let (actual_height, clicked) = self.render_visible(ctx);
            ctx.chunk.cached_height = Some(actual_height);
            was_clicked = clicked;
        } else {
            // Outside viewport - use cached height or estimate
            let height = ViewportCuller::get_height(ctx.chunk, estimated_height);
            ctx.ui.add_space(height);
        }

        let after_y = ctx.ui.cursor().min.y;

        // Record position in layout map (only if needed for selection)
        if ctx.need_layout_map {
            ctx.layout_map.record_chunk(
                ctx.chunk.line_start,
                ctx.chunk.line_end,
                before_y,
                after_y,
            );
        }

        // Add paragraph spacing if needed
        if ctx.chunk.newline_after {
            ctx.ui.add_space(ctx.theme.spacing.paragraph);
        }

        was_clicked
    }
}

/// Renderer for text chunks
pub struct TextRenderer;

impl ChunkRenderer for TextRenderer {
    fn estimate_height(&self, chunk: &TextChunk, theme: &Theme) -> f32 {
        let line_count = chunk.text.lines().count().max(1);
        line_count as f32 * theme.spacing.min_line_height
    }

    fn render_visible(&self, ctx: &mut RenderContext) -> (f32, bool) {
        let before_y = ctx.ui.cursor().min.y;

        let response = crate::rendering::text::render_text_chunk(ctx.ui, ctx.chunk, ctx.theme);

        let after_y = ctx.ui.cursor().min.y;
        let actual_height = after_y - before_y;

        // Handle selection interactions
        let mut selection_manager = SelectionManager::new(ctx.selection, ctx.layout_map);
        let was_clicked =
            selection_manager.handle_interaction(&response, ctx.chunk, before_y, after_y);

        (actual_height, was_clicked)
    }
}

/// Renderer for code blocks
pub struct CodeRenderer;

impl ChunkRenderer for CodeRenderer {
    fn estimate_height(&self, chunk: &TextChunk, theme: &Theme) -> f32 {
        let line_count = chunk.text.lines().count().max(1);
        (line_count as f32 * theme.spacing.min_line_height) + theme.spacing.code_block_padding * 2.0
    }

    fn render_visible(&self, ctx: &mut RenderContext) -> (f32, bool) {
        let before_y = ctx.ui.cursor().min.y;

        // Code blocks are not selectable via dragging
        let lang = ctx.chunk.code_block_lang.as_deref().unwrap_or("");
        crate::rendering::code::render_code_block(
            ctx.ui,
            &ctx.chunk.text,
            lang,
            ctx.highlighter,
            ctx.theme,
        );

        let after_y = ctx.ui.cursor().min.y;
        let actual_height = after_y - before_y;

        (actual_height, false)
    }
}

/// Renderer for tables
pub struct TableRenderer;

impl ChunkRenderer for TableRenderer {
    fn estimate_height(&self, chunk: &TextChunk, theme: &Theme) -> f32 {
        if let Some(ref table) = chunk.table {
            (table.rows.len() as f32 + 1.0) * theme.spacing.min_line_height
        } else {
            theme.spacing.min_line_height
        }
    }

    fn render_visible(&self, ctx: &mut RenderContext) -> (f32, bool) {
        let before_y = ctx.ui.cursor().min.y;
        let start_pos = ctx.ui.cursor().min;

        if let Some(ref table_data) = ctx.chunk.table {
            crate::rendering::table::render_table(ctx.ui, table_data, ctx.theme, ctx.idx);
        }

        let after_y = ctx.ui.cursor().min.y;
        let actual_height = after_y - before_y;

        // Sense drag on the table area for selection
        let table_rect = egui::Rect::from_min_size(
            start_pos,
            egui::vec2(ctx.ui.available_width(), actual_height),
        );
        let table_response = ctx.ui.interact(
            table_rect,
            ctx.ui.id().with(("table_sense", ctx.idx)),
            egui::Sense::click_and_drag(),
        );

        // Handle selection interactions
        let mut selection_manager = SelectionManager::new(ctx.selection, ctx.layout_map);
        let was_clicked =
            selection_manager.handle_interaction(&table_response, ctx.chunk, before_y, after_y);

        (actual_height, was_clicked)
    }
}

/// Renderer for images
pub struct ImageRenderer;

impl ChunkRenderer for ImageRenderer {
    fn estimate_height(&self, _chunk: &TextChunk, _theme: &Theme) -> f32 {
        300.0 // Default image height estimate
    }

    fn estimated_width(&self) -> f32 {
        400.0
    }

    fn render_visible(&self, ctx: &mut RenderContext) -> (f32, bool) {
        let before_y = ctx.ui.cursor().min.y;

        if let Some(ref image_path) = ctx.chunk.image_path {
            // Load and render
            crate::rendering::image::load_image_texture(ctx.ctx, image_path, ctx.loaded_images);
            if let Some(response) = crate::rendering::image::render_image(
                ctx.ui,
                image_path,
                ctx.loaded_images,
                ctx.chunk.alignment.clone(),
                ctx.chunk.image_width,
            ) {
                let actual_height = response.rect.height();

                // Images can be selected by clicking (single line selection)
                let mut selection_manager = SelectionManager::new(ctx.selection, ctx.layout_map);
                selection_manager.handle_single_line_interaction(&response, ctx.chunk.line_start);

                return (actual_height, false);
            }
        }

        let after_y = ctx.ui.cursor().min.y;
        (after_y - before_y, false)
    }
}
