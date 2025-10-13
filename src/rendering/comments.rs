use crate::models::{Comment, LayoutMap, Selection};
use crate::theme::Theme;
use eframe::egui;

/// Render the comment UI section as a floating panel in the right margin
pub fn render_comment_section(
    ctx: &egui::Context,
    layout_map: &LayoutMap,
    selection: &Selection,
    comment_text: &mut String,
    comments: &mut Vec<Comment>,
    theme: &Theme,
) {
    // Only show if there's an active selection
    if let (Some(start_line), Some(end_line)) = (selection.start_line, selection.end_line) {
        render_comment_input(
            ctx,
            layout_map,
            start_line,
            end_line,
            comment_text,
            comments,
            theme,
        );
    }

    // Show existing comments list at the bottom
    if !comments.is_empty() {
        render_comments_list(ctx, comments, theme);
    }
}

fn render_comment_input(
    ctx: &egui::Context,
    layout_map: &LayoutMap,
    start_line: usize,
    end_line: usize,
    comment_text: &mut String,
    comments: &mut Vec<Comment>,
    theme: &Theme,
) {
    let (min_line, max_line) = if start_line <= end_line {
        (start_line, end_line)
    } else {
        (end_line, start_line)
    };

    // Get the Y position for the selection start and end
    if let Some(selection_y_start) = layout_map.get_line_y(min_line) {
        let selection_y_end = layout_map.get_line_y(max_line).unwrap_or(selection_y_start);

        // Position the comment box at the selection Y position, in the right margin
        let screen_width = ctx.screen_rect().width();
        let screen_height = ctx.screen_rect().height();
        let window_x =
            screen_width - theme.layout.comment_box_width - theme.layout.comment_box_margin_right;

        // Clamp comment box to viewport with some padding
        let viewport_padding = 20.0;
        let clamped_y = selection_y_start
            .max(viewport_padding)
            .min(screen_height - theme.layout.comment_box_height - viewport_padding);

        // Determine if selection is off-screen
        let selection_above_viewport = selection_y_end < viewport_padding;
        let selection_below_viewport = selection_y_start > screen_height - viewport_padding;

        egui::Window::new("Add Comment")
            .fixed_pos(egui::pos2(window_x, clamped_y))
            .fixed_size(egui::vec2(
                theme.layout.comment_box_width,
                theme.layout.comment_box_height,
            ))
            .resizable(false)
            .collapsible(false)
            .title_bar(false)
            .frame(egui::Frame::window(&ctx.style()).inner_margin(10.0))
            .show(ctx, |ui| {
                // Show scroll indicator if selection is off-screen
                if selection_above_viewport {
                    ui.horizontal(|ui| {
                        ui.label("↑");
                        ui.label(egui::RichText::new("Scroll up to see selection").weak());
                    });
                    ui.separator();
                } else if selection_below_viewport {
                    ui.horizontal(|ui| {
                        ui.label("↓");
                        ui.label(egui::RichText::new("Scroll down to see selection").weak());
                    });
                    ui.separator();
                }

                ui.label(format!("Selection: Lines {}-{}", min_line, max_line));
                ui.add_space(5.0);

                ui.label("Comment:");
                ui.text_edit_multiline(comment_text);
                ui.add_space(5.0);

                if ui.button("Add Comment").clicked() && !comment_text.is_empty() {
                    comments.push(Comment::new(
                        comment_text.clone(),
                        min_line,
                        0, // col_start: 0 (beginning of line)
                        max_line,
                        0, // col_end: 0 (simplified for MVP)
                    ));
                    comment_text.clear();
                }
            });
    }
}

fn render_comments_list(ctx: &egui::Context, comments: &[Comment], theme: &Theme) {
    let screen_height = ctx.screen_rect().height();
    egui::Window::new("Comments")
        .fixed_pos(egui::pos2(
            theme.layout.comments_list_margin_left,
            screen_height - theme.layout.comments_list_margin_bottom,
        ))
        .fixed_size(egui::vec2(
            theme.layout.comments_list_width,
            theme.layout.comments_list_height,
        ))
        .resizable(false)
        .collapsible(true)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for comment in comments {
                    ui.label(comment.format());
                    ui.separator();
                }
            });
        });
}
