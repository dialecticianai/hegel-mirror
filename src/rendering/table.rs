use crate::models::Table;
use crate::rendering::text_builder::TextContext;
use crate::theme::Theme;
use eframe::egui;
use pulldown_cmark::Alignment;

pub fn render_table(ui: &mut egui::Ui, table: &Table, theme: &Theme, chunk_idx: usize) {
    let table_id = ui.id().with(("table", chunk_idx));

    egui::Grid::new(table_id)
        .striped(true)
        .spacing([
            theme.spacing.table_cell_padding * 2.0,
            theme.spacing.table_cell_padding,
        ])
        .show(ui, |ui| {
            // Render header row
            if let Some(header_row) = table.rows.first() {
                for (idx, cell) in header_row.iter().enumerate() {
                    let alignment = table
                        .alignments
                        .get(idx)
                        .copied()
                        .unwrap_or(Alignment::None);
                    render_cell(ui, cell, alignment, theme, true, ("header", chunk_idx, idx));
                }
                ui.end_row();
            }

            // Render body rows (skip first row if it was header)
            for (row_idx, row) in table.rows.iter().skip(1).enumerate() {
                for (col_idx, cell) in row.iter().enumerate() {
                    let alignment = table
                        .alignments
                        .get(col_idx)
                        .copied()
                        .unwrap_or(Alignment::None);
                    render_cell(
                        ui,
                        cell,
                        alignment,
                        theme,
                        false,
                        ("body", chunk_idx, row_idx, col_idx),
                    );
                }
                ui.end_row();
            }
        });
}

fn render_cell(
    ui: &mut egui::Ui,
    text: &str,
    alignment: Alignment,
    theme: &Theme,
    is_header: bool,
    _cell_id: impl std::hash::Hash + Copy,
) {
    // Build styled text for table cells
    // Note: We use build_styled_text but render with regular Label, not EmojiLabel
    // because EmojiLabel doesn't work well inside Grid cells with alignment layouts
    let styled_text = crate::rendering::text_builder::build_styled_text(
        text,
        false, // bold
        false, // italic
        false, // code
        TextContext::TableCell { is_header },
        theme,
    );

    let label = egui::Label::new(styled_text).selectable(false);

    // Handle alignment the same way as before emoji support
    match alignment {
        Alignment::Center => {
            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                |ui| {
                    ui.add(label);
                },
            );
        }
        Alignment::Right => {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add(label);
            });
        }
        Alignment::Left | Alignment::None => {
            ui.add(label);
        }
    }
}
