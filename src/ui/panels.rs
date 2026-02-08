//! Paneles laterales (explorador de colecciones, archivos, metadatos).

use eframe::egui;
use rfd::FileDialog;
use crate::models::{CollectionDef, FieldType, FileEntry};
use crate::services::content::calculate_rel_path;

/// Renderiza el panel izquierdo con colecciones y archivos.
pub fn show_left_panel(
    ctx: &egui::Context,
    showing_collections: bool,
    showing_files: bool,
    collections: &[String],
    selected_collection: &mut Option<String>,
    files: &[FileEntry],
    selected_file: &mut Option<String>,
    mut on_refresh_collections: impl FnMut(),
    mut on_collection_selected: impl FnMut(&str),
    mut on_file_selected: impl FnMut(&str),
    mut on_new_file: impl FnMut(),
) {
    egui::SidePanel::left("left_explorer")
        .resizable(true)
        .default_width(250.0)
        .show_animated(ctx, showing_collections || showing_files, |ui| {
            if showing_collections {
                ui.horizontal(|ui| {
                    ui.heading("📂 Colecciones");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("🔄").clicked() { 
                            on_refresh_collections(); 
                        }
                    });
                });
                egui::ScrollArea::vertical().id_salt("col_scroll").show(ui, |ui| {
                    for collection in collections {
                        if ui.selectable_label(selected_collection.as_ref() == Some(collection), collection).clicked() {
                            *selected_collection = Some(collection.clone());
                            on_collection_selected(collection);
                        }
                    }
                });
                ui.separator();
            }

            if showing_files && selected_collection.is_some() {
                let coll = selected_collection.clone().unwrap();
                ui.horizontal(|ui| {
                    ui.heading(format!("📄 {}", coll));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("➕").clicked() { 
                            on_new_file(); 
                        }
                    });
                });
                egui::ScrollArea::vertical().id_salt("file_scroll").show(ui, |ui| {
                    for entry in files {
                        let label = if entry.draft { 
                            format!("📝 {}", entry.title) 
                        } else { 
                            format!("✅ {}", entry.title) 
                        };
                        if ui.selectable_label(selected_file.as_ref() == Some(&entry.name), label).clicked() {
                            *selected_file = Some(entry.name.clone());
                            on_file_selected(&entry.name);
                        }
                    }
                });
            }
        });
}

