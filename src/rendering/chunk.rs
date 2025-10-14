use crate::models::{LayoutMap, Selection, TextChunk};
use crate::rendering::chunk_renderer::{
    ChunkRenderer, CodeRenderer, ImageRenderer, RenderContext, TableRenderer, TextRenderer,
};
use crate::rendering::viewport::ViewportCuller;
use crate::syntax::SyntaxHighlighter;
use crate::theme::Theme;
use eframe::egui;
use std::collections::HashMap;

/// Render a single chunk (image, code, table, or text) with viewport culling
/// Returns true if the chunk was clicked
pub fn render_chunk(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    chunk: &mut TextChunk,
    idx: usize,
    selection: &mut Selection,
    loaded_images: &mut HashMap<String, egui::TextureHandle>,
    highlighter: &SyntaxHighlighter,
    theme: &Theme,
    layout_map: &mut LayoutMap,
    need_layout_map: bool,
    culler: &ViewportCuller,
) -> bool {
    // Build render context
    let mut render_ctx = RenderContext {
        ui,
        ctx,
        chunk,
        idx,
        selection,
        loaded_images,
        highlighter,
        theme,
        layout_map,
        need_layout_map,
    };

    // Select appropriate renderer based on chunk type
    if render_ctx.chunk.image_path.is_some() {
        ImageRenderer.render(&mut render_ctx, culler)
    } else if render_ctx.chunk.code_block_lang.is_some() {
        CodeRenderer.render(&mut render_ctx, culler)
    } else if render_ctx.chunk.table.is_some() {
        TableRenderer.render(&mut render_ctx, culler)
    } else {
        TextRenderer.render(&mut render_ctx, culler)
    }
}
