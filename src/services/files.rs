//! Servicio de gestión de archivos y colecciones.

use crate::models::{CollectionDef, FieldType, FileEntry};
use std::collections::HashSet;
use std::path::PathBuf;

/// Escanea la carpeta de contenido en busca de subdirectorios (colecciones).
pub fn scan_collections(repo_path: &PathBuf, content_dir: &str) -> Vec<String> {
    let content_path = repo_path.join(content_dir);
    if content_path.exists() {
        std::fs::read_dir(content_path)
            .map(|rd| {
                rd.filter_map(|entry| {
                    let entry = entry.ok()?;
                    if entry.file_type().ok()?.is_dir() {
                        Some(entry.file_name().to_string_lossy().into_owned())
                    } else {
                        None
                    }
                })
                .collect()
            })
            .unwrap_or_default()
    } else {
        Vec::new()
    }
}

/// Escanea los archivos de una colección para extraer metadatos.
pub fn scan_files(repo_path: &PathBuf, content_dir: &str, collection: &str) -> Vec<FileEntry> {
    let collection_path = repo_path.join(content_dir).join(collection);
    if !collection_path.exists() {
        return Vec::new();
    }

    let mut entries = Vec::new();
    for entry in walkdir::WalkDir::new(&collection_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        if let Some(rel_path) = path.strip_prefix(&collection_path).ok() {
            let name = rel_path.to_string_lossy().into_owned().replace("\\", "/");
            if name.ends_with(".md") || name.ends_with(".mdx") {
                let mut title = name.clone();
                let mut date = String::new();
                let mut draft = false;
                let mut image = None;

                if let Ok(content) = std::fs::read_to_string(path) {
                    if content.starts_with("---") {
                        let parts: Vec<&str> = content.splitn(3, "---").collect();
                        if parts.len() >= 2 {
                            if let Ok(fm) = serde_yaml::from_str::<serde_yaml::Mapping>(parts[1]) {
                                if let Some(t) = fm.get(&serde_yaml::Value::String("title".to_string())).and_then(|v| v.as_str()) {
                                    title = t.to_string();
                                }
                                if let Some(d) = fm.get(&serde_yaml::Value::String("date".to_string())).and_then(|v| v.as_str()) {
                                    date = d.to_string();
                                } else if let Some(d) = fm.get(&serde_yaml::Value::String("publishDate".to_string())).and_then(|v| v.as_str()) {
                                    date = d.to_string();
                                }
                                draft = fm.get(&serde_yaml::Value::String("draft".to_string())).and_then(|v| v.as_bool()).unwrap_or(false);
                                if let Some(img) = fm.get(&serde_yaml::Value::String("image".to_string())).and_then(|v| v.as_str()) {
                                    image = Some(img.to_string());
                                }
                            }
                        }
                    }
                }

                entries.push(FileEntry {
                    name,
                    title,
                    date,
                    draft,
                    image,
                });
            }
        }
    }
    entries.sort_by(|a, b| b.date.cmp(&a.date));
    entries
}

