//! Servicio de parseo de contenido (frontmatter y body).
//!
//! Este módulo proporciona funciones para:
//! - Parsear archivos MDX/Markdown separando frontmatter YAML del body
//! - Serializar contenido de vuelta al formato MDX
//! - Manipular rutas y limpiar contenido para preview
//!
//! # Ejemplo
//!
//! ```
//! use interstellar_writer_astro::services::content::{parse_content, serialize_content};
//!
//! let content = "---\ntitle: Mi Post\n---\n\n# Hola Mundo";
//! let parsed = parse_content(content);
//! assert_eq!(parsed.body.trim(), "# Hola Mundo");
//! ```

use serde_yaml;

/// Resultado del parseo de un archivo MD/MDX.
///
/// Contiene el frontmatter deserializado como un `Mapping` YAML
/// y el cuerpo del documento como texto plano.
#[derive(Debug, Clone)]
pub struct ParsedContent {
    /// Mapa de claves-valores del frontmatter YAML.
    pub frontmatter: serde_yaml::Mapping,
    /// Contenido Markdown/MDX después del frontmatter.
    pub body: String,
}

/// Parsea el contenido de un archivo separando frontmatter (YAML) del body (Markdown).
///
/// El frontmatter debe estar delimitado por `---` al inicio del archivo.
/// Si el formato es inválido, retorna un frontmatter vacío y el contenido completo como body.
///
/// # Argumentos
///
/// * `content` - Contenido completo del archivo MDX/Markdown
///
/// # Ejemplo
///
/// ```
/// use interstellar_writer_astro::services::content::parse_content;
///
/// let content = "---\ntitle: Test\ndraft: true\n---\n\nContenido aquí";
/// let parsed = parse_content(content);
///
/// assert!(parsed.frontmatter.contains_key(&serde_yaml::Value::String("title".into())));
/// ```
pub fn parse_content(content: &str) -> ParsedContent {
    if content.starts_with("---") {
        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() == 3 {
            if let Ok(fm) = serde_yaml::from_str::<serde_yaml::Mapping>(parts[1]) {
                return ParsedContent {
                    frontmatter: fm,
                    body: parts[2].to_string(),
                };
            }
        }
    }
    ParsedContent {
        frontmatter: serde_yaml::Mapping::new(),
        body: content.to_string(),
    }
}

/// Genera el contenido final combinando frontmatter y body.
///
/// Elimina automáticamente los valores `null` del frontmatter para mantener
/// el archivo YAML limpio.
///
/// # Argumentos
///
/// * `frontmatter` - Mapping YAML con los metadatos del post
/// * `body` - Contenido Markdown/MDX
///
/// # Retorno
///
/// String con el formato MDX completo: `---\nfrontmatter\n---\n\nbody`
///
/// # Ejemplo
///
/// ```
/// use interstellar_writer_astro::services::content::serialize_content;
/// use serde_yaml::Mapping;
///
/// let mut fm = Mapping::new();
/// fm.insert("title".into(), "Mi Post".into());
/// let result = serialize_content(&fm, "# Contenido");
/// assert!(result.starts_with("---"));
/// ```
pub fn serialize_content(frontmatter: &serde_yaml::Mapping, body: &str) -> String {
    let mut clean_fm = serde_yaml::Mapping::new();
    // Eliminar valores nulos para mantener el YAML limpio
    for (k, v) in frontmatter {
        if !v.is_null() {
            clean_fm.insert(k.clone(), v.clone());
        }
    }

    if let Ok(fm_str) = serde_yaml::to_string(&clean_fm) {
        let fm_clean = fm_str.trim();
        let body_clean = body.trim_start();
        format!("---\n{}\n---\n\n{}", fm_clean, body_clean)
    } else {
        format!("---\n---\n\n{}", body.trim_start())
    }
}

