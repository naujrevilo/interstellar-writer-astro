use eframe::egui;
use crate::models::FileEntry;
use crate::ui::{components, theme};

/// Dashboard action results.
pub enum DashboardAction {
    None,
    EditFile(String),
    DeleteFile(String),
    NewFile,
    Refresh,
}

/// Render the dashboard with file cards — clean, warm aesthetic.
pub fn show_dashboard(
    ui: &mut egui::Ui,
    collection_name: &str,
    files: &[FileEntry],
) -> DashboardAction {
    let mut action = DashboardAction::None;
    let tokens = theme::tokens_from_ui(ui);

    ui.vertical(|ui| {
        ui.add_space(24.0);

        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(collection_name)
                    .font(egui::FontId::proportional(28.0))
                    .color(tokens.text_primary),
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if components::primary_button(ui, "+ New").clicked() {
                    action = DashboardAction::NewFile;
                }
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("Refresh").color(tokens.text_muted),
                    ).fill(egui::Color32::TRANSPARENT))
                    .clicked()
                {
                    action = DashboardAction::Refresh;
                }
            });
        });

        ui.add_space(8.0);
        // Subtle divider line
        let rect = ui.available_rect_before_wrap();
        let y = rect.top();
        ui.painter().line_segment(
            [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
            egui::Stroke::new(0.5, tokens.divider),
        );
        ui.add_space(16.0);

        egui::ScrollArea::vertical()
            .id_salt("dashboard_scroll")
            .show(ui, |ui| {
                if files.is_empty() {
                    ui.add_space(60.0);
                    ui.vertical_centered(|ui| {
                        ui.label(
                            egui::RichText::new("No files in this collection yet.")
                                .color(tokens.text_muted)
                                .font(egui::FontId::proportional(16.0)),
                        );
                    });
                } else {
                    let available_width = ui.available_width();
                    let card_width = 300.0;
                    let spacing = 16.0;
                    let columns =
                        (available_width / (card_width + spacing)).floor().max(1.0) as usize;

                    egui::Grid::new("dashboard_grid")
                        .spacing([spacing, spacing])
                        .show(ui, |ui| {
                            let mut count = 0;
                            for entry in files {
                                let card_action = render_card(ui, entry, card_width);
                                match card_action {
                                    CardAction::Edit => {
                                        action =
                                            DashboardAction::EditFile(entry.name.clone())
                                    }
                                    CardAction::Delete => {
                                        action =
                                            DashboardAction::DeleteFile(entry.name.clone())
                                    }
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

    let card_frame = egui::Frame::none()
        .fill(tokens.subtle_bg)
        .rounding(tokens.radius_md)
        .inner_margin(egui::Margin::same(16.0))
        .stroke(egui::Stroke::new(0.5, tokens.border_subtle));

    card_frame.show(ui, |ui| {
        ui.set_width(card_width - 32.0); // account for padding
        ui.set_min_height(120.0);

        ui.vertical(|ui| {
            // Title in Newsreader at larger size
            ui.label(
                egui::RichText::new(&entry.title)
                    .font(egui::FontId::proportional(18.0))
                    .color(tokens.text_primary),
            );
            ui.add_space(4.0);

            // Date — muted
            ui.label(
                egui::RichText::new(&entry.date)
                    .font(egui::FontId::proportional(12.0))
                    .color(tokens.text_muted),
            );

            ui.add_space(12.0);

            // Bottom row: status text + actions
            ui.horizontal(|ui| {
                // Text-only status indicator
                if entry.draft {
                    ui.label(
                        egui::RichText::new("Draft")
                            .font(egui::FontId::proportional(11.0))
                            .color(tokens.status_draft),
                    );
                } else {
                    ui.label(
                        egui::RichText::new("Published")
                            .font(egui::FontId::proportional(11.0))
                            .color(tokens.status_published),
                    );
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new("Edit")
                                    .color(tokens.brand_primary)
                                    .font(egui::FontId::proportional(12.0)),
                            )
                            .fill(egui::Color32::TRANSPARENT),
                        )
                        .clicked()
                    {
                        action = CardAction::Edit;
                    }
                    if entry.draft {
                        if ui
                            .add(
                                egui::Button::new(
                                    egui::RichText::new("Delete")
                                        .color(tokens.brand_danger)
                                        .font(egui::FontId::proportional(12.0)),
                                )
                                .fill(egui::Color32::TRANSPARENT),
                            )
                            .on_hover_text("Delete draft")
                            .clicked()
                        {
                            action = CardAction::Delete;
                        }
                    }
                });
            });
        });
    });

    action
}
