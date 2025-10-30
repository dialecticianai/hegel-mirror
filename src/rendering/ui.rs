use crate::image_manager::ImageManager;
use crate::models::{LayoutMap, Selection, TextChunk};
use crate::rendering::chunk;
use crate::rendering::inline_batcher::InlineTextBatcher;
use crate::rendering::selection_manager::SelectionManager;
use crate::rendering::viewport::ViewportCuller;
use crate::syntax::SyntaxHighlighter;
use crate::theme::Theme;
use eframe::egui;

/// Render the main UI with markdown content (with stable lazy loading)
pub fn render_content(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    chunks: &mut [TextChunk],
    selection: &mut Selection,
    image_manager: &mut ImageManager,
    highlighter: &SyntaxHighlighter,
    theme: &Theme,
    layout_map: &mut LayoutMap,
) {
    // Handle drag release
    {
        let mut selection_manager = SelectionManager::new(selection, layout_map);
        selection_manager.handle_drag_release(ui);
    }

    // Only build layout map if we have an active selection or are dragging
    let need_layout_map = selection.is_active() || selection.is_dragging;

    // Create viewport culler for this frame
    let mut culler = ViewportCuller::new(ui);

    // Track if any chunk was clicked (to distinguish click from drag)
    let mut any_chunk_clicked = false;

    let mut idx = 0;
    while idx < chunks.len() {
        let start_pos = ui.cursor().min;

        // Check if we can skip rendering early (past viewport with cached height)
        if !culler.should_render(start_pos, 0.0) && chunks[idx].cached_height.is_some() {
            // Fast path: skip this chunk entirely
            culler.render_offscreen(ui, &mut chunks[idx], 0.0, theme);
            idx += 1;
            continue;
        }

        // Check if this is an inline text chunk
        if InlineTextBatcher::is_inline_chunk(&chunks[idx]) {
            // Find the batch of consecutive inline chunks
            if let Some((batch_start, batch_end)) =
                InlineTextBatcher::find_inline_batch(chunks, idx)
            {
                // Batch render consecutive inline text chunks in horizontal_wrapped layout
                ui.horizontal_wrapped(|ui| {
                    for local_idx in batch_start..batch_end {
                        let was_clicked = chunk::render_chunk(
                            ui,
                            ctx,
                            &mut chunks[local_idx],
                            local_idx,
                            selection,
                            image_manager,
                            highlighter,
                            theme,
                            layout_map,
                            need_layout_map,
                            &culler,
                        );

                        if was_clicked {
                            any_chunk_clicked = true;
                        }
                    }
                });

                // Move index past the batched chunks
                idx = batch_end;
                continue;
            }
        }

        // Render non-inline chunks normally (images, code blocks, tables, headings)
        let was_clicked = chunk::render_chunk(
            ui,
            ctx,
            &mut chunks[idx],
            idx,
            selection,
            image_manager,
            highlighter,
            theme,
            layout_map,
            need_layout_map,
            &culler,
        );

        if was_clicked {
            any_chunk_clicked = true;
        }

        idx += 1;
    }

    // Clear selection if clicked without dragging
    if any_chunk_clicked && !selection.is_dragging {
        selection.clear();
    }

    // Update selection based on hover (second pass after all chunks rendered)
    // This ensures bidirectional drag works (both up and down)
    {
        let mut selection_manager = SelectionManager::new(selection, layout_map);
        selection_manager.update_from_hover(ui);
    }

    // Draw selection bar using layout map
    {
        let selection_manager = SelectionManager::new(selection, layout_map);
        selection_manager.draw_selection_bar(ui, theme);
    }
}