/// Extrae el valor de un atributo de un tag HTML/XML.
///
/// Busca el patrón `attr="value"` dentro del tag y extrae `value`.
///
/// # Argumentos
///
/// * `tag` - String con el tag HTML/XML completo
/// * `attr` - Nombre del atributo a buscar
///
/// # Retorno
///
/// `Some(String)` con el valor del atributo, o `None` si no se encuentra.
///
/// # Ejemplo
///
/// ```
/// use interstellar_writer_astro::services::content::extract_attr;
///
/// let tag = r#"<Notice type="tip" title="Importante">"#;
/// assert_eq!(extract_attr(tag, "type"), Some("tip".to_string()));
/// assert_eq!(extract_attr(tag, "title"), Some("Importante".to_string()));
/// assert_eq!(extract_attr(tag, "missing"), None);
/// ```
pub fn extract_attr(tag: &str, attr: &str) -> Option<String> {
    let pattern = format!("{}=\"", attr);
    if let Some(start) = tag.find(&pattern) {
        let start_val = start + pattern.len();
        if let Some(end) = tag[start_val..].find('"') {
            return Some(tag[start_val..start_val + end].to_string());
        }
    }
    None
}

/// Calcula la ruta relativa desde un archivo hacia un recurso objetivo.
///
/// Útil para generar rutas de importación de assets en archivos MDX.
///
/// # Argumentos
///
/// * `content_dir` - Directorio base de contenido (ej: "src/content")
/// * `collection` - Nombre de la colección (ej: "blog")
/// * `file` - Nombre del archivo MDX (ej: "mi-post.mdx")
/// * `target_repo_rel` - Ruta del recurso objetivo relativa a la raíz del repo
///
/// # Retorno
///
/// String con la ruta relativa (ej: "../../assets/imagen.png")
pub fn calculate_rel_path(
    content_dir: &str,
    collection: &str,
    file: &str,
    target_repo_rel: &str,
) -> String {
    let mut from_path = std::path::PathBuf::from(content_dir);
    from_path.push(collection);
    from_path.push(file);

    let from_dir = from_path.parent().unwrap_or(std::path::Path::new(""));
    let to_path = std::path::Path::new(target_repo_rel);

    let from_comps: Vec<_> = from_dir.components().collect();
    let to_comps: Vec<_> = to_path.components().collect();

    let mut common_count = 0;
    for (f, t) in from_comps.iter().zip(to_comps.iter()) {
        if f == t {
            common_count += 1;
        } else {
            break;
        }
    }

    let mut rel = String::new();
    for _ in 0..(from_comps.len() - common_count) {
        rel.push_str("../");
    }

    // Si no hay que subir niveles, empezamos con ./ para que sea una ruta relativa válida en JS
    if rel.is_empty() {
        rel.push_str("./");
    }

    for i in common_count..to_comps.len() {
        let comp_str = to_comps[i].as_os_str().to_string_lossy();
        if !rel.ends_with('/') {
            rel.push('/');
        }
        rel.push_str(&comp_str);
    }

    rel.replace("\\", "/")
}

/// Limpia los imports de un texto para la vista previa.
///
/// Elimina las líneas de import y reemplaza componentes de Astro
/// por texto placeholder para una mejor visualización en el preview.
///
/// # Transformaciones
///
/// - Elimina líneas que empiezan con `import `
/// - Reemplaza `<CTABox` por placeholder de anuncio
/// - Reemplaza `<Image` por placeholder de imagen
///
/// # Argumentos
///
/// * `text` - Contenido MDX con imports y componentes
///
/// # Retorno
///
/// String limpio sin imports y con placeholders
pub fn clean_imports_for_preview(text: &str) -> String {
    let mut clean = text
        .lines()
        .filter(|l| !l.trim().starts_with("import "))
        .collect::<Vec<_>>()
        .join("\n");

    // Reemplazos para componentes que no tienen renderizado nativo
    clean = clean.replace("<CTABox", "\n--- \n> \u{1F4E2} **ANUNCIO (CTA)**\n");
    clean = clean.replace("<Image", "\n--- \n> \u{1F5BC} **IMAGEN DE ASTRO**\n");
    clean = clean.replace("/>", "\n--- \n");

    clean
}

