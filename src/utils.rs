//! Funciones de utilidad general.

use eframe::egui;

/// Carga el icono de la aplicación desde el archivo PNG embebido.
pub fn load_icon() -> egui::IconData {
    // Cargar el icono PNG en tiempo de compilación
    let icon_bytes = include_bytes!("../favicon.png");

    // Decodificar la imagen PNG
    let image = image::load_from_memory(icon_bytes)
        .expect("Error al cargar favicon.png")
        .into_rgba8();

    let (width, height) = image.dimensions();
    let rgba = image.into_raw();

    egui::IconData {
        rgba,
        width,
        height,
    }
}

/// Aplica el estilo visual de la aplicación (inspirado en VS Code).
pub fn apply_visuals(ctx: &egui::Context, dark_mode: bool) {
    let mut visuals = if dark_mode {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };
    
    // Ajustes de redondeo para una estética más profesional
    visuals.window_rounding = 4.0.into();
    visuals.widgets.active.rounding = 4.0.into();
    visuals.widgets.inactive.rounding = 4.0.into();
    visuals.widgets.hovered.rounding = 4.0.into();
    
    // Color azul característico de VS Code para la selección
    visuals.selection.bg_fill = egui::Color32::from_rgb(0, 122, 204); 
    
    if dark_mode {
        visuals.panel_fill = egui::Color32::from_rgb(30, 30, 30);
        visuals.window_fill = egui::Color32::from_rgb(37, 37, 38);
        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(30, 30, 30);
        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(45, 45, 45);
    } else {
        visuals.panel_fill = egui::Color32::from_rgb(243, 243, 243);
        visuals.window_fill = egui::Color32::WHITE;
    }

    ctx.set_visuals(visuals);
}

/// Abre un diálogo para seleccionar un archivo de imagen.
pub fn pick_image_file() -> Option<std::path::PathBuf> {
    rfd::FileDialog::new()
        .add_filter("Imágenes", &["png", "jpg", "jpeg", "webp", "gif", "svg"])
        .pick_file()
}

/// Abre un diálogo para seleccionar una carpeta.
pub fn pick_folder() -> Option<std::path::PathBuf> {
    rfd::FileDialog::new().pick_folder()
}
