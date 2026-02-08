//! Diálogos y ventanas modales.

use eframe::egui;
use crate::models::{Config, FieldType};

/// Muestra el diálogo "Acerca de".
pub fn show_about_dialog(ctx: &egui::Context, showing: &mut bool) {
    egui::Window::new("\u{2139} Acerca de Interstellar Writer for MD/MDX")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add(egui::Image::new(egui::include_image!("../../logo.svg")).max_width(100.0));
                ui.add_space(10.0);
                ui.heading("Interstellar Writer");
                ui.label("v1.0.0");
                ui.add_space(10.0);
                ui.label("Tu compañero galáctico para Astro & MDX.");
                ui.add_space(10.0);
                ui.label(egui::RichText::new("Desarrollado por Juan Oliver").strong());
                ui.label("con Rust y egui.");
                ui.add_space(20.0);
                if ui.button("Cerrar").clicked() {
                    *showing = false;
                }
            });
        });
}

/// Muestra el diálogo del manual de usuario.
pub fn show_manual_dialog(
    ctx: &egui::Context, 
    showing: &mut bool, 
    manual_content: &str,
    commonmark_cache: &mut egui_commonmark::CommonMarkCache,
) {
    egui::Window::new("📖 Manual de Usuario")
        .default_width(800.0)
        .default_height(600.0)
        .open(showing)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui_commonmark::CommonMarkViewer::new()
                    .show(ui, commonmark_cache, manual_content);
            });
        });
}

/// Muestra el diálogo de confirmación de eliminación.
/// Retorna `true` si el usuario confirmó la eliminación.
pub fn show_delete_confirm_dialog(ctx: &egui::Context, showing: &mut bool) -> bool {
    let mut confirmed = false;
    
    egui::Window::new("\u{26A0} Confirmar Eliminación")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            ui.label("¿Estás seguro de que quieres eliminar este archivo?");
            ui.label("Esta acción no se puede deshacer.");
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                let btn = egui::Button::new(egui::RichText::new("Eliminar definitivamente").color(egui::Color32::WHITE))
                    .fill(egui::Color32::from_rgb(192, 57, 43));
                if ui.add(btn).clicked() {
                    confirmed = true;
                    *showing = false;
                }
                if ui.button("Cancelar").clicked() {
                    *showing = false;
                }
            });
        });
    
    confirmed
}

/// Muestra el diálogo de confirmación de commit/push.
/// Retorna `Some(commit_msg)` si el usuario confirmó.
pub fn show_commit_confirm_dialog(
    ctx: &egui::Context, 
    showing: &mut bool,
    commit_message: &mut String,
) -> bool {
    let mut confirmed = false;
    
    egui::Window::new("\u{1F680} Confirmar Sincronización")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            ui.label("Se guardarán los cambios locales y se enviarán a GitHub.");
            ui.add_space(5.0);
            ui.label("Mensaje de commit:");
            ui.text_edit_singleline(commit_message);
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                let btn = egui::Button::new(egui::RichText::new("Confirmar y Subir").color(egui::Color32::WHITE))
                    .fill(egui::Color32::from_rgb(41, 128, 185));
                if ui.add(btn).clicked() {
                    confirmed = true;
                    *showing = false;
                }
                if ui.button("Cancelar").clicked() {
                    *showing = false;
                }
            });
        });
    
    confirmed
}

/// Muestra el diálogo de creación de nuevo archivo.
/// Retorna `Some(filename)` si el usuario quiere crear el archivo.
pub fn show_new_file_dialog(
    ctx: &egui::Context,
    showing: &mut bool,
    new_file_name: &mut String,
) -> Option<String> {
    let mut result = None;
    
    egui::Window::new("Crear Nueva Publicación")
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Nombre:");
                ui.text_edit_singleline(new_file_name).on_hover_text("Puedes usar subcarpetas, ej: docs/mi-guia.md");
            });
            ui.horizontal(|ui| {
                if ui.button("Crear").clicked() {
                    result = Some(new_file_name.clone());
                    *showing = false;
                }
                if ui.button("Cancelar").clicked() {
                    *showing = false;
                }
            });
        });
    
    result
}

/// Muestra el indicador de sincronización en progreso.
pub fn show_syncing_indicator(ctx: &egui::Context) {
    egui::Window::new("⏳ Sincronizando")
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add(egui::Spinner::new());
                ui.add_space(10.0);
                ui.label("Subiendo cambios a GitHub...");
                ui.label("Por favor, espera.");
            });
        });
    ctx.request_repaint();
}