/// Repara recursivamente las rutas de assets en valores YAML.
///
/// Busca referencias a `assets/` y las prefija con el número correcto
/// de `../` para que las rutas relativas funcionen desde el archivo MDX.
///
/// # Argumentos
///
/// * `value` - Valor YAML a procesar (puede ser String, Mapping o Sequence)
/// * `needed_dots` - Prefijo de ruta relativa (ej: "../../")
///
/// # Ejemplo
///
/// Si `value` contiene `"assets/img.png"` y `needed_dots` es `"../../"`,
/// el valor se transforma a `"../../assets/img.png"`.
pub fn fix_dots_recursively(value: &mut serde_yaml::Value, needed_dots: &str) {
    match value {
        serde_yaml::Value::String(s) => {
            if let Some(pos) = s.find("assets/") {
                let subpath = &s[pos..]; // "assets/..."
                *s = format!("{}{}", needed_dots, subpath);
            }
        }
        serde_yaml::Value::Mapping(m) => {
            for (_, v) in m.iter_mut() {
                fix_dots_recursively(v, needed_dots);
            }
        }
        serde_yaml::Value::Sequence(s) => {
            for v in s.iter_mut() {
                fix_dots_recursively(v, needed_dots);
            }
        }
        _ => {}
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Tests para parse_content
    // -------------------------------------------------------------------------

    #[test]
    fn test_parse_content_valid_frontmatter() {
        let content = "---\ntitle: Mi Post\ndraft: true\n---\n\n# Contenido";
        let parsed = parse_content(content);

        assert_eq!(
            parsed.frontmatter.get("title").and_then(|v| v.as_str()),
            Some("Mi Post")
        );
        assert_eq!(
            parsed.frontmatter.get("draft").and_then(|v| v.as_bool()),
            Some(true)
        );
        assert!(parsed.body.contains("# Contenido"));
    }

    #[test]
    fn test_parse_content_empty_frontmatter() {
        let content = "---\n---\n\nSolo body";
        let parsed = parse_content(content);

        assert!(parsed.frontmatter.is_empty());
        assert!(parsed.body.contains("Solo body"));
    }

    #[test]
    fn test_parse_content_no_frontmatter() {
        let content = "# Título\n\nContenido sin frontmatter";
        let parsed = parse_content(content);

        assert!(parsed.frontmatter.is_empty());
        assert_eq!(parsed.body, content);
    }

    #[test]
    fn test_parse_content_malformed_yaml() {
        // YAML inválido: debería retornar frontmatter vacío
        let content = "---\ntitle: [invalid yaml\n---\n\nBody";
        let parsed = parse_content(content);

        assert!(parsed.frontmatter.is_empty());
        // El body contiene todo porque el parsing falló
        assert!(parsed.body.contains("title"));
    }

    #[test]
    fn test_parse_content_single_delimiter() {
        // Solo un delimitador, no es frontmatter válido
        let content = "---\nNo es frontmatter válido";
        let parsed = parse_content(content);

        assert!(parsed.frontmatter.is_empty());
    }

    // -------------------------------------------------------------------------
    // Tests para serialize_content
    // -------------------------------------------------------------------------

    #[test]
    fn test_serialize_content_basic() {
        let mut fm = serde_yaml::Mapping::new();
        fm.insert("title".into(), "Test".into());
        fm.insert("draft".into(), serde_yaml::Value::Bool(false));

        let result = serialize_content(&fm, "# Body");

        assert!(result.starts_with("---"));
        assert!(result.contains("title: Test"));
        assert!(result.contains("# Body"));
    }

    #[test]
    fn test_serialize_content_removes_nulls() {
        let mut fm = serde_yaml::Mapping::new();
        fm.insert("title".into(), "Test".into());
        fm.insert("image".into(), serde_yaml::Value::Null);

        let result = serialize_content(&fm, "Body");

        assert!(result.contains("title"));
        assert!(!result.contains("image"));
    }

    #[test]
    fn test_serialize_content_empty_frontmatter() {
        let fm = serde_yaml::Mapping::new();
        let result = serialize_content(&fm, "Solo body");

        assert!(result.starts_with("---"));
        assert!(result.contains("Solo body"));
    }

    // -------------------------------------------------------------------------
    // Tests para extract_attr
    // -------------------------------------------------------------------------

    #[test]
    fn test_extract_attr_found() {
        let tag = r#"<Notice type="tip" title="Importante">"#;

        assert_eq!(extract_attr(tag, "type"), Some("tip".to_string()));
        assert_eq!(extract_attr(tag, "title"), Some("Importante".to_string()));
    }

    #[test]
    fn test_extract_attr_not_found() {
        let tag = r#"<Notice type="tip">"#;
        assert_eq!(extract_attr(tag, "missing"), None);
    }

    #[test]
    fn test_extract_attr_empty_value() {
        let tag = r#"<Notice type="">"#;
        assert_eq!(extract_attr(tag, "type"), Some("".to_string()));
    }

    #[test]
    fn test_extract_attr_special_chars() {
        let tag = r#"<Component data="hello-world_123">"#;
        assert_eq!(
            extract_attr(tag, "data"),
            Some("hello-world_123".to_string())
        );
    }

    // -------------------------------------------------------------------------
    // Tests para clean_imports_for_preview
    // -------------------------------------------------------------------------

    #[test]
    fn test_clean_imports_removes_import_lines() {
        let text = "import Component from './comp';\nimport Other from 'other';\n\n# Title";
        let result = clean_imports_for_preview(text);

        assert!(!result.contains("import"));
        assert!(result.contains("# Title"));
    }

    #[test]
    fn test_clean_imports_replaces_components() {
        let text = "# Title\n<CTABox>content</CTABox>\n<Image src='img.png' />";
        let result = clean_imports_for_preview(text);

        assert!(result.contains("ANUNCIO"));
        assert!(result.contains("IMAGEN"));
    }

    // -------------------------------------------------------------------------
    // Tests para fix_dots_recursively
    // -------------------------------------------------------------------------

    #[test]
    fn test_fix_dots_string() {
        let mut value = serde_yaml::Value::String("assets/image.png".to_string());
        fix_dots_recursively(&mut value, "../../");

        assert_eq!(value.as_str(), Some("../../assets/image.png"));
    }

    #[test]
    fn test_fix_dots_no_assets() {
        let mut value = serde_yaml::Value::String("other/path.png".to_string());
        fix_dots_recursively(&mut value, "../../");

        assert_eq!(value.as_str(), Some("other/path.png"));
    }

    #[test]
    fn test_fix_dots_mapping() {
        let mut mapping = serde_yaml::Mapping::new();
        mapping.insert("image".into(), "assets/cover.jpg".into());
        mapping.insert("title".into(), "Test".into());

        let mut value = serde_yaml::Value::Mapping(mapping);
        fix_dots_recursively(&mut value, "../");

        if let serde_yaml::Value::Mapping(m) = value {
            assert_eq!(
                m.get("image").and_then(|v| v.as_str()),
                Some("../assets/cover.jpg")
            );
            assert_eq!(m.get("title").and_then(|v| v.as_str()), Some("Test"));
        } else {
            panic!("Expected mapping");
        }
    }

    #[test]
    fn test_fix_dots_sequence() {
        let seq = vec![
            serde_yaml::Value::String("assets/img1.png".to_string()),
            serde_yaml::Value::String("assets/img2.png".to_string()),
        ];
        let mut value = serde_yaml::Value::Sequence(seq);
        fix_dots_recursively(&mut value, "../../");

        if let serde_yaml::Value::Sequence(s) = value {
            assert_eq!(s[0].as_str(), Some("../../assets/img1.png"));
            assert_eq!(s[1].as_str(), Some("../../assets/img2.png"));
        } else {
            panic!("Expected sequence");
        }
    }

    // -------------------------------------------------------------------------
    // Tests para calculate_rel_path
    // -------------------------------------------------------------------------

    #[test]
    fn test_calculate_rel_path_basic() {
        let result = calculate_rel_path("src/content", "blog", "post.mdx", "src/assets/image.png");
        // Desde src/content/blog/ hacia src/assets/
        assert!(result.contains(".."));
        assert!(result.contains("assets"));
    }
}
