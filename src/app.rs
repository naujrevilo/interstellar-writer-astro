//! Estructura principal de la aplicación Interstellar Writer.

use eframe::egui;
use std::path::PathBuf;

use crate::models::{Config, FieldType, FileEntry, ProjectConfig};
use crate::services::{content, files, git};
use crate::ui::{dialogs, dashboard, toolbar, editor, preview, splash, theme, components};
use crate::utils;

/// Estructura principal de la aplicación que mantiene el estado global.
pub struct InterstellarApp {
    /// Configuración persistente del proyecto
    config: Config,
    /// Mensaje que se muestra en la barra de estado inferior
    status_message: String,
    /// Lista de carpetas (colecciones) detectadas en src/content
    collections: Vec<String>,
    /// Colección actualmente seleccionada
    selected_collection: Option<String>,
    /// Lista de archivos detectados en la colección seleccionada
    files: Vec<FileEntry>,
    /// Nombre del archivo cargado actualmente
    selected_file: Option<String>,
    /// Contenido crudo del archivo (Frontmatter + Markdown)
    content: String,
    /// Mapa de valores del Frontmatter (deserializado de YAML)
    frontmatter: serde_yaml::Mapping,
    /// Cuerpo del documento en Markdown/MDX
    body: String,
    
    /// Buffer para el nombre del nuevo archivo en el diálogo de creación
    new_file_name: String,
    /// Buffer para el mensaje de commit al sincronizar con Git
    commit_message: String,
    
    // Estados de visibilidad de diálogos y paneles
    showing_new_file_dialog: bool,
    showing_config_dialog: bool,
    showing_delete_confirm: bool,
    showing_commit_confirm: bool,
    showing_collections: bool,
    showing_files: bool,
    showing_metadata: bool,
    showing_preview: bool,
    showing_markdown_mode: bool,
    showing_about_dialog: bool,
    showing_manual: bool,
    manual_content: String,
    
    /// Canal para recibir resultados de la sincronización en segundo plano
    sync_rx: Option<std::sync::mpsc::Receiver<anyhow::Result<String>>>,
    /// Gestor de notificaciones emergentes
    toasts: egui_notify::Toasts,
    /// Caché para el renderizado eficiente de CommonMark
    commonmark_cache: egui_commonmark::CommonMarkCache,
    /// Caché de etiquetas (tags) encontradas en todos los archivos de la colección
    all_tags: Vec<String>,
    /// Caché de nombres de categorías disponibles (colección `categories`)
    all_categories: Vec<String>,
    /// Buffer para añadir nuevas etiquetas manualmente
    new_tag_input: String,
    /// Selección actual del texto en el editor (caracteres inicio, fin)
    selection: Option<(usize, usize)>,
    /// Posición del cursor forzada para sincronizar con egui::TextEdit
    pending_selection: Option<(usize, usize)>,
    /// Temporizador para la pantalla de inicio (Splash Screen)
    splash_start_time: Option<std::time::Instant>,
    /// Indica si ya se ha completado la visualización del splash screen
    splash_finished: bool,
    /// Indica si hay cambios sin guardar
    is_dirty: bool,
    /// Modo de escritura enfocada
    focus_mode: bool,
    /// Temporizador del último guardado automático (backup)
    last_autosave: Option<std::time::Instant>,
    /// Indica si hay un diálogo de recuperación activo
    showing_recovery_dialog: bool,
    /// Contenido candidato para recuperación desde backup
    backup_content_candidate: Option<String>,
    /// Filtro de búsqueda en el panel de archivos
    file_filter: String,
    /// Muestra/oculta la paleta de comandos (Ctrl+K)
    showing_command_palette: bool,
    /// Query de búsqueda de la paleta de comandos
    command_palette_query: String,
    /// Acción de toolbar pendiente de procesar (para resolver timing con selection)
    pending_toolbar_action: Option<(String, String)>,
    /// Whether the formatting toolbar is expanded (default: hidden per v3 design)
    showing_toolbar: bool,
}

impl InterstellarApp {
    fn load_manual_content() -> String {
        let raw = include_str!("../DOCS/MANUAL_USUARIO.md");
        let assets_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/DOCS/assets/");
        let uri_base = format!("file:///{}", assets_dir.replace('\\', "/"));
        raw.replace("](assets/", &format!("]({}", uri_base))
    }

    fn normalize_frontmatter(&mut self) {
        // labels -> tags (merge y eliminar duplicados)
        let labels_key = serde_yaml::Value::String("labels".to_string());
        let tags_key = serde_yaml::Value::String("tags".to_string());
        if let Some(labels_val) = self.frontmatter.get(&labels_key).cloned() {
            match labels_val {
                serde_yaml::Value::Sequence(seq) => {
                    let mut set = std::collections::BTreeSet::<String>::new();
                    for v in seq {
                        if let Some(s) = v.as_str() {
                            set.insert(s.to_string());
                        }
                    }
                    if let Some(serde_yaml::Value::Sequence(tag_seq)) =
                        self.frontmatter.get(&tags_key)
                    {
                        for v in tag_seq {
                            if let Some(s) = v.as_str() {
                                set.insert(s.to_string());
                            }
                        }
                    }
                    let merged: Vec<serde_yaml::Value> =
                        set.into_iter().map(serde_yaml::Value::String).collect();
                    self.frontmatter
                        .insert(tags_key.clone(), serde_yaml::Value::Sequence(merged));
                    self.frontmatter.remove(&labels_key);
                }
                serde_yaml::Value::String(s) => {
                    let val = serde_yaml::Value::Sequence(vec![serde_yaml::Value::String(s)]);
                    self.frontmatter.insert(tags_key.clone(), val);
                    self.frontmatter.remove(&labels_key);
                }
                _ => {
                    self.frontmatter.remove(&labels_key);
                }
            }
        }

        // category -> categories (merge y dedupe)
        let category_key = serde_yaml::Value::String("category".to_string());
        let categories_key = serde_yaml::Value::String("categories".to_string());
        if let Some(cat_val) = self.frontmatter.get(&category_key).cloned() {
            let mut set = std::collections::BTreeSet::<String>::new();
            match cat_val {
                serde_yaml::Value::String(s) => {
                    set.insert(s);
                }
                serde_yaml::Value::Sequence(seq) => {
                    for v in seq {
                        if let Some(s) = v.as_str() {
                            set.insert(s.to_string());
                        }
                    }
                }
                _ => {}
            }
            if let Some(serde_yaml::Value::Sequence(seq)) =
                self.frontmatter.get(&categories_key)
            {
                for v in seq {
                    if let Some(s) = v.as_str() {
                        set.insert(s.to_string());
                    }
                }
            }
            if !set.is_empty() {
                let merged: Vec<serde_yaml::Value> =
                    set.into_iter().map(serde_yaml::Value::String).collect();
                self.frontmatter
                    .insert(categories_key.clone(), serde_yaml::Value::Sequence(merged));
            }
            self.frontmatter.remove(&category_key);
        }

        // Asegurar unicidad en tags y categories si existen
        for key in &["tags", "categories"] {
            let k = serde_yaml::Value::String((*key).to_string());
            if let Some(serde_yaml::Value::Sequence(seq)) = self.frontmatter.get_mut(&k) {
                let mut set = std::collections::BTreeSet::<String>::new();
                for v in seq.iter() {
                    if let Some(s) = v.as_str() {
                        set.insert(s.to_string());
                    }
                }
                *seq = set
                    .into_iter()
                    .map(serde_yaml::Value::String)
                    .collect::<Vec<_>>();
            }
        }
    }

