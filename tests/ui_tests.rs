use egui_kittest::kittest::Queryable;
/// UI integration tests using egui_kittest
use egui_kittest::Harness;
use mirror::image_manager::ImageManager;
use mirror::parse_markdown;
use std::path::Path;

#[test]
fn test_render_simple_text() {
    let markdown = "Hello, world!";
    let mut image_manager = ImageManager::new(Path::new("."));
    let chunks = parse_markdown(markdown, Path::new("."), &mut image_manager);

    let mut harness = Harness::new_ui(|ui| {
        for chunk in &chunks {
            ui.label(&chunk.text);
        }
    });

    harness.run();

    // Should find the label with our text (query doesn't panic = success)
    let _label = harness.get_by_label("Hello, world!");
}

#[test]
fn test_render_heading() {
    let markdown = "# Main Heading";
    let mut image_manager = ImageManager::new(Path::new("."));
    let chunks = parse_markdown(markdown, Path::new("."), &mut image_manager);

    let mut harness = Harness::new_ui(|ui| {
        for chunk in &chunks {
            if chunk.heading_level.is_some() {
                ui.heading(&chunk.text);
            } else {
                ui.label(&chunk.text);
            }
        }
    });

    harness.run();

    let _heading = harness.get_by_label("Main Heading");
}

