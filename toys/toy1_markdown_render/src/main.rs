use eframe::egui;
use egui::text_selection::LabelSelectionState;
use std::env;
use std::fs;

fn main() -> eframe::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <markdown-file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let markdown_content = fs::read_to_string(file_path).unwrap_or_else(|e| {
        eprintln!("Failed to read {}: {}", file_path, e);
        std::process::exit(1);
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Mirror - Markdown Render Toy",
        options,
        Box::new(|_cc| Ok(Box::new(MarkdownApp::new(markdown_content)))),
    )
}

struct MarkdownApp {
    markdown_source: String,
    cache: egui_commonmark::CommonMarkCache,
    selected_text: Option<String>,
    last_selection_check: std::time::Instant,
    show_comment_dialog: bool,
    comment_input: String,
    comments: Vec<Comment>,
}

#[derive(Clone, Debug)]
struct Comment {
    selected_text: String,
    comment: String,
    line_range: Option<(usize, usize)>, // (start_line, end_line)
}

impl MarkdownApp {
    fn new(markdown_source: String) -> Self {
        Self {
            markdown_source,
            cache: egui_commonmark::CommonMarkCache::default(),
            selected_text: None,
            last_selection_check: std::time::Instant::now(),
            show_comment_dialog: false,
            comment_input: String::new(),
            comments: Vec::new(),
        }
    }
}

