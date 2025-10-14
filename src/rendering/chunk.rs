use crate::models::{LayoutMap, Selection, TextChunk};
use crate::rendering::{code, image, table, text};
use crate::syntax::SyntaxHighlighter;
use crate::theme::Theme;
use eframe::egui;
use std::collections::HashMap;

/// Check if a widget rect is within the visible viewport
fn is_in_viewport(ui: &egui::Ui, widget_rect: egui::Rect) -> bool {
    let viewport = ui.clip_rect();
    widget_rect.intersects(viewport)
}

/// Calculate which line within a chunk corresponds to a given Y position
/// Returns the precise line number based on interpolation within the chunk
pub fn calculate_line_from_y(
    line_start: usize,
    line_end: usize,
    chunk_y_start: f32,
    chunk_y_end: f32,
    y_pos: f32,
) -> usize {
    let chunk_height = chunk_y_end - chunk_y_start;
    let line_count = (line_end - line_start + 1).max(1);
    let y_offset = y_pos - chunk_y_start;
    let line_height = chunk_height / line_count as f32;
    let line_index = (y_offset / line_height).floor() as usize;
    line_start + line_index.min(line_count - 1)
}

/// Handle common selection interactions (drag start, click) for any response
/// Returns true if clicked (for click-to-clear detection)
fn handle_selection_interaction(
    response: &egui::Response,
    chunk: &TextChunk,
    before_y: f32,
    after_y: f32,
    selection: &mut Selection,
) -> bool {
    let mut was_clicked = false;

    // Handle drag start with precise line calculation
    if response.drag_started() {
        if let Some(interact_pos) = response.interact_pointer_pos() {
            let precise_line = calculate_line_from_y(
                chunk.line_start,
                chunk.line_end,
                before_y,
                after_y,
                interact_pos.y,
            );
            selection.start_drag(precise_line);
        } else {
            selection.start_drag(chunk.line_start);
        }
    }

    // Track clicks for click-to-clear
    if response.clicked() {
        was_clicked = true;
    }

    was_clicked
}

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
) -> bool {
    let start_pos = ui.cursor().min;
    let mut chunk_was_clicked = false;

    // Clone data needed for rendering to avoid borrow issues
    let image_path = chunk.image_path.clone();
    let code_block_lang = chunk.code_block_lang.clone();
    let table = chunk.table.clone();

    if let Some(image_path) = image_path {
        render_image_chunk(
            ui,
            ctx,
            chunk,
            &image_path,
            loaded_images,
            selection,
            layout_map,
            need_layout_map,
            start_pos,
        );
    } else if let Some(lang) = code_block_lang {
        render_code_chunk(
            ui,
            chunk,
            &lang,
            highlighter,
            theme,
            layout_map,
            need_layout_map,
            start_pos,
        );
    } else if let Some(table_data) = table {
        chunk_was_clicked = render_table_chunk(
            ui,
            chunk,
            &table_data,
            idx,
            selection,
            theme,
            layout_map,
            need_layout_map,
            start_pos,
        );
    } else {
        chunk_was_clicked = render_text_chunk(
            ui,
            chunk,
            selection,
            theme,
            layout_map,
            need_layout_map,
            start_pos,
        );
    }

    // Add paragraph spacing if needed
    if chunk.newline_after {
        ui.add_space(theme.spacing.paragraph);
    }

    chunk_was_clicked
}

fn render_image_chunk(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    chunk: &mut TextChunk,
    image_path: &str,
    loaded_images: &mut HashMap<String, egui::TextureHandle>,
    selection: &mut Selection,
    layout_map: &mut LayoutMap,
    need_layout_map: bool,
    start_pos: egui::Pos2,
) {
    let estimated_height = chunk.cached_height.unwrap_or(300.0);
    let approx_rect = egui::Rect::from_min_size(start_pos, egui::vec2(400.0, estimated_height));

    let before_y = ui.cursor().min.y;

    if is_in_viewport(ui, approx_rect) {
        // Load and render, cache actual height
        image::load_image_texture(ctx, image_path, loaded_images);
        if let Some(response) = image::render_image(
            ui,
            image_path,
            loaded_images,
            chunk.alignment.clone(),
            chunk.image_width,
        ) {
            chunk.cached_height = Some(response.rect.height());
            // Images can be selected by clicking (single line selection)
            if response.clicked() {
                selection.start_drag(chunk.line_start);
                selection.end_drag();
            }
        }
    } else {
        // Use cached height for stable placeholder
        ui.add_space(estimated_height);
    }

    let after_y = ui.cursor().min.y;
    // Record position in layout map (only if needed for selection)
    if need_layout_map {
        layout_map.record_chunk(chunk.line_start, chunk.line_end, before_y, after_y);
    }
}