/// Muestra el diálogo de configuración avanzada.
pub fn show_config_dialog(
    ctx: &egui::Context,
    showing: &mut bool,
    config: &mut Config,
    on_repo_change: impl FnOnce() -> Option<std::path::PathBuf>,
) -> bool {
    let mut should_refresh = false;
    
    egui::Window::new("Configuración Avanzada")
        .default_width(500.0)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("General");
                ui.label("Repositorio Local:");
                ui.horizontal(|ui| {
                    ui.label(format!("{:?}", config.repo_path));
                    if ui.button("Cambiar").clicked() {
                        if let Some(path) = on_repo_change() {
                            config.repo_path = Some(path);
                            should_refresh = true;
                        }
                    }
                });
                
                ui.add_space(5.0);
                ui.label("GitHub Personal Access Token:");
                let mut token = config.github_token.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut token).changed() {
                    config.github_token = Some(token);
                }
                
                ui.separator();
                ui.heading("Rutas (Relativas al repositorio)");
                ui.horizontal(|ui| {
                    ui.label("Carpeta de Contenido:");
                    ui.text_edit_singleline(&mut config.content_dir);
                });
                ui.horizontal(|ui| {
                    ui.label("Carpeta de Assets (Imágenes):");
                    ui.text_edit_singleline(&mut config.assets_dir);
                });
                
                ui.separator();
                ui.heading("Estructura de Colecciones");
                
                let mut coll_to_remove = None;
                for (i, coll) in config.collections.iter_mut().enumerate() {
                    egui::CollapsingHeader::new(format!("Colección: {}", coll.name))
                        .default_open(false)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label("Nombre:");
                                ui.text_edit_singleline(&mut coll.name);
                                if ui.button("🗑 Eliminar Colección").clicked() {
                                    coll_to_remove = Some(i);
                                }
                            });
                            
                            ui.label("Campos:");
                            let mut field_to_remove = None;
                            for (j, field) in coll.fields.iter_mut().enumerate() {
                                ui.group(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.label("Nombre:");
                                        ui.text_edit_singleline(&mut field.name);
                                        
                                        egui::ComboBox::new(format!("type_{}_{}", i, j), "")
                                            .selected_text(format!("{:?}", field.field_type))
                                            .show_ui(ui, |ui| {
                                                ui.selectable_value(&mut field.field_type, FieldType::String, "String");
                                                ui.selectable_value(&mut field.field_type, FieldType::Boolean, "Boolean");
                                                ui.selectable_value(&mut field.field_type, FieldType::Date, "Date");
                                                ui.selectable_value(&mut field.field_type, FieldType::Image, "Image");
                                                ui.selectable_value(&mut field.field_type, FieldType::List, "List");
                                                ui.selectable_value(&mut field.field_type, FieldType::Categories, "Categories (Enum)");
                                                ui.selectable_value(&mut field.field_type, FieldType::Number, "Number");
                                            });
                                        
                                        if ui.button("❌").clicked() {
                                            field_to_remove = Some(j);
                                        }
                                    });
                                    
                                    ui.horizontal(|ui| {
                                        ui.label("Valor por defecto:");
                                        ui.text_edit_singleline(&mut field.default_value);
                                    });
                                    
                                    if field.field_type == FieldType::Categories {
                                        ui.label("Opciones (separadas por coma):");
                                        let mut opts_str = field.options.as_ref().map(|o| o.join(", ")).unwrap_or_default();
                                        if ui.text_edit_singleline(&mut opts_str).changed() {
                                            field.options = Some(opts_str.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect());
                                        }
                                    }
                                });
                            }
                            
                            if let Some(j) = field_to_remove {
                                coll.fields.remove(j);
                            }
                            
                            if ui.button("➕ Añadir Campo").clicked() {
                                coll.fields.push(crate::models::FieldDef {
                                    name: "nuevo_campo".to_string(),
                                    field_type: FieldType::String,
                                    default_value: "".to_string(),
                                    options: None,
                                });
                            }
                        });
                }
                
                if let Some(i) = coll_to_remove {
                    config.collections.remove(i);
                }
                
                if ui.button("➕ Añadir Nueva Colección").clicked() {
                    config.collections.push(crate::models::CollectionDef {
                        name: "nueva_coleccion".to_string(),
                        fields: vec![],
                    });
                }
            });
            
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("💾 Guardar y Cerrar").clicked() {
                    config.save();
                    should_refresh = true;
                    *showing = false;
                }
                if ui.button("Cancelar").clicked() {
                    *config = Config::load(); // Revertir cambios
                    *showing = false;
                }
            });
        });
    
    should_refresh
}