impl eframe::App for MarkdownApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Log frame time (FPS calculation)
        let frame_time = ctx.input(|i| i.stable_dt);
        let fps = if frame_time > 0.0 {
            1.0 / frame_time
        } else {
            0.0
        };

        // Only log FPS every 60 frames to avoid spam
        static mut FRAME_COUNT: u64 = 0;
        unsafe {
            FRAME_COUNT += 1;
            if FRAME_COUNT % 60 == 0 {
                println!(
                    "[METRICS] FPS: {:.1}, Frame time: {:.2}ms",
                    fps,
                    frame_time * 1000.0
                );
            }
        }

        // Check for selection state (poll every 100ms to avoid spam)
        let _has_active_selection = if self.last_selection_check.elapsed().as_millis() > 100 {
            let selection_info = ctx
                .with_plugin(|state: &mut LabelSelectionState| {
                    if state.has_selection() {
                        // Try to get the selected text from the state
                        // LabelSelectionState has the selection but we need to extract the text
                        Some(state.clone())
                    } else {
                        None
                    }
                })
                .unwrap_or(None);

            if selection_info.is_some() {
                println!("[SELECTION] Selection detected via LabelSelectionState");
                // Set a marker so the button shows up
                if self.selected_text.is_none() {
                    self.selected_text = Some("(selection active)".to_string());
                }
            }
            self.last_selection_check = std::time::Instant::now();
            selection_info.is_some()
        } else {
            false
        };

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Render markdown with egui_commonmark
                let _response = egui_commonmark::CommonMarkViewer::new().show(
                    ui,
                    &mut self.cache,
                    &self.markdown_source,
                );
            });
        });

        // Check for clipboard commands AFTER all panels to avoid deadlock
        let clipboard_text = ctx.output_mut(|o| {
            o.commands.iter().find_map(|cmd| {
                if let egui::output::OutputCommand::CopyText(text) = cmd {
                    Some(text.clone())
                } else {
                    None
                }
            })
        });

        if let Some(text) = clipboard_text {
            if !text.is_empty() {
                println!("[SELECTION] Copied text ({} chars): {:?}", text.len(), text);
                self.selected_text = Some(text);
            }
        }

        // Show comment dialog if active
        if self.show_comment_dialog {
            egui::Window::new("Add Comment")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label("Selected text:");
                    if let Some(ref text) = self.selected_text {
                        ui.monospace(text);
                    }
                    ui.separator();

                    ui.label("Comment:");
                    let response = ui.text_edit_multiline(&mut self.comment_input);

                    // Auto-focus on first frame
                    if response.has_focus() == false {
                        response.request_focus();
                    }

                    ui.horizontal(|ui| {
                        if ui.button("Submit").clicked()
                            || ui.input(|i| i.key_pressed(egui::Key::Enter) && i.modifiers.command)
                        {
                            if !self.comment_input.is_empty() {
                                // Get the actual selected text from clipboard/output commands
                                let actual_text = ctx.output_mut(|o| {
                                    o.commands.iter().find_map(|cmd| {
                                        if let egui::output::OutputCommand::CopyText(text) = cmd {
                                            Some(text.clone())
                                        } else {
                                            None
                                        }
                                    })
                                });

                                let text_to_use =
                                    actual_text.or_else(|| self.selected_text.clone());

                                if let Some(ref text) = text_to_use {
                                    // Find where this text appears in the source markdown
                                    let line_range = if let Some(byte_pos) =
                                        self.markdown_source.find(text)
                                    {
                                        // Found the text, look up its byte range
                                        let byte_range = byte_pos..(byte_pos + text.len());
                                        println!(
                                            "[DEBUG] Found text at byte range {:?}",
                                            byte_range
                                        );
                                        println!(
                                            "[DEBUG] Cache contains {} range entries",
                                            self.cache.line_mappings.ranges.len()
                                        );
                                        for (range, lines) in
                                            self.cache.line_mappings.ranges.iter().take(5)
                                        {
                                            println!(
                                                "[DEBUG]   Range {:?} -> lines {:?}",
                                                range, lines
                                            );
                                        }
                                        self.cache.get_line_numbers_for_range(&byte_range)
                                    } else {
                                        println!("[DEBUG] Text not found in source: {:?}", text);
                                        None
                                    };

                                    println!(
                                        "[COMMENT] Added comment: {:?} at lines {:?}",
                                        self.comment_input, line_range
                                    );
                                    self.comments.push(Comment {
                                        selected_text: text.clone(),
                                        comment: self.comment_input.clone(),
                                        line_range,
                                    });
                                }
                                self.comment_input.clear();
                                self.show_comment_dialog = false;
                                self.selected_text = None;
                            }
                        }

                        if ui.button("Cancel").clicked()
                            || ui.input(|i| i.key_pressed(egui::Key::Escape))
                        {
                            self.show_comment_dialog = false;
                            self.comment_input.clear();
                        }
                    });
                });
        }

        // Show selection info and comments in a side panel
        egui::SidePanel::right("selection_panel")
            .default_width(250.0)
            .show(ctx, |ui| {
                ui.heading("Selection");
                ui.separator();

                if let Some(ref text) = self.selected_text {
                    ui.label("Selected text:");
                    ui.monospace(text);
                    ui.separator();
                    ui.label(format!("Length: {} chars", text.len()));

                    if ui.button("Add Comment").clicked() {
                        // When "Add Comment" is clicked, try to capture the selected text
                        // by checking if there's a CopyText command pending
                        let captured_text = ctx.output_mut(|o| {
                            o.commands.iter().find_map(|cmd| {
                                if let egui::output::OutputCommand::CopyText(text) = cmd {
                                    Some(text.clone())
                                } else {
                                    None
                                }
                            })
                        });

                        if let Some(text) = captured_text {
                            println!("[CAPTURE] Got text from CopyText command: {:?}", text);
                            self.selected_text = Some(text);
                        }

                        self.show_comment_dialog = true;
                    }
                } else {
                    ui.label("No text selected");
                    ui.separator();
                    ui.label("Select text with mouse + Cmd+C");
                }

                ui.separator();
                ui.heading("Comments");
                ui.separator();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (i, comment) in self.comments.iter().enumerate() {
                        ui.group(|ui| {
                            if let Some((start, end)) = comment.line_range {
                                if start == end {
                                    ui.label(format!("Comment {} (line {})", i + 1, start));
                                } else {
                                    ui.label(format!(
                                        "Comment {} (lines {}-{})",
                                        i + 1,
                                        start,
                                        end
                                    ));
                                }
                            } else {
                                ui.label(format!("Comment {} (position unknown)", i + 1));
                            }
                            ui.monospace(&comment.selected_text);
                            ui.separator();
                            ui.label(&comment.comment);
                        });
                        ui.add_space(5.0);
                    }
                });
            });

        ctx.request_repaint();
    }
}
