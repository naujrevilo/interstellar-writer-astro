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
#[allow(dead_code)]
pub fn apply_visuals(ctx: &egui::Context, dark_mode: bool) {
    crate::ui::theme::apply_theme(ctx, dark_mode);
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