#[test]
fn test_render_code_block() {
    let markdown = r#"```rust
fn main() {}
```"#;
    let mut image_manager = ImageManager::new(Path::new("."));
    let chunks = parse_markdown(markdown, Path::new("."), &mut image_manager);

    let mut harness = Harness::new_ui(|ui| {
        for chunk in &chunks {
            if chunk.code {
                ui.code(&chunk.text);
            }
        }
    });

    harness.run();

    // Code should be rendered
    let code_chunks: Vec<_> = chunks.iter().filter(|c| c.code).collect();
    assert!(code_chunks.len() > 0, "Should have code chunks");
}

#[test]
fn test_render_multiple_paragraphs() {
    let markdown = "Paragraph 1\n\nParagraph 2\n\nParagraph 3";
    let mut image_manager = ImageManager::new(Path::new("."));
    let chunks = parse_markdown(markdown, Path::new("."), &mut image_manager);

    let mut harness = Harness::new_ui(|ui| {
        for chunk in &chunks {
            ui.label(&chunk.text);
            if chunk.newline_after {
                ui.add_space(10.0);
            }
        }
    });

    harness.run();

    // Should have parsed multiple text chunks
    let text_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| !c.text.trim().is_empty())
        .collect();
    assert!(text_chunks.len() >= 2, "Should have multiple paragraphs");
}

#[test]
fn test_render_with_selection_state() {
    use mirror::models::Selection;

    let markdown = "Line 1\nLine 2\nLine 3";
    let mut image_manager = ImageManager::new(Path::new("."));
    let chunks = parse_markdown(markdown, Path::new("."), &mut image_manager);
    let mut selection = Selection::default();

    // Start a selection
    selection.start_drag(2);
    selection.update_drag(3);

    let mut harness = Harness::new_ui(|ui| {
        for chunk in &chunks {
            // Render with selection highlight
            let is_selected = selection.contains_line(chunk.line_start);
            if is_selected {
                ui.colored_label(egui::Color32::LIGHT_BLUE, &chunk.text);
            } else {
                ui.label(&chunk.text);
            }
        }
    });

    harness.run();

    // Selection state should be active
    assert!(selection.is_active());
    assert!(selection.contains_line(2));
    assert!(selection.contains_line(3));
}

#[test]
fn test_render_styled_text() {
    let markdown = "This is **bold** and *italic* text";
    let mut image_manager = ImageManager::new(Path::new("."));
    let chunks = parse_markdown(markdown, Path::new("."), &mut image_manager);

    let mut harness = Harness::new_ui(|ui| {
        ui.horizontal(|ui| {
            for chunk in &chunks {
                let mut job = egui::text::LayoutJob::default();
                let format = egui::TextFormat {
                    font_id: egui::FontId::proportional(14.0),
                    color: egui::Color32::WHITE,
                    italics: chunk.italic,
                    ..Default::default()
                };
                job.append(&chunk.text, 0.0, format);
                ui.label(job);
            }
        });
    });

    harness.run();

    // Should have bold and italic chunks
    let bold_chunks: Vec<_> = chunks.iter().filter(|c| c.bold).collect();
    let italic_chunks: Vec<_> = chunks.iter().filter(|c| c.italic).collect();

    assert!(bold_chunks.len() > 0, "Should have bold text");
    assert!(italic_chunks.len() > 0, "Should have italic text");
}

#[test]
fn test_clickable_line() {
    let markdown = "Click me";
    let mut image_manager = ImageManager::new(Path::new("."));
    let chunks = parse_markdown(markdown, Path::new("."), &mut image_manager);
    let mut clicked = false;

    let mut harness = Harness::new_state(
        |ctx, clicked| {
            egui::CentralPanel::default().show(ctx, |ui| {
                for chunk in &chunks {
                    if ui.button(&chunk.text).clicked() {
                        **clicked = true;
                    }
                }
            });
        },
        &mut clicked,
    );

    // Find and click the button
    let button = harness.get_by_label("Click me");
    button.click();
    harness.run();

    assert!(**harness.state(), "Button should have been clicked");
}

#[test]
fn test_layout_with_margins() {
    let markdown = "Test content";
    let mut image_manager = ImageManager::new(Path::new("."));
    let chunks = parse_markdown(markdown, Path::new("."), &mut image_manager);

    let mut harness = Harness::new_ui(|ui| {
        // Add margins
        ui.add_space(20.0);
        ui.horizontal(|ui| {
            ui.add_space(10.0);
            for chunk in &chunks {
                ui.label(&chunk.text);
            }
        });
    });

    harness.run();

    let label = harness.get_by_label("Test content");

    // Check that the label has a position (margins applied)
    let rect = label.rect();
    assert!(rect.min.x >= 10.0, "Should have left margin");
}

#[test]
fn test_multiline_document_rendering() {
    let markdown = include_str!("fixtures/basic.md");
    let mut image_manager = ImageManager::new(Path::new("."));
    let chunks = parse_markdown(markdown, Path::new("."), &mut image_manager);

    let mut harness = Harness::new_ui(|ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            for chunk in &chunks {
                if let Some(level) = chunk.heading_level {
                    let size = match level {
                        1 => 24.0,
                        2 => 20.0,
                        _ => 16.0,
                    };
                    ui.label(egui::RichText::new(&chunk.text).size(size).strong());
                } else if chunk.code {
                    ui.code(&chunk.text);
                } else {
                    ui.label(&chunk.text);
                }

                if chunk.newline_after {
                    ui.add_space(8.0);
                }
            }
        });
    });

    harness.run();

    // Should have rendered the document
    assert!(chunks.len() > 10, "Basic.md should have many chunks");
    // UI rendered successfully (no panic)
}

#[test]
fn test_comment_input_dialog() {
    #[derive(Default)]
    struct DialogState {
        comment_text: String,
        show_dialog: bool,
    }

    let mut harness = Harness::new_state(
        |ctx, state: &mut DialogState| {
            if state.show_dialog {
                egui::Window::new("Add Comment").show(ctx, |ui| {
                    ui.text_edit_singleline(&mut state.comment_text);
                    if ui.button("Submit").clicked() {
                        state.show_dialog = false;
                    }
                });
            }
        },
        DialogState {
            comment_text: String::new(),
            show_dialog: true,
        },
    );

    harness.run();

    // Dialog rendered successfully (no panic)
}

#[test]
fn test_table_rendering_structure() {
    let markdown = include_str!("fixtures/tables.md");
    let mut image_manager = ImageManager::new(Path::new("."));
    let chunks = parse_markdown(markdown, Path::new("."), &mut image_manager);

    // Find table chunks
    let table_chunks: Vec<_> = chunks.iter().filter(|c| c.table.is_some()).collect();
    assert!(table_chunks.len() > 0, "Should have table chunks");

    let mut harness = Harness::builder()
        .with_max_steps(10) // Grid needs more steps to stabilize
        .build_ui(|ui| {
            for chunk in &chunks {
                if let Some(table) = &chunk.table {
                    // Simple table rendering test
                    egui::Grid::new("table").show(ui, |ui| {
                        // Header
                        for cell in &table.header {
                            ui.label(cell);
                        }
                        ui.end_row();

                        // Rows
                        for row in &table.rows {
                            for cell in row {
                                ui.label(cell);
                            }
                            ui.end_row();
                        }
                    });
                }
            }
        });

    // Grid may keep requesting repaints, use run_ok() which doesn't panic
    let _ = harness.run_ok();

    // Tables should be parsed
    let first_table = table_chunks[0].table.as_ref().unwrap();
    assert!(first_table.alignments.len() > 0);
}

#[test]
fn test_scroll_area_with_long_content() {
    let mut long_markdown = String::new();
    for i in 1..=50 {
        long_markdown.push_str(&format!("Line {}\n", i));
    }
    let mut image_manager = ImageManager::new(Path::new("."));
    let chunks = parse_markdown(&long_markdown, Path::new("."), &mut image_manager);

    let mut harness = Harness::builder()
        .with_size(egui::vec2(400.0, 300.0))
        .build_ui(|ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for chunk in &chunks {
                    ui.label(&chunk.text);
                }
            });
        });

    harness.run();

    // Should have created many line chunks
    assert!(chunks.len() > 40, "Should have many lines");
}

#[test]
fn test_responsive_layout() {
    let markdown = "Test content for responsive layout";
    let mut image_manager = ImageManager::new(Path::new("."));
    let chunks = parse_markdown(markdown, Path::new("."), &mut image_manager);

    // Test with small window
    let mut harness_small = Harness::builder()
        .with_size(egui::vec2(200.0, 100.0))
        .build_ui(|ui| {
            for chunk in &chunks {
                ui.label(&chunk.text);
            }
        });

    harness_small.run();

    // Test with large window
    let mut harness_large = Harness::builder()
        .with_size(egui::vec2(800.0, 600.0))
        .build_ui(|ui| {
            for chunk in &chunks {
                ui.label(&chunk.text);
            }
        });

    harness_large.run();

    // Both should render successfully (queries don't panic = success)
    let _label_small = harness_small.get_by_label("Test content for responsive layout");
    let _label_large = harness_large.get_by_label("Test content for responsive layout");
}