/// Crea una caché de todas las etiquetas únicas en los archivos de la colección.
pub fn scan_tags(repo_path: &PathBuf, content_dir: &str, collection: &str) -> Vec<String> {
    let mut tags = HashSet::new();
    let collection_path = repo_path.join(content_dir).join(collection);
    
    if collection_path.exists() {
        for entry in walkdir::WalkDir::new(collection_path).into_iter().flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |e| e == "md" || e == "mdx") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if content.starts_with("---") {
                        let parts: Vec<&str> = content.splitn(3, "---").collect();
                        if parts.len() >= 2 {
                            if let Ok(fm) = serde_yaml::from_str::<serde_yaml::Value>(parts[1]) {
                                if let Some(t) = fm.get("tags").and_then(|v| v.as_sequence()) {
                                    for tag in t {
                                        if let Some(tag_str) = tag.as_str() {
                                            tags.insert(tag_str.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    let mut tags_vec: Vec<String> = tags.into_iter().collect();
    tags_vec.sort();
    tags_vec
}

/// Lee el contenido de un archivo.
pub fn read_file(repo_path: &PathBuf, content_dir: &str, collection: &str, file: &str) -> Option<String> {
    let file_path = repo_path.join(content_dir).join(collection).join(file);
    std::fs::read_to_string(file_path).ok()
}

/// Guarda el contenido en un archivo.
pub fn write_file(repo_path: &PathBuf, content_dir: &str, collection: &str, file: &str, content: &str) -> Result<(), std::io::Error> {
    let file_path = repo_path.join(content_dir).join(collection).join(file);
    std::fs::write(file_path, content)
}

/// Elimina un archivo del disco.
pub fn delete_file(repo_path: &PathBuf, content_dir: &str, collection: &str, file: &str) -> Result<(), std::io::Error> {
    let file_path = repo_path.join(content_dir).join(collection).join(file);
    std::fs::remove_file(file_path)
}

/// Renombra un archivo de .md a .mdx
pub fn rename_to_mdx(repo_path: &PathBuf, content_dir: &str, collection: &str, file: &str) -> Option<String> {
    if file.ends_with(".md") {
        let old_path = repo_path.join(content_dir).join(collection).join(file);
        let new_file = file.replace(".md", ".mdx");
        let new_path = repo_path.join(content_dir).join(collection).join(&new_file);
        
        if std::fs::rename(old_path, new_path).is_ok() {
            return Some(new_file);
        }
    }
    None
}

/// Crea un nuevo archivo con contenido inicial basado en la definición de la colección.
pub fn create_file(
    repo_path: &PathBuf, 
    content_dir: &str, 
    collection: &str, 
    filename: &str,
    collection_def: Option<&CollectionDef>
) -> Result<String, String> {
    let mut filename = filename.trim().to_string();
    if filename.is_empty() { 
        return Err("Nombre de archivo vacío".to_string()); 
    }
    
    if !filename.ends_with(".md") && !filename.ends_with(".mdx") {
        filename.push_str(".md");
    }
    
    let file_path = repo_path.join(content_dir).join(collection).join(&filename);
    
    if file_path.exists() {
        return Err(format!("El archivo {} ya existe", filename));
    }

    // Asegurar que existe el directorio padre
    if let Some(parent) = file_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let now_iso = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S.000Z").to_string();
    let title = filename.replace(".md", "").replace(".mdx", "");

    let initial_content = if let Some(def) = collection_def {
        let mut fm = serde_yaml::Mapping::new();
        for field in &def.fields {
            let val = match field.field_type {
                FieldType::String => serde_yaml::Value::String(if field.name == "title" { title.clone() } else { field.default_value.clone() }),
                FieldType::Boolean => serde_yaml::Value::Bool(field.default_value.parse().unwrap_or(false)),
                FieldType::Date => serde_yaml::Value::String(if field.default_value == "now" { now_iso.clone() } else { field.default_value.clone() }),
                FieldType::List => serde_yaml::Value::Sequence(vec![]),
                FieldType::Categories => serde_yaml::Value::Sequence(vec![]),
                FieldType::Image => serde_yaml::Value::Null,
                FieldType::Number => serde_yaml::Value::Number(field.default_value.parse::<f64>().unwrap_or(0.0).into()),
            };
            fm.insert(serde_yaml::Value::String(field.name.clone()), val);
        }
        let fm_str = serde_yaml::to_string(&fm).unwrap_or_default();
        format!("---\n{}---\n\nEscribe aquí tu contenido...", fm_str)
    } else {
        format!("---\ntitle: \"{}\"\npublishDate: {}\n---\n\nContenido...", title, now_iso)
    };

    std::fs::write(&file_path, initial_content)
        .map(|_| filename)
        .map_err(|e| format!("Error al crear archivo: {}", e))
}

/// Copia una imagen externa al directorio de assets y devuelve su ruta relativa al repo.
pub fn copy_image_to_assets(
    repo_path: &PathBuf, 
    assets_dir: &str, 
    source_path: &std::path::Path
) -> Option<String> {
    let file_name = source_path.file_name()?.to_string_lossy();
    let assets_path = repo_path.join(assets_dir);
    let _ = std::fs::create_dir_all(&assets_path);
    
    let dest_path = assets_path.join(&*file_name);
    if std::fs::copy(source_path, &dest_path).is_ok() {
        Some(format!("{}/{}", assets_dir.replace("\\", "/"), file_name))
    } else {
        None
    }
}