    fn render_generic_metadata(&mut self, ui: &mut egui::Ui) {
        let tokens = theme::tokens_from_ui(ui);
        let keys: Vec<serde_yaml::Value> = self.frontmatter.keys().cloned().collect();
        for key in keys {
            let label = match &key {
                serde_yaml::Value::String(s) => s.clone(),
                other => format!("{:?}", other),
            };
            ui.label(egui::RichText::new(label).strong());
            if let Some(value) = self.frontmatter.get_mut(&key) {
                match value {
                    serde_yaml::Value::Bool(b) => {
                        ui.checkbox(b, "");
                    }
                    serde_yaml::Value::String(s) => {
                        ui.add(
                            egui::TextEdit::multiline(s)
                                .desired_rows(1)
                                .min_size(egui::vec2(ui.available_width(), 0.0)),
                        );
                    }
                    serde_yaml::Value::Null => {
                        let mut s = String::new();
                        if ui
                            .add(
                                egui::TextEdit::singleline(&mut s)
                                    .hint_text(""),
                            )
                            .changed()
                        {
                            *value = serde_yaml::Value::String(s);
                        }
                    }
                    serde_yaml::Value::Sequence(seq) => {
                        ui.horizontal_wrapped(|ui| {
                            let mut to_remove = None;
                            for (i, val) in seq.iter().enumerate() {
                                if let Some(text) = val.as_str() {
                                    if components::status_badge(
                                        ui,
                                        &format!("{} ✖", text),
                                        tokens.text_muted,
                                    )
                                    .clicked()
                                    {
                                        to_remove = Some(i);
                                    }
                                }
                            }
                            if let Some(i) = to_remove {
                                seq.remove(i);
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.text_edit_singleline(&mut self.new_tag_input);
                            if ui.button("➕").clicked() && !self.new_tag_input.trim().is_empty() {
                                let new_tag = self.new_tag_input.trim().to_string();
                                if !seq
                                    .iter()
                                    .any(|v| v.as_str() == Some(&new_tag))
                                {
                                    seq.push(serde_yaml::Value::String(new_tag));
                                }
                                self.new_tag_input.clear();
                            }
                        });
                    }
                    serde_yaml::Value::Number(n) => {
                        let mut s = n.to_string();
                        if ui
                            .add(
                                egui::TextEdit::singleline(&mut s)
                                    .hint_text("0"),
                            )
                            .changed()
                        {
                            if let Ok(parsed) = s.parse::<f64>() {
                                *value = serde_yaml::Value::Number(parsed.into());
                            }
                        }
                    }
                    _ => {}
                }
            }
            ui.add_space(4.0);
        }
    }

    /// Inicializa la aplicación con la configuración cargada y valores por defecto.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Habilitar cargadores de imágenes (PNG, JPG, etc.)
        egui_extras::install_image_loaders(&cc.egui_ctx);

        // Cargar Newsreader como fuente principal (editorial)
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "Newsreader".to_owned(),
            egui::FontData::from_static(
                include_bytes!("assets/fonts/Newsreader-VariableFont_opsz,wght.ttf"),
            ),
        );
        fonts.font_data.insert(
            "Newsreader-Italic".to_owned(),
            egui::FontData::from_static(
                include_bytes!("assets/fonts/Newsreader-Italic-VariableFont_opsz,wght.ttf"),
            ),
        );
        // Newsreader como primera opción proporcional (fallback a la default de egui)
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "Newsreader".to_owned());
        cc.egui_ctx.set_fonts(fonts);

        let config = Config::load();
        let mut app = Self {
            config,
            status_message: "Bienvenido a Interstellar Writer".to_string(),
            collections: Vec::new(),
            selected_collection: None,
            files: Vec::new(),
            selected_file: None,
            content: String::new(),
            frontmatter: serde_yaml::Mapping::new(),
            body: String::new(),
            new_file_name: String::new(),
            commit_message: "Update content via Interstellar Writer".to_string(),
            showing_new_file_dialog: false,
            showing_config_dialog: false,
            showing_delete_confirm: false,
            showing_commit_confirm: false,
            showing_collections: true,
            showing_files: true,
            showing_metadata: true,
            showing_preview: false,
            showing_markdown_mode: false,
            showing_about_dialog: false,
            showing_manual: false,
            manual_content: Self::load_manual_content(),
            sync_rx: None,
            toasts: egui_notify::Toasts::default(),
            commonmark_cache: egui_commonmark::CommonMarkCache::default(),
            all_tags: Vec::new(),
            all_categories: Vec::new(),
            new_tag_input: String::new(),
            selection: None,
            pending_selection: None,
            splash_start_time: None,
            splash_finished: false,
            is_dirty: false,
            focus_mode: false,
            last_autosave: None,
            showing_recovery_dialog: false,
            backup_content_candidate: None,
            file_filter: String::new(),
            showing_command_palette: false,
            command_palette_query: String::new(),
            pending_toolbar_action: None,
            showing_toolbar: false,
        };
        app.refresh_collections();
        app
    }

    /// Escanea la carpeta de contenido en busca de subdirectorios (colecciones).
    fn refresh_collections(&mut self) {
        if let Some(repo_path) = &self.config.repo_path {
            self.collections = files::scan_collections(repo_path, &self.config.content_dir);
        }
        self.refresh_categories_cache();
    }

    /// Escanea los archivos de la colección seleccionada.
    fn refresh_files(&mut self) {
        if let (Some(repo_path), Some(collection)) = (&self.config.repo_path, &self.selected_collection) {
            self.files = files::scan_files(repo_path, &self.config.content_dir, collection);
        }
    }

    /// Crea una caché de todas las etiquetas únicas en los archivos de la colección.
    fn refresh_tags_cache(&mut self) {
        if let (Some(repo_path), Some(collection)) = (&self.config.repo_path, &self.selected_collection) {
            self.all_tags = files::scan_tags(repo_path, &self.config.content_dir, collection);
        }
    }

    fn refresh_categories_cache(&mut self) {
        self.all_categories.clear();
        if let Some(repo_path) = &self.config.repo_path {
            let categories_name = "categories";
            let categories_path = repo_path.join(&self.config.content_dir).join(categories_name);
            if categories_path.exists() {
                let entries = files::scan_files(repo_path, &self.config.content_dir, categories_name);
                self.all_categories = entries.into_iter().map(|e| e.title).collect();
            }
        }
    }

    /// Carga el contenido de un archivo seleccionado y lo parsea.
    fn load_file(&mut self) {
        let file_name = self.selected_file.clone();
        if let (Some(repo_path), Some(collection), Some(file)) = 
            (&self.config.repo_path, &self.selected_collection, &file_name) 
        {
            if let Some(file_content) = files::read_file(repo_path, &self.config.content_dir, collection, file) {
                self.content = file_content;
                let parsed = content::parse_content(&self.content);
                self.frontmatter = parsed.frontmatter;
                self.body = parsed.body;
                self.ensure_mandatory_fields();
                self.normalize_frontmatter();
                self.status_message = format!("Cargado: {}", file);
                self.is_dirty = false;
            }
        }
        self.check_backup_for_recovery();
    }

    /// Asegura que el Frontmatter tenga los campos mínimos requeridos.
    fn ensure_mandatory_fields(&mut self) {
        if let Some(collection) = &self.selected_collection {
            let now_iso = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S.000Z").to_string();

            let def_opt = self
                .config
                .collections
                .iter()
                .find(|c| c.name == *collection)
                .or_else(|| self.config.collections.first());

            if let Some(def) = def_opt {
                for field in &def.fields {
                    let yaml_key = serde_yaml::Value::String(field.name.clone());
                    if !self.frontmatter.contains_key(&yaml_key) {
                        let val = match field.field_type {
                            FieldType::String => serde_yaml::Value::String(field.default_value.clone()),
                            FieldType::Boolean => serde_yaml::Value::Bool(field.default_value.parse().unwrap_or(false)),
                            FieldType::Date => serde_yaml::Value::String(
                                if field.default_value == "now" { now_iso.clone() } else { field.default_value.clone() }
                            ),
                            FieldType::List => serde_yaml::Value::Sequence(vec![]),
                            FieldType::Categories => serde_yaml::Value::Sequence(vec![]),
                            FieldType::Image => serde_yaml::Value::Null,
                            FieldType::Number => serde_yaml::Value::Number(
                                field.default_value.parse::<f64>().unwrap_or(0.0).into()
                            ),
                        };
                        self.frontmatter.insert(yaml_key, val);
                    }
                }
            }
        }
    }

    /// Guarda los cambios en el archivo actual.
    fn save_file(&mut self) {
        if let (Some(repo_path), Some(collection), Some(file)) = 
            (&self.config.repo_path, &self.selected_collection, &self.selected_file) 
        {
            self.content = content::serialize_content(&self.frontmatter, &self.body);
            if files::write_file(repo_path, &self.config.content_dir, collection, file, &self.content).is_ok() {
                self.status_message = format!("✅ Guardado: {}", file);
                self.toasts.success(&self.status_message);
                self.refresh_tags_cache();
                self.is_dirty = false;
                self.last_autosave = None;
                if let Some(bp) = self.backup_path() {
                    let _ = std::fs::remove_file(bp);
                }
            } else {
                self.status_message = "❌ Error al guardar".to_string();
            }
        }
    }

    /// Crea un nuevo archivo en la colección seleccionada.
    fn create_file(&mut self) {
        if let (Some(repo_path), Some(collection)) = (&self.config.repo_path, &self.selected_collection) {
            let coll_def = self.config.collections.iter().find(|c| c.name == *collection);
            
            match files::create_file(repo_path, &self.config.content_dir, collection, &self.new_file_name, coll_def) {
                Ok(filename) => {
                    self.status_message = format!("✨ Creado: {}", filename);
                    self.toasts.success(&self.status_message);
                    self.new_file_name.clear();
                    self.showing_new_file_dialog = false;
                    self.refresh_files();
                    self.selected_file = Some(filename);
                    self.load_file();
                }
                Err(e) => {
                    self.status_message = format!("❌ {}", e);
                }
            }
        }
    }

    /// Elimina el archivo seleccionado.
    fn delete_selected_file(&mut self) {
        if let (Some(repo_path), Some(collection), Some(file)) = 
            (&self.config.repo_path, &self.selected_collection, &self.selected_file) 
        {
            // Solo permitir borrar si es borrador
            let is_draft = self.frontmatter.get(&serde_yaml::Value::String("draft".to_string()))
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            
            if !is_draft {
                self.status_message = "⚠️ Solo se pueden eliminar archivos en estado 'draft: true'".to_string();
                self.toasts.warning(&self.status_message);
                return;
            }

            if files::delete_file(repo_path, &self.config.content_dir, collection, file).is_ok() {
                self.status_message = format!("🗑 Eliminado: {}", file);
                self.toasts.info(&self.status_message);
                self.selected_file = None;
                self.content.clear();
                self.frontmatter = serde_yaml::Mapping::new();
                self.body.clear();
                self.refresh_files();
            } else {
                self.status_message = "❌ Error al eliminar archivo (verifica permisos)".to_string();
            }
        }
    }

    /// Recalcula las rutas relativas de imágenes e importaciones.
    fn fix_all_image_paths(&mut self) {
        if let (Some(_), Some(file)) = (&self.selected_collection, &self.selected_file) {
            let depth = file.split('/').count();
            let needed_dots = "../".repeat(depth + 1);
            
            // 1. Reparar Frontmatter
            let mut fm_value = serde_yaml::Value::Mapping(self.frontmatter.clone());
            content::fix_dots_recursively(&mut fm_value, &needed_dots);
            if let serde_yaml::Value::Mapping(m) = fm_value {
                self.frontmatter = m;
            }
            
            // 2. Reparar Body
            let patterns = ["../../../assets/", "../../assets/", "../assets/", "./assets/"];
            for p in patterns {
                let target = format!("{}assets/", needed_dots);
                self.body = self.body.replace(p, &target);
            }

            // 3. Reparar importaciones de componentes
            let components = ["Notice.astro", "CTABox.astro"];
            for comp in components {
                let lines: Vec<String> = self.body.lines().map(|line| {
                    if line.contains("import") && line.contains(comp) {
                        let target_path = format!("{}components/{}", needed_dots, comp);
                        if let Some(start) = line.find('"') {
                            if let Some(_) = line[start+1..].find('"') {
                                let new_line = format!("{}from \"{}\";", &line[..line.find("from").unwrap_or(line.len())], target_path);
                                return new_line;
                            }
                        }
                    }
                    line.to_string()
                }).collect();
                self.body = lines.join("\n");
            }
            
            self.status_message = "🔧 Rutas de activos y componentes recalculadas".to_string();
            self.toasts.info(&self.status_message);
        }
    }

    /// Cambia la extensión del archivo actual a .mdx.
    fn rename_to_mdx(&mut self) {
        if let (Some(repo_path), Some(collection), Some(file)) = 
            (&self.config.repo_path, &self.selected_collection, &self.selected_file) 
        {
            if let Some(new_file) = files::rename_to_mdx(repo_path, &self.config.content_dir, collection, file) {
                self.selected_file = Some(new_file);
                self.status_message = "✨ Convertido a .mdx".to_string();
                self.toasts.success(&self.status_message);
                self.refresh_files();
            } else {
                self.status_message = "❌ Error al renombrar archivo".to_string();
            }
        }
    }

    /// Inserta texto en la posición actual del cursor.
    fn insert_at_cursor(&mut self, text: &str) {
        self.insert_replacement(text, "");
    }

    fn apply_format(&mut self, prefix: &str, suffix: &str) {
        self.insert_replacement(prefix, suffix);
    }

    fn fuzzy_match(query: &str, candidate: &str) -> bool {
        let q = query.trim().to_lowercase();
        if q.is_empty() {
            return true;
        }

        let c = candidate.to_lowercase();
        if c.contains(&q) {
            return true;
        }

        let mut q_chars = q.chars();
        let mut next_q = q_chars.next();
        for ch in c.chars() {
            if Some(ch) == next_q {
                next_q = q_chars.next();
                if next_q.is_none() {
                    return true;
                }
            }
        }
        false
    }

    /// Lógica central de inserción con manejo de selección.
    fn insert_replacement(&mut self, before: &str, after: &str) {
        self.showing_markdown_mode = false;
        let (start_char, end_char) = self.selection.unwrap_or((self.body.chars().count(), self.body.chars().count()));
        let start_idx = start_char.min(end_char);
        let end_idx = start_char.max(end_char);
        let had_selection = start_idx != end_idx;

        let mut char_indices = self.body.char_indices();
        let start_byte = char_indices.nth(start_idx).map(|(i, _)| i).unwrap_or(self.body.len());
        let end_byte = self.body.char_indices().nth(end_idx).map(|(i, _)| i).unwrap_or(self.body.len());

        let mut selected_text = self.body[start_byte..end_byte].to_string();
        let mut select_inserted_text = false;

        if !had_selection {
            selected_text.clear();
            if before.contains("<Notice") {
                selected_text = "Escribe aquí tu contenido...".to_string();
                select_inserted_text = true;
            } else if before.contains("```") || before.contains("<pre") {
                selected_text = "\n  // Código aquí\n".to_string();
                select_inserted_text = true;
            }
        }

        let mut final_before = before.to_string();
        let prefix = &self.body[..start_byte];
        
        // Lógica de saltos de línea
        if prefix.is_empty() {
            final_before = final_before.trim_start_matches('\n').to_string();
        } else if prefix.ends_with("\n\n") {
            final_before = final_before.trim_start_matches('\n').to_string();
        } else if prefix.ends_with('\n') {
            if final_before.starts_with("\n\n") {
                final_before = final_before[1..].to_string();
            }
        } else if !prefix.is_empty() && (before.contains('<') || before.contains('|')) {
            if !final_before.starts_with('\n') {
                final_before = format!("\n\n{}", final_before);
            }
        }

        let new_content = format!("{}{}{}", final_before, selected_text, after);
        let mut new_body = String::with_capacity(self.body.len() + final_before.len() + after.len());
        new_body.push_str(&self.body[..start_byte]);
        new_body.push_str(&new_content);
        new_body.push_str(&self.body[end_byte..]);
        
        self.body = new_body;

        let before_len = final_before.chars().count();
        let selected_len = selected_text.chars().count();
        let after_len = after.chars().count();

        if had_selection {
            let new_pos = start_idx + before_len + selected_len + after_len;
            self.selection = Some((new_pos, new_pos));
            self.pending_selection = Some((new_pos, new_pos));
        } else if select_inserted_text && selected_len > 0 {
            let sel_start = start_idx + before_len;
            let sel_end = sel_start + selected_len;
            self.selection = Some((sel_start, sel_end));
            self.pending_selection = Some((sel_start, sel_end));
        } else {
            let new_pos = start_idx + before_len;
            self.selection = Some((new_pos, new_pos));
            self.pending_selection = Some((new_pos, new_pos));
        }
        self.is_dirty = true;
    }

    /// Verifica si un import ya existe y si no, lo añade al principio del cuerpo.
    fn ensure_import(&mut self, import_stmt: &str) {
        let component_tag = if import_stmt.contains("import Notice from") {
            Some("import Notice from")
        } else if import_stmt.contains("import CTABox from") {
            Some("import CTABox from")
        } else if import_stmt.contains("astro:assets") {
            Some("import { Image }")
        } else {
            None
        };

        let already_has_component = if let Some(tag) = component_tag {
            self.body.contains(tag)
        } else {
            self.body.contains(import_stmt)
        };

        if !already_has_component {
            self.body.insert_str(0, &format!("{}\n", import_stmt));
            
            let shift = import_stmt.chars().count() + 1;
            if let Some((start, end)) = self.selection {
                let new_sel = (start + shift, end + shift);
                self.selection = Some(new_sel);
                self.pending_selection = Some(new_sel);
            }
            self.status_message = "📦 Componente importado".to_string();
        }
    }

    /// Inserta un componente <Notice />.
    fn insert_notice(&mut self, notice_type: &str, title: &str) {
        let file_name = self.selected_file.clone().unwrap_or_default();
        let coll = self.selected_collection.clone().unwrap_or_default();
        
        let rel_path = content::calculate_rel_path(&self.config.content_dir, &coll, &file_name, "src/components/Notice.astro");
        let import_stmt = format!("import Notice from \"{}\";", rel_path);
        
        self.ensure_import(&import_stmt);
        
        let before = format!("\n\n<Notice type=\"{}\" title=\"{}\">\n", notice_type, title);
        let after = "\n</Notice>\n\n";
        
        self.insert_replacement(&before, &after);
        self.status_message = format!("✅ Aviso '{}' insertado", title);
    }

    /// Maneja la inserción de una imagen.
    fn handle_image_insertion(&mut self) {
        if let Some(path) = utils::pick_image_file() {
            if let Some(repo_path) = &self.config.repo_path {
                if let Some(repo_rel_img_path) = files::copy_image_to_assets(repo_path, &self.config.assets_dir, &path) {
                    let rel_path = if let (Some(coll), Some(file)) = (&self.selected_collection, &self.selected_file) {
                        content::calculate_rel_path(&self.config.content_dir, coll, file, &repo_rel_img_path)
                    } else {
                        repo_rel_img_path.clone()
                    };
                    let file_name = std::path::Path::new(&repo_rel_img_path).file_name().unwrap().to_string_lossy();
                    let base_name = std::path::Path::new(&repo_rel_img_path).file_stem().unwrap().to_string_lossy().replace("-", "_").replace(" ", "_");
                    
                    let import_stmt = format!("import img_{} from \"{}\";", base_name, rel_path);
                    self.ensure_import(&import_stmt);
                    self.ensure_import("import { Image } from \"astro:assets\";");

                    let markdown_img = format!(
                        "\n\n<div class=\"my-12 flex flex-col items-center\">\n  <Image\n    src={{img_{}}}\n    alt=\"{}\"\n    class=\"rounded-xl shadow-lg w-full max-w-3xl\"\n  />\n  <p class=\"text-center text-sm text-slate-500 mt-3 italic\">\n    Descripción de la imagen\n  </p>\n</div>\n\n", 
                        base_name, file_name
                    );
                    self.insert_at_cursor(&markdown_img);
                    
                    self.status_message = "🖼 Imagen insertada con componente Astro".to_string();
                }
            }
        }
    }

    /// Actualiza la configuración de rutas para el repositorio actual.
    fn sync_project_config(&mut self) {
        if let Some(path) = &self.config.repo_path {
            if let Some(p) = self.config.project_configs.iter_mut().find(|p| p.repo_path == *path) {
                p.content_dir = self.config.content_dir.clone();
                p.assets_dir = self.config.assets_dir.clone();
                p.collections = self.config.collections.clone();
            } else {
                self.config.project_configs.push(ProjectConfig {
                    repo_path: path.clone(),
                    content_dir: self.config.content_dir.clone(),
                    assets_dir: self.config.assets_dir.clone(),
                    collections: self.config.collections.clone(),
                });
            }
        }
    }

    /// Cambia el repositorio activo y recarga las colecciones.
    fn set_repo_path(&mut self, path: PathBuf) {
        self.sync_project_config();
        
        self.config.repo_path = Some(path.clone());
        
        if let Some(p) = self.config.project_configs.iter().find(|p| p.repo_path == path) {
            self.config.content_dir = p.content_dir.clone();
            self.config.assets_dir = p.assets_dir.clone();
            self.config.collections = p.collections.clone();
        } else {
            self.config.project_configs.push(ProjectConfig {
                repo_path: path.clone(),
                content_dir: self.config.content_dir.clone(),
                assets_dir: self.config.assets_dir.clone(),
                collections: self.config.collections.clone(),
            });
        }
        
        self.refresh_collections();
        self.selected_collection = None;
        self.selected_file = None;
        self.content.clear();
        self.frontmatter = serde_yaml::Mapping::new();
        self.body.clear();
    }

    /// Inicia la sincronización con GitHub en segundo plano.
    fn start_sync(&mut self) {
        self.save_file();
        
        let repo_path = self.config.repo_path.clone();
        let token = self.config.github_token.clone();
        let commit_msg = self.commit_message.clone();
        let content_dir = self.config.content_dir.clone();
        let assets_dir = self.config.assets_dir.clone();
        
        if let (Some(repo_path), Some(token)) = (repo_path, token) {
            let (tx, rx) = std::sync::mpsc::channel();
            self.sync_rx = Some(rx);
            self.status_message = "⏳ Sincronizando con GitHub...".to_string();
            
            std::thread::spawn(move || {
                let result = git::sync_to_github(repo_path, token, commit_msg, content_dir, assets_dir);
                let _ = tx.send(result);
            });
        }
    }

    fn backup_path(&self) -> Option<std::path::PathBuf> {
        let base = dirs::data_local_dir()?.join("interstellar-writer").join("backups");
        let col = self.selected_collection.as_deref().unwrap_or("default");
        let file = self.selected_file.as_deref().unwrap_or("unknown");
        Some(base.join(col).join(format!("{}.bak", file)))
    }

    fn save_backup(&self) {
        let Some(path) = self.backup_path() else { return };
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(&path, self.body.as_bytes());
    }

    fn check_backup_for_recovery(&mut self) {
        let repo = self.config.repo_path.clone();
        let coll = self.selected_collection.clone();
        let file = self.selected_file.clone();
        let content_dir = self.config.content_dir.clone();
        let bp = match self.backup_path() {
            Some(p) => p,
            None => return,
        };
        if let (Some(repo), Some(coll), Some(file)) = (repo, coll, file) {
            let file_path = repo.join(&content_dir).join(&coll).join(&file);
            if let (Ok(bm), Ok(om)) = (
                std::fs::metadata(&bp).and_then(|m| m.modified()),
                std::fs::metadata(&file_path).and_then(|m| m.modified()),
            ) {
                if bm > om {
                    if let Ok(content) = std::fs::read_to_string(&bp) {
                        self.backup_content_candidate = Some(content);
                        self.showing_recovery_dialog = true;
                    }
                }
            }
        }
    }
}