/// Renderiza el panel derecho con metadatos del archivo.
pub fn show_metadata_panel(
    ctx: &egui::Context,
    showing: bool,
    has_file: bool,
    frontmatter: &mut serde_yaml::Mapping,
    collection_def: Option<&CollectionDef>,
    config_content_dir: &str,
    config_assets_dir: &str,
    repo_path: Option<&std::path::PathBuf>,
    selected_collection: Option<&String>,
    selected_file: Option<&String>,
    new_tag_input: &mut String,
    on_fix_paths: impl FnOnce(),
    on_ensure_import: impl Fn(&str),
) {
    egui::SidePanel::right("right_metadata")
        .resizable(true)
        .default_width(300.0)
        .show_animated(ctx, showing && has_file, |ui| {
            ui.horizontal(|ui| {
                ui.heading("📝 Metadatos");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("🔧").on_hover_text("Recalcular rutas").clicked() { 
                        on_fix_paths(); 
                    }
                });
            });
            ui.separator();
            
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                    if let Some(def) = collection_def {
                        let cur_coll = selected_collection.cloned().unwrap_or_default();
                        let cur_file = selected_file.cloned().unwrap_or_default();

                        let mut handled_keys = std::collections::HashSet::new();
                        
                        // Grupo: General
                        ui.group(|ui| {
                            ui.label(egui::RichText::new("General").strong().size(16.0));
                            ui.add_space(5.0);
                            
                            for field_name in &["title", "description", "date", "publishDate", "draft"] {
                                if let Some(field) = def.fields.iter().find(|f| f.name == *field_name) {
                                    render_field(ui, field, frontmatter, &mut handled_keys);
                                }
                            }
                        });

                        ui.add_space(10.0);
                        
                        // Grupo: Taxonomía e Imágenes
                        ui.group(|ui| {
                            ui.label(egui::RichText::new("Taxonomía e Imágenes").strong().size(16.0));
                            ui.add_space(5.0);

                            for field in &def.fields {
                                let yaml_key = serde_yaml::Value::String(field.name.clone());
                                if handled_keys.contains(&yaml_key) { continue; }
                                
                                render_complex_field(
                                    ui, 
                                    field, 
                                    frontmatter, 
                                    &mut handled_keys,
                                    new_tag_input,
                                    repo_path,
                                    config_content_dir,
                                    config_assets_dir,
                                    &cur_coll,
                                    &cur_file,
                                );
                            }
                        });

                        ui.add_space(10.0);
                        
                        // Grupo: Componentes MDX
                        ui.group(|ui| {
                            ui.label(egui::RichText::new("Componentes MDX").strong().size(16.0));
                            ui.add_space(5.0);
                            
                            let file_name = cur_file.clone();
                            let coll = cur_coll.clone();
                            
                            let btn_notice = egui::Button::new("📦 Importar Notice").fill(egui::Color32::from_rgb(52, 152, 219));
                            if ui.add(btn_notice).on_hover_text("Añade el import de Notice.astro").clicked() {
                                let rel_path = calculate_rel_path(config_content_dir, &coll, &file_name, "src/components/Notice.astro");
                                let import_stmt = format!("import Notice from \"{}\";", rel_path);
                                on_ensure_import(&import_stmt);
                            }
                            
                            let btn_cta = egui::Button::new("📢 Importar CTABox").fill(egui::Color32::from_rgb(41, 128, 185));
                            if ui.add(btn_cta).on_hover_text("Añade el import de CTABox.astro").clicked() {
                                let rel_path = calculate_rel_path(config_content_dir, &coll, &file_name, "src/components/CTABox.astro");
                                let import_stmt = format!("import CTABox from \"{}\";", rel_path);
                                on_ensure_import(&import_stmt);
                            }
                            
                            let btn_img = egui::Button::new("🖼 Importar Image").fill(egui::Color32::from_rgb(39, 174, 96));
                            if ui.add(btn_img).on_hover_text("Añade el import de Image de astro:assets").clicked() {
                                on_ensure_import("import { Image } from \"astro:assets\";");
                            }
                        });
                    }
                });
            });
        });
}

fn render_field(
    ui: &mut egui::Ui, 
    field: &crate::models::FieldDef, 
    frontmatter: &mut serde_yaml::Mapping,
    handled_keys: &mut std::collections::HashSet<serde_yaml::Value>,
) {
    let yaml_key = serde_yaml::Value::String(field.name.clone());
    handled_keys.insert(yaml_key.clone());
    
    ui.label(egui::RichText::new(&field.name).strong());
    
    if let Some(value) = frontmatter.get_mut(&yaml_key) {
        match field.field_type {
            FieldType::String => {
                if let serde_yaml::Value::String(s) = value {
                    ui.add(egui::TextEdit::multiline(s).desired_rows(1).min_size(egui::vec2(ui.available_width(), 0.0)));
                }
            },
            FieldType::Boolean => {
                if let serde_yaml::Value::Bool(b) = value {
                    ui.checkbox(b, "");
                }
            },
            FieldType::Date => {
                if let serde_yaml::Value::String(s) = value {
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(s);
                        if ui.button("📅").clicked() {
                            *s = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S.000Z").to_string();
                        }
                    });
                }
            },
            _ => {}
        }
    }
    ui.add_space(4.0);
}

