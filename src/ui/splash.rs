//! Pantalla de inicio (Splash Screen).

use eframe::egui;

/// Muestra la pantalla de splash durante el inicio de la aplicación.
/// Retorna `true` si el splash ha terminado.
pub fn show_splash(
    ctx: &egui::Context, 
    splash_start_time: &mut Option<std::time::Instant>,
) -> bool {
    // Si es el primer frame, inicializamos el temporizador
    if splash_start_time.is_none() {
        *splash_start_time = Some(std::time::Instant::now());
    }

    if let Some(start_time) = splash_start_time {
        // Mostrar splash durante 3 segundos
        if start_time.elapsed().as_secs_f32() < 3.0 {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(ui.available_height() * 0.2);
                        ui.add(egui::Image::new(egui::include_image!("../../logo.svg")).max_height(200.0));
                        ui.add_space(30.0);
                        ui.heading(egui::RichText::new("Interstellar Writer for MD/MDX").size(48.0).strong());
                        ui.add_space(10.0);
                        ui.label(egui::RichText::new("Tu compañero galáctico para MD/MDX").size(20.0).italics());
                        ui.add_space(40.0);
                        ui.add(egui::Spinner::new().size(30.0));
                        ui.add_space(15.0);
                        ui.label(egui::RichText::new("Cargando motores de curvatura...").color(egui::Color32::GRAY));
                        
                        // Forzar repintado para la animación
                        ctx.request_repaint();
                    });
                });
            });
            return false; // Splash no terminado
        }
    }
    
    true // Splash terminado
}
