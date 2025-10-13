use crate::models::Table;
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
                    render_cell(ui, cell, alignment, theme, true);
                }
                ui.end_row();
            }

            // Render body rows (skip first row if it was header)
            for row in table.rows.iter().skip(1) {
                for (idx, cell) in row.iter().enumerate() {
                    let alignment = table
                        .alignments
                        .get(idx)
                        .copied()
                        .unwrap_or(Alignment::None);
                    render_cell(ui, cell, alignment, theme, false);
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
) {
    let text_color = if is_header {
        theme.colors.heading
    } else {
        theme.colors.text
    };

    let font_id = if is_header {
        egui::FontId::new(theme.typography.body_size, egui::FontFamily::Proportional)
    } else {
        egui::FontId::new(theme.typography.body_size, egui::FontFamily::Proportional)
    };

    let label = egui::RichText::new(text).color(text_color).font(font_id);

    // Disable built-in text selection in table cells
    match alignment {
        Alignment::Center => {
            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                |ui| {
                    ui.add(egui::Label::new(label).selectable(false));
                },
            );
        }
        Alignment::Right => {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add(egui::Label::new(label).selectable(false));
            });
        }
        Alignment::Left | Alignment::None => {
            ui.add(egui::Label::new(label).selectable(false));
        }
    }
}
