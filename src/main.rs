#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // ocultar consola en release

//! Interstellar Writer Astro - Editor de contenidos para proyectos Astro/MDX.
//!
//! Este es el punto de entrada principal. La logica esta organizada en modulos:
//! - `models`: Estructuras de datos y configuracion
//! - `services`: Logica de negocio (archivos, Git, parseo)
//! - `ui`: Componentes de interfaz de usuario
//! - `utils`: Funciones auxiliares
//! - `app`: Estado principal de la aplicacion

mod models;
mod services;
mod ui;
mod utils;
mod app;

use eframe::egui;
use app::InterstellarApp;

fn main() -> eframe::Result {
    let icon_data = utils::load_icon();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_drag_and_drop(true)
            .with_icon(icon_data),
        ..Default::default()
    };

    eframe::run_native(
        "Interstellar Writer",
        options,
        Box::new(|cc| Ok(Box::new(InterstellarApp::new(cc)))),
    )
}