fn render_complex_field(
    ui: &mut egui::Ui,
    field: &crate::models::FieldDef,
    frontmatter: &mut serde_yaml::Mapping,
    handled_keys: &mut std::collections::HashSet<serde_yaml::Value>,
    new_tag_input: &mut String,
    repo_path: Option<&std::path::PathBuf>,
    content_dir: &str,
    assets_dir: &str,
    collection: &str,
    file: &str,
) {
    let yaml_key = serde_yaml::Value::String(field.name.clone());
    handled_keys.insert(yaml_key.clone());
    
    ui.label(egui::RichText::new(&field.name).strong());
    
    if let Some(value) = frontmatter.get_mut(&yaml_key) {
        match field.field_type {
            FieldType::Image => {
                ui.horizontal(|ui| {
                    let mut s = match value {
                        serde_yaml::Value::String(sv) => sv.clone(),
                        _ => String::new(),
                    };
                    if ui.add(egui::TextEdit::singleline(&mut s)).changed() {
                        *value = serde_yaml::Value::String(s);
                    }
                    if ui.button("🖼").clicked() {
                        if let Some(repo) = repo_path {
                            if let Some(path) = FileDialog::new()
                                .add_filter("Imágenes", &["png", "jpg", "jpeg", "webp", "gif", "svg"])
                                .pick_file() 
                            {
                                if let Some(repo_rel_path) = crate::services::files::copy_image_to_assets(repo, assets_dir, &path) {
                                    *value = serde_yaml::Value::String(calculate_rel_path(content_dir, collection, file, &repo_rel_path));
                                }
                            }
                        }
                    }
                });
            },
            FieldType::Categories | FieldType::List => {
                if let serde_yaml::Value::Sequence(seq) = value {
                    ui.horizontal_wrapped(|ui| {
                        let mut to_remove = None;
                        for (i, val) in seq.iter().enumerate() {
                            if let Some(tag) = val.as_str() {
                                let color = if field.field_type == FieldType::Categories {
                                    egui::Color32::from_rgb(0, 122, 204)
                                } else {
                                    egui::Color32::from_rgb(100, 100, 100)
                                };
                                let btn = egui::Button::new(egui::RichText::new(format!("{} ✖", tag)).size(12.0).color(egui::Color32::WHITE))
                                    .fill(color)
                                    .rounding(12.0);
                                if ui.add(btn).clicked() { to_remove = Some(i); }
                            }
                        }
                        if let Some(i) = to_remove { seq.remove(i); }
                    });
                    
                    if field.field_type == FieldType::Categories {
                        if let Some(opts) = &field.options {
                            egui::ComboBox::from_id_salt(format!("combo_{}", field.name))
                                .selected_text("Seleccionar categoría...")
                                .show_ui(ui, |ui| {
                                    for opt in opts {
                                        if ui.selectable_label(false, opt).clicked() {
                                            if !seq.iter().any(|v| v.as_str() == Some(opt)) {
                                                seq.push(serde_yaml::Value::String(opt.clone()));
                                            }
                                        }
                                    }
                                });
                        }
                    } else {
                        ui.horizontal(|ui| {
                            ui.text_edit_singleline(new_tag_input);
                            if ui.button("➕").clicked() && !new_tag_input.trim().is_empty() {
                                let new_tag = new_tag_input.trim().to_string();
                                if !seq.iter().any(|v| v.as_str() == Some(&new_tag)) {
                                    seq.push(serde_yaml::Value::String(new_tag));
                                }
                                new_tag_input.clear();
                            }
                        });
                    }
                }
            },
            FieldType::String => {
                if let serde_yaml::Value::String(s) = value {
                    ui.add(egui::TextEdit::multiline(s).desired_rows(1).min_size(egui::vec2(ui.available_width(), 0.0)));
                }
            },
            FieldType::Boolean => {
                if let serde_yaml::Value::Bool(b) = value {
                    ui.checkbox(b, "");
                }
            },
            _ => {}
        }
    }
    ui.add_space(4.0);
}
