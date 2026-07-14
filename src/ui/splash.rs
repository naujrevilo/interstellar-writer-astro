//! Splash screen — minimal warm aesthetic.

use eframe::egui;
use crate::ui::theme;

/// Show the splash screen. Returns `true` when done.
pub fn show_splash(
    ctx: &egui::Context,
    splash_start_time: &mut Option<std::time::Instant>,
) -> bool {
    if splash_start_time.is_none() {
        *splash_start_time = Some(std::time::Instant::now());
    }

    if let Some(start_time) = splash_start_time {
        if start_time.elapsed().as_secs_f32() < 2.5 {
            let tokens = theme::UiTokens::for_mode(ctx.style().visuals.dark_mode);

            egui::CentralPanel::default()
                .frame(egui::Frame::none().fill(tokens.panel_bg))
                .show(ctx, |ui| {
                    let total_h = ui.available_height();
                    ui.vertical_centered(|ui| {
                        ui.add_space(total_h * 0.30);

                        // Logo — compact
                        ui.add(
                            egui::Image::new(egui::include_image!("../../logo.svg"))
                                .max_height(80.0),
                        );
                        ui.add_space(20.0);

                        // Title in Newsreader (Proportional) — editorial feel
                        ui.label(
                            egui::RichText::new("Interstellar Writer")
                                .font(egui::FontId::proportional(36.0))
                                .italics()
                                .color(tokens.text_primary),
                        );
                        ui.add_space(6.0);

                        // Subtitle — muted
                        ui.label(
                            egui::RichText::new("A writing companion for Astro content")
                                .font(egui::FontId::proportional(14.0))
                                .color(tokens.text_muted),
                        );
                        ui.add_space(30.0);

                        // Minimal spinner
                        ui.add(egui::Spinner::new().size(18.0).color(tokens.text_faint));

                        ctx.request_repaint();
                    });
                });
            return false;
        }
    }

    true
}
