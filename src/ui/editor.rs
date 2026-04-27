//! Editor de texto principal.

use eframe::egui;

/// Renderiza el editor de texto con resaltado de sintaxis.
pub fn show_editor(
    ui: &mut egui::Ui,
    body: &mut String,
    selection: &mut Option<(usize, usize)>,
    pending_selection: &mut Option<(usize, usize)>,
) -> bool {
    let output = ui.add_sized(
        [ui.available_width(), ui.available_height().max(600.0)],
        egui::TextEdit::multiline(body)
            .font(egui::TextStyle::Monospace)
            .desired_width(f32::INFINITY)
            .layouter(&mut |ui, text, wrap_width| {
                let mut layout_job = egui::text::LayoutJob::default();
                layout_job.wrap.max_width = wrap_width;

                let font_id = egui::TextStyle::Monospace.resolve(ui.style());
                let default_color = ui.visuals().text_color();

                for line in text.lines() {
                    let dark_mode = ui.visuals().dark_mode;
                    let format = if line.starts_with("#") {
                        egui::TextFormat {
                            font_id: egui::FontId::proportional(font_id.size * 1.1),
                            // Dark: azul claro brillante visible sobre fondo oscuro y sobre selección.
                            // Light: azul más saturado para contraste sobre fondo blanco.
                            color: if dark_mode {
                                egui::Color32::from_rgb(100, 180, 255)
                            } else {
                                egui::Color32::from_rgb(0, 80, 180)
                            },
                            ..Default::default()
                        }
                    } else if line.starts_with("import") || line.starts_with("---") {
                        egui::TextFormat {
                            color: egui::Color32::from_gray(120),
                            font_id: font_id.clone(),
                            ..Default::default()
                        }
                    } else if line.trim().starts_with("<Notice") {
                        let color = if line.contains("type=\"danger\"") {
                            egui::Color32::from_rgb(231, 76, 60)
                        } else if line.contains("type=\"warning\"") {
                            egui::Color32::from_rgb(241, 196, 15)
                        } else if line.contains("type=\"success\"") {
                            egui::Color32::from_rgb(46, 204, 113)
                        } else if line.contains("type=\"tip\"") {
                            egui::Color32::from_rgb(155, 89, 182)
                        } else {
                            egui::Color32::from_rgb(52, 152, 219)
                        };
                        egui::TextFormat {
                            color,
                            font_id: font_id.clone(),
                            ..Default::default()
                        }
                    } else if line.trim().starts_with("<") {
                        egui::TextFormat {
                            color: egui::Color32::from_rgb(155, 89, 182),
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
                    layout_job.append("\n", 0.0, egui::TextFormat::default());
                }

                ui.fonts(|f| f.layout_job(layout_job))
            })
            .lock_focus(true),
    );

    // Si tenemos una selección pendiente (programática), la aplicamos
    if let Some((start, end)) = pending_selection.take() {
        let mut state =
            egui::text_edit::TextEditState::load(ui.ctx(), output.id).unwrap_or_default();
        let c_start = egui::text::CCursor::new(start);
        let c_end = egui::text::CCursor::new(end);
        state
            .cursor
            .set_char_range(Some(egui::text::CCursorRange::two(c_start, c_end)));
        state.store(ui.ctx(), output.id);
    }

    // Sincronizar selection con el estado real del editor
    if let Some(state) = egui::TextEdit::load_state(ui.ctx(), output.id) {
        if let Some(range) = state.cursor.char_range() {
            *selection = Some((range.primary.index, range.secondary.index));
        }
    }
    output.changed()
}
