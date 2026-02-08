//! Modelos de datos para Interstellar Writer.
//!
//! Contiene las estructuras de configuración, definiciones de campos,
//! entradas de archivo y colecciones.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Tipos de datos soportados para los campos del Frontmatter.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FieldType {
    /// Texto simple.
    String,
    /// Valor booleano (true/false).
    Boolean,
    /// Fecha (se autocompleta con 'now' si es necesario).
    Date,
    /// Ruta a una imagen.
    Image,
    /// Lista de elementos (tags).
    List,
    /// Categorías con opciones predefinidas.
    Categories,
    /// Valor numérico.
    Number,
}

/// Definición de un campo dentro de una colección.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FieldDef {
    /// Nombre del campo (clave en el YAML).
    pub name: String,
    /// Tipo de dato del campo.
    pub field_type: FieldType,
    /// Valor por defecto si el campo no existe.
    pub default_value: String,
    /// Opciones sugeridas para campos de tipo Categories o List.
    pub options: Option<Vec<String>>,
}

/// Representa una entrada de archivo en la lista lateral.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FileEntry {
    /// Nombre del archivo en disco.
    pub name: String,
    /// Título extraído del frontmatter.
    pub title: String,
    /// Fecha extraída del frontmatter.
    pub date: String,
    /// Indica si el archivo es un borrador.
    pub draft: bool,
    /// Ruta a la imagen de portada, si tiene.
    pub image: Option<String>,
}

/// Define una colección de contenido (ej. "blog", "docs").
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CollectionDef {
    /// Nombre de la colección (nombre de la carpeta).
    pub name: String,
    /// Campos definidos para esta colección.
    pub fields: Vec<FieldDef>,
}

/// Configuración de rutas y colecciones para un proyecto específico.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectConfig {
    /// Ruta absoluta al repositorio.
    pub repo_path: PathBuf,
    /// Directorio de contenidos relativo a la raíz del repo.
    pub content_dir: String,
    /// Directorio de assets relativo a la raíz del repo.
    pub assets_dir: String,
    /// Colecciones configuradas para este proyecto.
    pub collections: Vec<CollectionDef>,
}

/// Configuración global persistente de la aplicación.
#[derive(Serialize, Deserialize)]
pub struct Config {
    /// Ruta al repositorio actual.
    pub repo_path: Option<PathBuf>,
    /// Token de acceso para integraciones con GitHub (opcional).
    pub github_token: Option<String>,
    /// Preferencia de tema visual.
    pub dark_mode: bool,
    /// Directorio base para el contenido.
    pub content_dir: String,
    /// Directorio base para las imágenes.
    pub assets_dir: String,
    /// Definiciones de colecciones globales.
    pub collections: Vec<CollectionDef>,
    /// Historial de configuraciones de proyectos abiertos.
    #[serde(default)]
    pub project_configs: Vec<ProjectConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            repo_path: None,
            github_token: None,
            dark_mode: true,
            content_dir: "src/content".to_string(),
            assets_dir: "src/assets/imagesblog".to_string(),
            collections: vec![
                CollectionDef {
                    name: "blog".to_string(),
                    fields: vec![
                        FieldDef {
                            name: "title".to_string(),
                            field_type: FieldType::String,
                            default_value: "".to_string(),
                            options: None,
                        },
                        FieldDef {
                            name: "description".to_string(),
                            field_type: FieldType::String,
                            default_value: "".to_string(),
                            options: None,
                        },
                        FieldDef {
                            name: "date".to_string(),
                            field_type: FieldType::Date,
                            default_value: "now".to_string(),
                            options: None,
                        },
                        FieldDef {
                            name: "categories".to_string(),
                            field_type: FieldType::Categories,
                            default_value: "".to_string(),
                            options: Some(vec![
                                "ciberseguridad".to_string(),
                                "pentesting".to_string(),
                                "automatización".to_string(),
                                "tutoriales".to_string(),
                                "hobbies".to_string(),
                                "informática".to_string(),
                                "seguridad".to_string(),
                            ]),
                        },
                        FieldDef {
                            name: "tags".to_string(),
                            field_type: FieldType::List,
                            default_value: "".to_string(),
                            options: None,
                        },
                        FieldDef {
                            name: "author".to_string(),
                            field_type: FieldType::String,
                            default_value: "Juan Oliver".to_string(),
                            options: None,
                        },
                        FieldDef {
                            name: "image".to_string(),
                            field_type: FieldType::Image,
                            default_value: "".to_string(),
                            options: None,
                        },
                        FieldDef {
                            name: "socialImage".to_string(),
                            field_type: FieldType::Image,
                            default_value: "".to_string(),
                            options: None,
                        },
                        FieldDef {
                            name: "showToc".to_string(),
                            field_type: FieldType::Boolean,
                            default_value: "false".to_string(),
                            options: None,
                        },
                        FieldDef {
                            name: "draft".to_string(),
                            field_type: FieldType::Boolean,
                            default_value: "false".to_string(),
                            options: None,
                        },
                    ],
                },
                CollectionDef {
                    name: "docs".to_string(),
                    fields: vec![
                        FieldDef {
                            name: "title".to_string(),
                            field_type: FieldType::String,
                            default_value: "".to_string(),
                            options: None,
                        },
                        FieldDef {
                            name: "description".to_string(),
                            field_type: FieldType::String,
                            default_value: "".to_string(),
                            options: None,
                        },
                        FieldDef {
                            name: "date".to_string(),
                            field_type: FieldType::Date,
                            default_value: "now".to_string(),
                            options: None,
                        },
                        FieldDef {
                            name: "categories".to_string(),
                            field_type: FieldType::List,
                            default_value: "".to_string(),
                            options: None,
                        },
                        FieldDef {
                            name: "tags".to_string(),
                            field_type: FieldType::List,
                            default_value: "".to_string(),
                            options: None,
                        },
                        FieldDef {
                            name: "draft".to_string(),
                            field_type: FieldType::Boolean,
                            default_value: "false".to_string(),
                            options: None,
                        },
                    ],
                },
            ],
            project_configs: vec![],
        }
    }
}

impl Config {
    /// Carga la configuración desde el almacenamiento persistente.
    pub fn load() -> Self {
        let mut cfg: Self = confy::load("interstellar-writer", None).unwrap_or_default();
        // Migración: si no hay proyectos configurados pero tenemos un path, creamos la entrada
        if cfg.project_configs.is_empty() {
            if let Some(path) = &cfg.repo_path {
                cfg.project_configs.push(ProjectConfig {
                    repo_path: path.clone(),
                    content_dir: cfg.content_dir.clone(),
                    assets_dir: cfg.assets_dir.clone(),
                    collections: cfg.collections.clone(),
                });
            }
        }
        cfg
    }

    /// Guarda la configuración en el almacenamiento persistente.
    pub fn save(&self) {
        let _ = confy::store("interstellar-writer", None, self);
    }
}