fn render_code_chunk(
    ui: &mut egui::Ui,
    chunk: &mut TextChunk,
    lang: &str,
    highlighter: &SyntaxHighlighter,
    theme: &Theme,
    layout_map: &mut LayoutMap,
    need_layout_map: bool,
    start_pos: egui::Pos2,
) {
    let line_count = chunk.text.lines().count().max(1);
    let estimated_height = chunk.cached_height.unwrap_or(
        (line_count as f32 * theme.spacing.min_line_height)
            + theme.spacing.code_block_padding * 2.0,
    );
    let approx_rect = egui::Rect::from_min_size(start_pos, egui::vec2(600.0, estimated_height));

    let before_y = ui.cursor().min.y;

    if is_in_viewport(ui, approx_rect) {
        // Render and cache actual height
        code::render_code_block(ui, &chunk.text, lang, highlighter, theme);
        let after_y = ui.cursor().min.y;
        chunk.cached_height = Some(after_y - before_y);
    } else {
        // Use cached height for stable placeholder
        ui.add_space(estimated_height);
    }

    let after_y = ui.cursor().min.y;
    // Record position in layout map (only if needed for selection)
    if need_layout_map {
        layout_map.record_chunk(chunk.line_start, chunk.line_end, before_y, after_y);
    }
}

fn render_table_chunk(
    ui: &mut egui::Ui,
    chunk: &mut TextChunk,
    table_data: &crate::models::Table,
    idx: usize,
    selection: &mut Selection,
    theme: &Theme,
    layout_map: &mut LayoutMap,
    need_layout_map: bool,
    start_pos: egui::Pos2,
) -> bool {
    let estimated_height = chunk
        .cached_height
        .unwrap_or((table_data.rows.len() as f32 + 1.0) * theme.spacing.min_line_height);
    let approx_rect = egui::Rect::from_min_size(start_pos, egui::vec2(600.0, estimated_height));

    let before_y = ui.cursor().min.y;
    let mut chunk_was_clicked = false;

    if is_in_viewport(ui, approx_rect) {
        // Render and cache actual height
        table::render_table(ui, table_data, theme, idx);
        let after_y = ui.cursor().min.y;
        chunk.cached_height = Some(after_y - before_y);

        // Sense drag on the table area for selection
        let table_rect = egui::Rect::from_min_size(
            start_pos,
            egui::vec2(ui.available_width(), after_y - before_y),
        );
        let table_response = ui.interact(
            table_rect,
            ui.id().with(("table_sense", idx)),
            egui::Sense::click_and_drag(),
        );

        // Handle selection interactions
        if handle_selection_interaction(&table_response, chunk, before_y, after_y, selection) {
            chunk_was_clicked = true;
        }
    } else {
        // Use cached height for stable placeholder
        ui.add_space(estimated_height);
    }

    let after_y = ui.cursor().min.y;
    // Record position in layout map (only if needed for selection)
    if need_layout_map {
        layout_map.record_chunk(chunk.line_start, chunk.line_end, before_y, after_y);
    }

    chunk_was_clicked
}

fn render_text_chunk(
    ui: &mut egui::Ui,
    chunk: &mut TextChunk,
    selection: &mut Selection,
    theme: &Theme,
    layout_map: &mut LayoutMap,
    need_layout_map: bool,
    start_pos: egui::Pos2,
) -> bool {
    let before_y = ui.cursor().min.y;
    let mut chunk_was_clicked = false;

    // Estimate height based on line count
    let line_count = chunk.text.lines().count().max(1);
    let estimated_height = line_count as f32 * theme.spacing.min_line_height;
    let approx_rect = egui::Rect::from_min_size(start_pos, egui::vec2(600.0, estimated_height));

    if is_in_viewport(ui, approx_rect) {
        // In viewport - render and cache actual height
        let response = text::render_text_chunk(ui, chunk, theme);
        let after_y = ui.cursor().min.y;
        chunk.cached_height = Some(after_y - before_y);

        // Record position in layout map (only if needed for selection)
        if need_layout_map {
            layout_map.record_chunk(chunk.line_start, chunk.line_end, before_y, after_y);
        }

        // Handle selection interactions
        if handle_selection_interaction(&response, chunk, before_y, after_y, selection) {
            chunk_was_clicked = true;
        }
    } else {
        // Outside viewport - use cached height if available, otherwise estimate
        let height = chunk.cached_height.unwrap_or(estimated_height);
        ui.add_space(height);
        let after_y = ui.cursor().min.y;

        // Record position in layout map (only if needed for selection)
        if need_layout_map {
            layout_map.record_chunk(chunk.line_start, chunk.line_end, before_y, after_y);
        }
    }

    chunk_was_clicked
}