impl eframe::App for InterstellarApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- SPLASH SCREEN ---
        if !self.splash_finished {
            if self.selected_file.is_none() && self.selected_collection.is_none() {
                if !splash::show_splash(ctx, &mut self.splash_start_time) {
                    return;
                }
            }
            self.splash_finished = true;
            self.splash_start_time = None;
        }

        // Manejar resultado de sincronización
        if let Some(rx) = &self.sync_rx {
            if let Ok(result) = rx.try_recv() {
                match result {
                    Ok(msg) => {
                        self.status_message = msg.clone();
                        self.toasts.success(msg).duration(Some(std::time::Duration::from_secs(5)));
                    },
                    Err(e) => {
                        self.status_message = format!("❌ Error en Git: {}", e);
                        self.toasts.error(self.status_message.clone()).duration(None);
                    },
                }
                self.sync_rx = None;
            }
        }

        self.toasts.show(ctx);
        theme::apply_theme(ctx, self.config.dark_mode);

        // --- ATAJOS DE TECLADO ---
        let mut save_now = false;
        let mut toggle_focus = false;
        let mut toggle_palette = false;
        let mut escape_pressed = false;
        ctx.input(|i| {
            if i.key_pressed(egui::Key::S) && i.modifiers.ctrl { save_now = true; }
            if i.key_pressed(egui::Key::F11) { toggle_focus = true; }
            if i.key_pressed(egui::Key::K) && i.modifiers.ctrl { toggle_palette = true; }
            if i.key_pressed(egui::Key::Escape) { escape_pressed = true; }
        });
        if save_now && self.selected_file.is_some() { self.save_file(); }
        if toggle_focus { self.focus_mode = !self.focus_mode; }
        if toggle_palette { self.showing_command_palette = !self.showing_command_palette; }
        if escape_pressed && self.showing_command_palette {
            self.showing_command_palette = false;
            self.command_palette_query.clear();
        }

        // --- DIÁLOGOS ---
        if self.showing_about_dialog {
            dialogs::show_about_dialog(ctx, &mut self.showing_about_dialog);
        }

        if self.showing_manual {
            dialogs::show_manual_dialog(ctx, &mut self.showing_manual, &self.manual_content, &mut self.commonmark_cache);
        }

        if self.showing_recovery_dialog {
            egui::Window::new("Borrador recuperado").show(ctx, |ui| {
                ui.label("Existe un borrador más reciente sin guardar. ¿Restaurar?");
                ui.horizontal(|ui| {
                    if components::success_button(ui, "✅ Restaurar borrador").clicked() {
                        if let Some(c) = self.backup_content_candidate.take() {
                            self.body = c;
                            self.is_dirty = true;
                        }
                        self.showing_recovery_dialog = false;
                    }
                    if components::danger_button(ui, "🗑 Descartar").clicked() {
                        self.backup_content_candidate = None;
                        self.showing_recovery_dialog = false;
                        if let Some(bp) = self.backup_path() { let _ = std::fs::remove_file(bp); }
                    }
                });
            });
        }

        // --- PALETA DE COMANDOS (Ctrl+K) ---
        if self.showing_command_palette {
            let files_snapshot: Vec<_> = self.files.clone();
            egui::Window::new("🔍 Paleta de comandos")
                .collapsible(false)
                .resizable(true)
                .default_width(520.0)
                .anchor(egui::Align2::CENTER_TOP, egui::vec2(0.0, 60.0))
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        let response = ui.add(
                            egui::TextEdit::singleline(&mut self.command_palette_query)
                                .hint_text("Buscar archivo por título o nombre…")
                                .desired_width(ui.available_width() - 40.0),
                        );
                        response.request_focus();
                        if ui.button("✕").clicked() {
                            self.showing_command_palette = false;
                            self.command_palette_query.clear();
                        }
                    });
                    ui.add_space(4.0);
                    ui.separator();
                    let query = self.command_palette_query.to_lowercase();
                    let matches: Vec<_> = files_snapshot.iter()
                        .filter(|f| {
                            Self::fuzzy_match(&query, &f.title)
                                || Self::fuzzy_match(&query, &f.name)
                        })
                        .take(12)
                        .cloned()
                        .collect();
                    egui::ScrollArea::vertical().max_height(320.0).show(ui, |ui| {
                        let mut to_open: Option<String> = None;
                        for entry in &matches {
                            let icon = if entry.draft { "🔒" } else { "✅" };
                            let row = format!("{} {}  —  {}", icon, entry.title, entry.name);
                            if ui.selectable_label(false, row).clicked() {
                                to_open = Some(entry.name.clone());
                            }
                        }
                        if let Some(name) = to_open {
                            self.selected_file = Some(name);
                            self.load_file();
                            self.showing_command_palette = false;
                            self.command_palette_query.clear();
                        }
                    });
                });
        }

        if self.showing_delete_confirm {
            if dialogs::show_delete_confirm_dialog(ctx, &mut self.showing_delete_confirm) {
                self.delete_selected_file();
            }
        }

        if self.showing_commit_confirm {
            if dialogs::show_commit_confirm_dialog(ctx, &mut self.showing_commit_confirm, &mut self.commit_message) {
                self.start_sync();
            }
        }

        if self.sync_rx.is_some() {
            dialogs::show_syncing_indicator(ctx);
        }

        if self.showing_new_file_dialog {
            if let Some(_) = dialogs::show_new_file_dialog(ctx, &mut self.showing_new_file_dialog, &mut self.new_file_name) {
                self.create_file();
            }
        }

        if self.showing_config_dialog {
            let should_refresh = dialogs::show_config_dialog(
                ctx, 
                &mut self.showing_config_dialog, 
                &mut self.config,
                || utils::pick_folder(),
            );
            if should_refresh {
                if let Some(path) = &self.config.repo_path {
                    self.set_repo_path(path.clone());
                } else {
                    self.refresh_collections();
                }
            }
        }

        // --- TOP BAR — minimal chrome ---
        egui::TopBottomPanel::top("top_panel")
            .frame(egui::Frame::none()
                .fill(if self.config.dark_mode {
                    egui::Color32::from_rgb(23, 23, 26) // panel_sidebar_bg dark
                } else {
                    egui::Color32::from_rgb(236, 234, 231) // panel_sidebar_bg light
                })
                .inner_margin(egui::Margin::symmetric(12.0, 6.0)))
            .show(ctx, |ui| {
                let tokens = theme::tokens_from_ui(ui);
                ui.horizontal(|ui| {
                    // Logo + title
                    ui.add(egui::Image::new(egui::include_image!("../logo.svg")).max_height(18.0));
                    ui.add_space(6.0);
                    ui.label(
                        egui::RichText::new("Interstellar Writer")
                            .font(egui::FontId::proportional(14.0))
                            .color(tokens.text_muted),
                    );

                    // Breadcrumbs if file is open
                    if let Some(file) = &self.selected_file {
                        ui.label(egui::RichText::new("/").color(tokens.text_faint));
                        let dirty_mark = if self.is_dirty { " ●" } else { "" };
                        let parts: Vec<&str> = file.split('/').collect();
                        if parts.len() > 1 {
                            for (i, part) in parts.iter().enumerate() {
                                if i > 0 {
                                    ui.label(egui::RichText::new("/").color(tokens.text_faint));
                                }
                                let is_last = i == parts.len() - 1;
                                let text = if is_last {
                                    egui::RichText::new(format!("{}{}", part, dirty_mark))
                                        .color(tokens.text_primary)
                                } else {
                                    egui::RichText::new(*part).color(tokens.text_muted)
                                };
                                ui.label(text);
                            }
                        } else {
                            ui.label(
                                egui::RichText::new(format!("{}{}", file, dirty_mark))
                                    .color(tokens.text_primary),
                            );
                        }
                    }

                    // Right-aligned actions
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Settings
                        if ui
                            .add(egui::Button::new(egui::RichText::new("Settings").color(tokens.text_muted).small()).fill(egui::Color32::TRANSPARENT))
                            .clicked()
                        {
                            self.showing_config_dialog = true;
                        }

                        // Theme toggle
                        let theme_label = if self.config.dark_mode { "Light" } else { "Dark" };
                        if ui
                            .add(egui::Button::new(egui::RichText::new(theme_label).color(tokens.text_muted).small()).fill(egui::Color32::TRANSPARENT))
                            .clicked()
                        {
                            self.config.dark_mode = !self.config.dark_mode;
                            self.config.save();
                        }

                        // View toggles via menu
                        ui.menu_button(egui::RichText::new("View").color(tokens.text_muted).small(), |ui| {
                            ui.checkbox(&mut self.showing_collections, "Collections");
                            ui.checkbox(&mut self.showing_files, "Files");
                            ui.checkbox(&mut self.showing_metadata, "Metadata");
                            ui.checkbox(&mut self.showing_preview, "Preview");
                        });

                        // Help menu
                        ui.menu_button(egui::RichText::new("Help").color(tokens.text_muted).small(), |ui| {
                            if ui.button("User Manual").clicked() {
                                self.showing_manual = true;
                                ui.close_menu();
                            }
                            if ui.button("About").clicked() {
                                self.showing_about_dialog = true;
                                ui.close_menu();
                            }
                        });

                        // Push/sync
                        if self.config.repo_path.is_some() {
                            if ui
                                .add(egui::Button::new(egui::RichText::new("Push").color(tokens.brand_primary).small()).fill(egui::Color32::TRANSPARENT))
                                .on_hover_text("Commit & push to GitHub")
                                .clicked()
                            {
                                self.showing_commit_confirm = true;
                            }
                        }

                        // Save + delete when file is open
                        if self.selected_file.is_some() {
                            if ui
                                .add(egui::Button::new(egui::RichText::new("Save").color(tokens.brand_success).small()).fill(egui::Color32::TRANSPARENT))
                                .clicked()
                            {
                                self.save_file();
                            }

                            let is_draft = self
                                .frontmatter
                                .get(&serde_yaml::Value::String("draft".to_string()))
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false);

                            if is_draft {
                                if ui
                                    .add(egui::Button::new(egui::RichText::new("Del").color(tokens.brand_danger).small()).fill(egui::Color32::TRANSPARENT))
                                    .on_hover_text("Delete this draft")
                                    .clicked()
                                {
                                    self.showing_delete_confirm = true;
                                }
                            }
                        }
                    });
                });
            });

        // --- BOTTOM BAR — subtle status ---
        egui::TopBottomPanel::bottom("bottom_panel")
            .frame(egui::Frame::none()
                .fill(if self.config.dark_mode {
                    egui::Color32::from_rgb(23, 23, 26)
                } else {
                    egui::Color32::from_rgb(236, 234, 231)
                })
                .inner_margin(egui::Margin::symmetric(12.0, 4.0)))
            .show(ctx, |ui| {
                let tokens = theme::tokens_from_ui(ui);
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(&self.status_message).color(tokens.text_muted).small());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if self.selected_file.is_some() {
                            let words = self.body.split_whitespace().count();
                            let mins = (words / 200).max(1);
                            ui.label(
                                egui::RichText::new(format!("{} words  ~{} min", words, mins))
                                    .color(tokens.text_faint)
                                    .small(),
                            );
                        }
                    });
                });
            });

        // --- PANEL IZQUIERDO ---
        let collections = self.collections.clone();
        let files_list = self.files.clone();
        let mut should_refresh_files = false;
        let mut should_refresh_tags = false;
        let mut file_to_load: Option<String> = None;

        let showing_left = (self.showing_collections || self.showing_files) && !self.focus_mode;

        if showing_left && self.showing_collections {
            egui::SidePanel::left("left_tag_rail")
                .resizable(false)
                .default_width(56.0)
                .frame(
                    egui::Frame::none()
                        .fill(if self.config.dark_mode {
                            egui::Color32::from_rgb(20, 20, 19)
                        } else {
                            egui::Color32::from_rgb(236, 234, 231)
                        })
                        .inner_margin(egui::Margin::symmetric(0.0, 12.0)),
                )
                .show(ctx, |ui| {
                    let tokens = theme::tokens_from_ui(ui);
                    ui.spacing_mut().item_spacing = egui::vec2(4.0, 4.0);

                    ui.vertical_centered(|ui| {
                        ui.add_space(2.0);
                        ui.add(
                            egui::Image::new(egui::include_image!("../logo.svg"))
                                .max_height(22.0),
                        )
                        .on_hover_text("Interstellar Writer");
                        ui.add_space(8.0);

                        ui.painter().line_segment(
                            [
                                egui::pos2(ui.min_rect().center().x - 16.0, ui.cursor().top()),
                                egui::pos2(ui.min_rect().center().x + 16.0, ui.cursor().top()),
                            ],
                            egui::Stroke::new(1.0, tokens.divider),
                        );
                        ui.add_space(10.0);
                    });

                    let collection_abbrev = |name: &str| -> &'static str {
                        match name {
                            "blog" => "B",
                            "docs" => "D",
                            "projects" => "P",
                            "app-landings" => "A",
                            _ => "•",
                        }
                    };

                    ui.vertical_centered(|ui| {
                        for collection in &collections {
                            let selected = self.selected_collection.as_ref() == Some(collection);
                            let bg = if selected {
                                tokens.subtle_bg
                            } else {
                                egui::Color32::TRANSPARENT
                            };
                            let fg = if selected { tokens.text_primary } else { tokens.text_faint };

                            let button = egui::Button::new(
                                egui::RichText::new(collection_abbrev(collection))
                                    .size(14.0)
                                    .strong()
                                    .color(fg),
                            )
                            .fill(bg)
                            .rounding(tokens.radius_md);

                            if ui
                                .add_sized([36.0, 36.0], button)
                                .on_hover_text(collection)
                                .clicked()
                            {
                                self.selected_collection = Some(collection.clone());
                                should_refresh_files = true;
                                should_refresh_tags = true;
                                self.selected_file = None;
                                self.content.clear();
                                self.file_filter.clear();
                            }
                        }
                    });

                    ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                        let button = egui::Button::new(
                            egui::RichText::new("R")
                                .size(14.0)
                                .color(tokens.text_faint),
                        )
                        .fill(egui::Color32::TRANSPARENT)
                        .rounding(tokens.radius_md);

                        if ui.add_sized([36.0, 36.0], button).clicked() {
                            self.refresh_collections();
                        }
                    });
                });

            egui::SidePanel::left("left_divider_rail")
                .resizable(false)
                .default_width(1.0)
                .frame(egui::Frame::none().fill(if self.config.dark_mode {
                    egui::Color32::from_rgb(37, 35, 32)
                } else {
                    egui::Color32::from_rgb(221, 217, 213)
                }))
                .show(ctx, |_| {});
        }

        if showing_left && self.showing_files && self.selected_collection.is_some() {
            let file_key = self.selected_file.as_deref().unwrap_or("");
            egui::SidePanel::left(egui::Id::new(("left_file_list", file_key)))
                .resizable(false)
                .default_width(300.0)
                .frame(
                    egui::Frame::none()
                        .fill(if self.config.dark_mode {
                            egui::Color32::from_rgb(23, 23, 26)
                        } else {
                            egui::Color32::from_rgb(236, 234, 231)
                        })
                        .inner_margin(egui::Margin::symmetric(16.0, 16.0)),
                )
                .show(ctx, |ui| {
                    let tokens = theme::tokens_from_ui(ui);
                    let coll = self.selected_collection.clone().unwrap_or_default();

                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(&coll)
                                .italics()
                                .size(20.0)
                                .color(tokens.text_primary),
                        );
                        ui.add_space(ui.available_width() - 40.0);
                        let plus = egui::Button::new(
                            egui::RichText::new("+")
                                .size(16.0)
                                .color(tokens.text_faint),
                        )
                        .fill(egui::Color32::TRANSPARENT);
                        if ui.add(plus).clicked() {
                            self.showing_new_file_dialog = true;
                        }
                    });

                    ui.add_space(10.0);

                    egui::Frame::none()
                        .fill(if self.config.dark_mode {
                            egui::Color32::from_rgb(28, 27, 26)
                        } else {
                            egui::Color32::from_rgb(245, 243, 240)
                        })
                        .rounding(tokens.radius_sm)
                        .inner_margin(egui::Margin::symmetric(10.0, 8.0))
                        .show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::singleline(&mut self.file_filter)
                                    .hint_text("Buscar…")
                                    .desired_width(ui.available_width()),
                            );
                        });

                    ui.add_space(10.0);

                    let filter_lc = self.file_filter.to_lowercase();
                    let filtered: Vec<_> = files_list
                        .iter()
                        .filter(|f| {
                            filter_lc.is_empty()
                                || f.title.to_lowercase().contains(&filter_lc)
                                || f.name.to_lowercase().contains(&filter_lc)
                        })
                        .collect();

                    egui::ScrollArea::vertical()
                        .id_salt("file_scroll_v3")
                        .show(ui, |ui| {
                            ui.spacing_mut().item_spacing = egui::vec2(0.0, 8.0);

                            for entry in filtered {
                                let selected = self.selected_file.as_ref() == Some(&entry.name);
                                let rect_height = 56.0;
                                let (rect, response) = ui.allocate_exact_size(
                                    egui::vec2(ui.available_width(), rect_height),
                                    egui::Sense::click(),
                                );

                                let bg = if selected {
                                    tokens.subtle_bg
                                } else {
                                    egui::Color32::TRANSPARENT
                                };
                                ui.painter()
                                    .rect_filled(rect, tokens.radius_md, bg);

                                let content_rect =
                                    rect.shrink2(egui::vec2(12.0, 10.0));
                                let mut row_ui = ui.child_ui(
                                    content_rect,
                                    egui::Layout::top_down(egui::Align::Min),
                                    None,
                                );

                                let title_color = if selected {
                                    tokens.text_primary
                                } else {
                                    tokens.text_muted
                                };

                                row_ui.label(
                                    egui::RichText::new(&entry.title)
                                        .italics()
                                        .color(title_color),
                                );

                                row_ui.add_space(2.0);

                                row_ui.horizontal(|ui| {
                                    let status_text = if entry.draft { "borrador" } else { "publicado" };
                                    let status_color = if entry.draft { tokens.status_draft } else { tokens.status_published };
                                    ui.label(
                                        egui::RichText::new(status_text)
                                            .small()
                                            .color(status_color),
                                    );
                                    ui.label(
                                        egui::RichText::new("·")
                                            .small()
                                            .color(tokens.text_faint),
                                    );
                                    ui.label(
                                        egui::RichText::new(&entry.date)
                                            .small()
                                            .color(tokens.text_faint),
                                    );
                                });

                                if response.clicked() {
                                    file_to_load = Some(entry.name.clone());
                                }
                            }
                        });

                    let rect = ui.max_rect();
                    ui.painter().line_segment(
                        [egui::pos2(rect.right(), rect.top()), egui::pos2(rect.right(), rect.bottom())],
                        egui::Stroke::new(1.0, tokens.divider),
                    );
                });
        }

        if should_refresh_files {
            self.refresh_files();
        }
        if should_refresh_tags {
            self.refresh_tags_cache();
        }
        if let Some(file) = file_to_load {
            self.selected_file = Some(file);
            self.load_file();
        }

        // --- PANEL DERECHO (METADATOS) ---
        let selected_coll = self.selected_collection.clone();
        let selected_file_ref = self.selected_file.clone();
        let coll_def = selected_coll.as_ref()
            .and_then(|c| self.config.collections.iter().find(|def| def.name == *c))
            .cloned()
            .or_else(|| self.config.collections.first().cloned());

        egui::SidePanel::right("right_metadata")
            .resizable(true)
            .default_width(280.0)
            .min_width(0.0)
            .frame(egui::Frame::none()
                .fill(if self.config.dark_mode {
                    egui::Color32::from_rgb(23, 23, 26)
                } else {
                    egui::Color32::from_rgb(236, 234, 231)
                })
                .inner_margin(egui::Margin::symmetric(8.0, 8.0)))
            .show_animated(ctx, self.showing_metadata && self.selected_file.is_some() && !self.focus_mode, |ui| {
                let tokens = theme::tokens_from_ui(ui);
                ui.horizontal(|ui| {
                    ui.heading("📝 Metadatos");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if components::secondary_button(ui, "🔧").on_hover_text("Recalcular rutas").clicked() { 
                            self.fix_all_image_paths(); 
                        }
                    });
                });
                ui.separator();
                
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                        if let Some(def) = &coll_def {
                            if def.fields.is_empty() {
                                self.render_generic_metadata(ui);
                                return;
                            }
                            let cur_coll = selected_coll.clone().unwrap_or_default();
                            let cur_file = selected_file_ref.clone().unwrap_or_default();
                            let content_dir = self.config.content_dir.clone();
                            let assets_dir = self.config.assets_dir.clone();
                            let repo_path = self.config.repo_path.clone();

                            let mut handled_keys = std::collections::HashSet::new();
                            
                            ui.group(|ui| {
                                ui.label(egui::RichText::new("General").strong().size(tokens.font_size_md + 2.0));
                                ui.add_space(tokens.spacing_xs + 1.0);
                                
                                for field_name in &["title", "description", "date", "publishDate", "draft"] {
                                    if let Some(field) = def.fields.iter().find(|f| f.name == *field_name) {
                                        let yaml_key = serde_yaml::Value::String(field.name.clone());
                                        handled_keys.insert(yaml_key.clone());
                                        
                                        ui.label(egui::RichText::new(&field.name).strong());
                                        
                                        if let Some(value) = self.frontmatter.get_mut(&yaml_key) {
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
                                }
                            });

                            ui.add_space(tokens.spacing_md - 2.0);
                            
                            ui.group(|ui| {
                                ui.label(egui::RichText::new("Taxonomía e Imágenes").strong().size(tokens.font_size_md + 2.0));
                                ui.add_space(tokens.spacing_xs + 1.0);

                                for field in &def.fields {
                                    let yaml_key = serde_yaml::Value::String(field.name.clone());
                                    if handled_keys.contains(&yaml_key) { continue; }
                                    handled_keys.insert(yaml_key.clone());
                                    
                                    ui.label(egui::RichText::new(&field.name).strong());
                                    if let Some(value) = self.frontmatter.get_mut(&yaml_key) {
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
                                                        if let Some(path) = utils::pick_image_file() {
                                                            if let Some(repo) = &repo_path {
                                                                if let Some(repo_rel_path) = files::copy_image_to_assets(repo, &assets_dir, &path) {
                                                                    *value = serde_yaml::Value::String(content::calculate_rel_path(&content_dir, &cur_coll, &cur_file, &repo_rel_path));
                                                                }
                                                            }
                                                        }
                                                    }
                                                });
                                            },
                                            FieldType::Categories => {
                                                if let serde_yaml::Value::Sequence(seq) = value {
                                                    ui.horizontal_wrapped(|ui| {
                                                        let mut to_remove = None;
                                                        for (i, val) in seq.iter().enumerate() {
                                                            if let Some(tag) = val.as_str() {
                                                                if components::status_badge(
                                                                    ui,
                                                                    &format!("{} ✖", tag),
                                                                    tokens.brand_primary,
                                                                )
                                                                .clicked()
                                                                {
                                                                    to_remove = Some(i);
                                                                }
                                                            }
                                                        }
                                                        if let Some(i) = to_remove {
                                                            seq.remove(i);
                                                        }
                                                    });

                                                    let mut opts: Vec<String> = if !self
                                                        .all_categories
                                                        .is_empty()
                                                    {
                                                        self.all_categories.clone()
                                                    } else if let Some(o) = &field.options {
                                                        o.clone()
                                                    } else {
                                                        Vec::new()
                                                    };
                                                    opts.sort();
                                                    opts.dedup();

                                                    if !opts.is_empty() {
                                                        egui::ComboBox::from_id_salt(format!(
                                                            "combo_{}",
                                                            field.name
                                                        ))
                                                        .selected_text("Seleccionar categoría...")
                                                        .show_ui(ui, |ui| {
                                                            for opt in opts {
                                                                if ui
                                                                    .selectable_label(
                                                                        false,
                                                                        &opt,
                                                                    )
                                                                    .clicked()
                                                                {
                                                                    if !seq.iter().any(|v| {
                                                                        v.as_str() == Some(&opt)
                                                                    }) {
                                                                        seq.push(
                                                                            serde_yaml::Value::String(
                                                                                opt.clone(),
                                                                            ),
                                                                        );
                                                                    }
                                                                }
                                                            }
                                                        });
                                                    }
                                                }
                                            },
                                            FieldType::List => {
                                                if let serde_yaml::Value::Sequence(seq) = value {
                                                    ui.horizontal_wrapped(|ui| {
                                                        let mut to_remove = None;
                                                        for (i, val) in seq.iter().enumerate() {
                                                            if let Some(tag) = val.as_str() {
                                                                if components::status_badge(
                                                                    ui,
                                                                    &format!("{} ✖", tag),
                                                                    tokens.text_muted,
                                                                )
                                                                .clicked()
                                                                {
                                                                    to_remove = Some(i);
                                                                }
                                                            }
                                                        }
                                                        if let Some(i) = to_remove { seq.remove(i); }
                                                    });
                                                    ui.horizontal(|ui| {
                                                        ui.text_edit_singleline(&mut self.new_tag_input);
                                                        if ui.button("➕").clicked() && !self.new_tag_input.trim().is_empty() {
                                                            let new_tag = self.new_tag_input.trim().to_string();
                                                            if !seq.iter().any(|v| v.as_str() == Some(&new_tag)) {
                                                                seq.push(serde_yaml::Value::String(new_tag));
                                                            }
                                                            self.new_tag_input.clear();
                                                        }
                                                    });
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
                            });

                            ui.add_space(tokens.spacing_md - 2.0);
                            ui.group(|ui| {
                                ui.label(egui::RichText::new("Componentes MDX").strong().size(tokens.font_size_md + 2.0));
                                ui.add_space(tokens.spacing_xs + 1.0);
                                if components::primary_button(ui, "📦 Importar Notice")
                                    .on_hover_text("Añade el import de Notice.astro")
                                    .clicked()
                                {
                                    let rel_path = content::calculate_rel_path(&content_dir, &cur_coll, &cur_file, "src/components/Notice.astro");
                                    let import_stmt = format!("import Notice from \"{}\";", rel_path);
                                    self.ensure_import(&import_stmt);
                                }
                                if components::secondary_button(ui, "📢 Importar CTABox")
                                    .on_hover_text("Añade el import de CTABox.astro")
                                    .clicked()
                                {
                                    let rel_path = content::calculate_rel_path(&content_dir, &cur_coll, &cur_file, "src/components/CTABox.astro");
                                    let import_stmt = format!("import CTABox from \"{}\";", rel_path);
                                    self.ensure_import(&import_stmt);
                                }
                                if components::success_button(ui, "🖼 Importar Image")
                                    .on_hover_text("Añade el import de Image de astro:assets")
                                    .clicked()
                                {
                                    self.ensure_import("import { Image } from \"astro:assets\";");
                                }
                            });
                        } else {
                            self.render_generic_metadata(ui);
                        }
                    });
                });
            });

        // --- PANEL CENTRAL ---
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.selected_file.is_some() {
                // Toolbar (hidden by default, toggled via ▼/▲)
                let is_md = self.selected_file.as_ref().map_or(false, |f| f.ends_with(".md"));
                let actions = toolbar::show_toolbar(ui, is_md, &mut self.showing_markdown_mode, self.focus_mode, self.showing_toolbar);
                
                // Process toolbar actions
                if actions.toggle_toolbar { self.showing_toolbar = !self.showing_toolbar; }
                if actions.toggle_focus_mode { self.focus_mode = !self.focus_mode; }
                if actions.convert_to_mdx { self.rename_to_mdx(); }
                if actions.insert_h1 { self.insert_at_cursor("\n\n# "); }
                if actions.insert_h2 { self.insert_at_cursor("\n\n## "); }
                if actions.insert_h3 { self.insert_at_cursor("\n\n### "); }
                if actions.insert_bold { self.pending_toolbar_action = Some(("**".to_string(), "**".to_string())); }
                else if actions.insert_italic { self.pending_toolbar_action = Some(("*".to_string(), "*".to_string())); }
                else if actions.insert_link { self.pending_toolbar_action = Some(("[".to_string(), "](url)".to_string())); }
                else { self.pending_toolbar_action = None; }
                if actions.insert_color { self.insert_replacement("<span style={{color: '#e74c3c'}}>\n", "\n</span>"); }
                if actions.insert_image { self.handle_image_insertion(); }
                if actions.insert_table { self.insert_at_cursor("\n\n| Columna 1 | Columna 2 |\n| :--- | :--- |\n| Dato 1 | Dato 2 |\n\n"); }
                if actions.insert_code { self.insert_replacement("\n```rust\n", "\n```\n"); }
                if actions.insert_youtube { self.insert_at_cursor("\n\n<lite-youtube videoid=\"ID_AQUI\" playlabel=\"Play\"></lite-youtube>\n\n"); }
                if actions.insert_cta {
                    let file_name = self.selected_file.clone().unwrap_or_default();
                    let coll = self.selected_collection.clone().unwrap_or_default();
                    let rel_path = content::calculate_rel_path(&self.config.content_dir, &coll, &file_name, "src/components/CTABox.astro");
                    self.ensure_import(&format!("import CTABox from \"{}\";", rel_path));
                    self.insert_replacement("\n\n<CTABox\n  title=\"Título\"\n  description=\"Descripción\"\n  buttonText=\"Botón\"\n  buttonHref=\"/\"\n/>\n\n", "");
                }
                if actions.insert_notice_note { self.insert_notice("note", "Nota"); }
                if actions.insert_notice_tip { self.insert_notice("tip", "Tip"); }
                if actions.insert_notice_info { self.insert_notice("info", "Info"); }
                if actions.insert_notice_warning { self.insert_notice("warning", "Aviso"); }
                if actions.insert_notice_danger { self.insert_notice("danger", "Peligro"); }
                if actions.insert_notice_success { self.insert_notice("success", "Éxito"); }

                // Subtle divider between toolbar and editor
                {
                    let tokens = theme::tokens_from_ui(ui);
                    let rect = ui.available_rect_before_wrap();
                    let y = rect.top();
                    ui.painter().line_segment(
                        [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
                        egui::Stroke::new(0.5, tokens.divider),
                    );
                    ui.add_space(1.0);
                }
                
                egui::ScrollArea::vertical().id_salt("editor_scroll").show(ui, |ui| {
                    egui::Frame::none()
                        .inner_margin(egui::Margin::symmetric(0.0, 0.0))
                        .show(ui, |ui| {
                            if self.showing_markdown_mode {
                                preview::render_body_preview(ui, &self.body, &mut self.commonmark_cache);
                            } else {
                                let editor_changed = editor::show_editor(ui, &mut self.body, &mut self.selection, &mut self.pending_selection);
                                if editor_changed { self.is_dirty = true; }
                            }
                        });
                });
                
                // Procesar acción pendiente de toolbar AHORA que selection está actualizado
                if let Some((prefix, suffix)) = self.pending_toolbar_action.take() {
                    self.apply_format(&prefix, &suffix);
                }
            } else {
                ui.centered_and_justified(|ui| {
                    if self.config.repo_path.is_none() {
                        let tokens = theme::tokens_from_ui(ui);
                        ui.vertical_centered(|ui| {
                            ui.add_space(ui.available_height() * 0.25);
                            ui.label(
                                egui::RichText::new("Welcome to Interstellar Writer")
                                    .font(egui::FontId::proportional(28.0))
                                    .color(tokens.text_primary),
                            );
                            ui.add_space(8.0);
                            ui.label(
                                egui::RichText::new("Select your Astro project to get started.")
                                    .font(egui::FontId::proportional(14.0))
                                    .color(tokens.text_muted),
                            );
                            ui.add_space(24.0);
                            if components::primary_button(ui, "Open Project Folder").clicked() {
                                if let Some(path) = utils::pick_folder() {
                                    self.set_repo_path(path);
                                    self.config.save();
                                }
                            }
                        });
                    } else if self.selected_collection.is_some() {
                        let coll_name = self.selected_collection.clone().unwrap_or_default();
                        let files_list = self.files.clone();
                        let action = dashboard::show_dashboard(ui, &coll_name, &files_list);
                        
                        match action {
                            dashboard::DashboardAction::EditFile(name) => {
                                self.selected_file = Some(name);
                                self.load_file();
                            },
                            dashboard::DashboardAction::DeleteFile(name) => {
                                self.selected_file = Some(name);
                                self.load_file();
                                self.showing_delete_confirm = true;
                            },
                            dashboard::DashboardAction::NewFile => {
                                self.showing_new_file_dialog = true;
                            },
                            dashboard::DashboardAction::Refresh => {
                                self.refresh_files();
                            },
                            dashboard::DashboardAction::None => {},
                        }
                    } else {
                        let tokens = theme::tokens_from_ui(ui);
                        ui.vertical_centered(|ui| {
                            ui.add_space(ui.available_height() * 0.3);
                            ui.label(
                                egui::RichText::new("Select a collection from the left panel")
                                    .font(egui::FontId::proportional(16.0))
                                    .color(tokens.text_muted),
                            );
                        });
                    }
                });
            }
        });

        // --- VENTANA FLOTANTE DE VISTA PREVIA ---
        if self.showing_preview && self.selected_file.is_some() && !self.focus_mode {
            let body = self.body.clone();
            preview::show_preview_window(ctx, &mut self.showing_preview, &body, &mut self.commonmark_cache);
        }

        // --- AUTOGUARDADO (backup) ---
        if self.is_dirty && self.selected_file.is_some() {
            let secs = self.config.autosave_seconds.max(1);
            let should_save = self
                .last_autosave
                .map(|t| t.elapsed().as_secs() >= secs)
                .unwrap_or(true);
            if should_save {
                self.save_backup();
                self.last_autosave = Some(std::time::Instant::now());
            }
            ctx.request_repaint_after(std::time::Duration::from_secs(secs));
        }
    }
}
