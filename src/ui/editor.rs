//! Main text editor — clean centered writing area.

use eframe::egui;
use crate::ui::theme;

/// Render the text editor with syntax highlighting and centered layout.
pub fn show_editor(
    ui: &mut egui::Ui,
    body: &mut String,
    selection: &mut Option<(usize, usize)>,
    pending_selection: &mut Option<(usize, usize)>,
) -> bool {
    let tokens = theme::tokens_from_ui(ui);

    // Center the text column at ~640px max width
    let available = ui.available_width();
    let max_col: f32 = 640.0;
    let col_width = max_col.min((available - 40.0).max(0.0));

    // Add top padding for breathing room
    ui.add_space(56.0);

    let output = ui
        .with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            egui::TextEdit::multiline(body)
                .font(egui::TextStyle::Body)
                .desired_width(col_width)
                .layouter(&mut |ui, text, wrap_width| {
                    let mut layout_job = egui::text::LayoutJob::default();
                    layout_job.wrap.max_width = wrap_width;

                    let font_id = egui::TextStyle::Body.resolve(ui.style());
                    let default_color = tokens.text_body;

                    let parts: Vec<&str> = text.split('\n').collect();
                    for (i, part) in parts.iter().enumerate() {
                        let line = part.trim_end_matches('\r');
                        let dark_mode = ui.visuals().dark_mode;
                        let format = if line.starts_with("#") {
                            egui::TextFormat {
                                font_id: egui::FontId::proportional(font_id.size * 1.15),
                                color: tokens.text_primary,
                                ..Default::default()
                            }
                        } else if line.starts_with("import") || line.starts_with("---") {
                            egui::TextFormat {
                                color: tokens.text_faint,
                                font_id: font_id.clone(),
                                ..Default::default()
                            }
                        } else if line.trim().starts_with("<Notice") {
                            let color = if line.contains("type=\"danger\"") {
                                tokens.brand_danger
                            } else if line.contains("type=\"warning\"") {
                                tokens.brand_warning
                            } else if line.contains("type=\"success\"") {
                                tokens.brand_success
                            } else if line.contains("type=\"tip\"") {
                                egui::Color32::from_rgb(155, 89, 182)
                            } else {
                                tokens.brand_primary
                            };
                            egui::TextFormat {
                                color,
                                font_id: font_id.clone(),
                                ..Default::default()
                            }
                        } else if line.trim().starts_with("<") {
                            egui::TextFormat {
                                color: if dark_mode {
                                    egui::Color32::from_rgb(154, 149, 144)
                                } else {
                                    egui::Color32::from_rgb(74, 69, 64)
                                },
                                font_id: font_id.clone(),
                                ..Default::default()
                            }
                        } else {
                            egui::TextFormat {
                                color: default_color,
                                font_id: font_id.clone(),
                                ..Default::default()
                            }
                        };
                        layout_job.append(line, 0.0, format);
                        if i < parts.len() - 1 {
                            layout_job.append("\n", 0.0, egui::TextFormat::default());
                        }
                    }

                    ui.fonts(|f| f.layout_job(layout_job))
                })
                .lock_focus(true)
                .show(ui)
        })
        .inner;

    // Apply pending programmatic selection
    if let Some((start, end)) = pending_selection.take() {
        let mut state = output.state.clone();
        let c_start = egui::text::CCursor::new(start);
        let c_end = egui::text::CCursor::new(end);
        state
            .cursor
            .set_char_range(Some(egui::text::CCursorRange::two(c_start, c_end)));
        state.store(ui.ctx(), output.response.id);
        *selection = Some((start, end));
        return output.response.changed();
    }

    // Sync selection with actual editor state
    if let Some(cursor_range) = output.state.cursor.char_range() {
        *selection = Some((cursor_range.primary.index, cursor_range.secondary.index));
    }
    output.response.changed()
}
