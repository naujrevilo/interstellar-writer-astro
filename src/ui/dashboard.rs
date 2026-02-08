use eframe::egui;
use crate::models::FileEntry;

/// Resultado de las acciones del dashboard.
pub enum DashboardAction {
    None,
    EditFile(String),
    DeleteFile(String),
    NewFile,
    Refresh,
}

/// Renderiza el dashboard con las tarjetas de publicaciones.
pub fn show_dashboard(
    ui: &mut egui::Ui,
    collection_name: &str,
    files: &[FileEntry],
) -> DashboardAction {
    let mut action = DashboardAction::None;

    ui.vertical(|ui| {
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.heading(egui::RichText::new(format!("Dashboard: {}", collection_name)).size(24.0));

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let btn_new = egui::Button::new(egui::RichText::new("➕ Nueva Publicación").color(egui::Color32::WHITE))
                    .fill(egui::Color32::from_rgb(0, 122, 204))
                    .rounding(4.0);
                if ui.add(btn_new).clicked() {
                    action = DashboardAction::NewFile;
                }
                if ui.button("🔄 Refrescar").clicked() {
                    action = DashboardAction::Refresh;
                }
            });
        });

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);

        egui::ScrollArea::vertical().id_salt("dashboard_scroll").show(ui, |ui| {
            if files.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.label("No hay archivos en esta colección.");
                });
            } else {
                let available_width = ui.available_width();
                let card_width = 280.0;
                let spacing = 20.0;
                let columns = (available_width / (card_width + spacing)).floor().max(1.0) as usize;

                egui::Grid::new("dashboard_grid")
                    .spacing([spacing, spacing])
                    .show(ui, |ui| {
                        let mut count = 0;
                        for entry in files {
                            let card_action = render_card(ui, entry, card_width);
                            match card_action {
                                CardAction::Edit => action = DashboardAction::EditFile(entry.name.clone()),
                                CardAction::Delete => action = DashboardAction::DeleteFile(entry.name.clone()),
                                CardAction::None => {}
                            }
                            count += 1;
                            if count % columns == 0 {
                                ui.end_row();
                            }
                        }
                    });
            }
        });
    });

    action
}

enum CardAction {
    None,
    Edit,
    Delete,
}

fn render_card(ui: &mut egui::Ui, entry: &FileEntry, card_width: f32) -> CardAction {
    let mut action = CardAction::None;

    ui.group(|ui| {
        ui.set_width(card_width);
        ui.set_height(180.0);

        ui.vertical(|ui| {
            // Header color/area
            let (header_color, icon) = if entry.draft {
                (egui::Color32::from_rgb(60, 60, 60), "📝")
            } else {
                (egui::Color32::from_rgb(0, 122, 204), "🚀")
            };

            let (rect, _) = ui.allocate_at_least(egui::vec2(card_width, 80.0), egui::Sense::hover());
            ui.painter().rect_filled(rect, 4.0, header_color);
            // Dibujar icono
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                icon,
                egui::FontId::proportional(32.0),
                egui::Color32::WHITE,
            );

            ui.add_space(5.0);
            ui.label(egui::RichText::new(&entry.title).strong().size(16.0));
            ui.label(egui::RichText::new(&entry.date).small().color(egui::Color32::GRAY));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    if entry.draft {
                        ui.label(egui::RichText::new("BORRADOR").small().strong().color(egui::Color32::from_rgb(241, 196, 15)));
                    } else {
                        ui.label(egui::RichText::new("PUBLICADO").small().strong().color(egui::Color32::from_rgb(46, 204, 113)));
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let btn_edit = egui::Button::new("Editar").rounding(4.0);
                        if ui.add(btn_edit).clicked() {
                            action = CardAction::Edit;
                        }
                        if entry.draft {
                            if ui.button("🗑").on_hover_text("Eliminar borrador").clicked() {
                                action = CardAction::Delete;
                            }
                        }
                    });
                });
            });
        });
    });

    action
}
