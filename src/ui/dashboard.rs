use eframe::egui;
use crate::models::FileEntry;
use crate::ui::{components, theme};

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
    let tokens = theme::tokens_from_ui(ui);

    ui.vertical(|ui| {
        ui.add_space(tokens.spacing_md);

        ui.horizontal(|ui| {
            ui.heading(egui::RichText::new(format!("Dashboard: {}", collection_name)).size(24.0));

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if components::primary_button(ui, "➕ Nueva Publicación").clicked() {
                    action = DashboardAction::NewFile;
                }
                if ui.button("🔄 Refrescar").clicked() {
                    action = DashboardAction::Refresh;
                }
            });
        });

        ui.add_space(tokens.spacing_md);
        ui.separator();
        ui.add_space(tokens.spacing_md);

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
    let tokens = theme::tokens_from_ui(ui);

    ui.group(|ui| {
        ui.set_width(card_width);
        ui.set_height(180.0);

        ui.vertical(|ui| {
            // Header color/area
            let (header_color, icon) = if entry.draft {
                (tokens.subtle_bg, "📝")
            } else {
                (tokens.brand_primary, "🚀")
            };

            let (rect, _) = ui.allocate_at_least(egui::vec2(card_width, 80.0), egui::Sense::hover());
            ui.painter().rect_filled(rect, tokens.radius_sm, header_color);
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
            ui.label(egui::RichText::new(&entry.date).small().color(tokens.text_muted));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    if entry.draft {
                        components::status_badge(ui, "BORRADOR", tokens.brand_warning);
                    } else {
                        components::status_badge(ui, "PUBLICADO", tokens.brand_success);
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if components::primary_button(ui, "Editar").clicked() {
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
