# Interstellar Writer - Documentación de Desarrollo

> **Versión:** 1.0.0  
> **Licencia:** AGPL-3.0  
> **Autor:** Juan Oliver

## Resumen del Proyecto

**Interstellar Writer** es una aplicación de escritorio desarrollada en Rust utilizando el framework **eframe/egui** para la interfaz gráfica. Su propósito principal es gestionar contenido MDX para proyectos Astro, proporcionando una experiencia de edición visual similar a un CMS de escritorio.

### Tecnologías Principales

| Dependencia | Versión | Propósito |
|-------------|---------|-----------|
| **eframe/egui** | 0.29.1 | Framework GUI inmediato para aplicaciones nativas |
| **serde/serde_yaml** | 1.0/0.9 | Serialización y parsing de frontmatter YAML |
| **anyhow** | 1.0 | Manejo idiomático de errores con contexto |
| **rfd** | 0.15.0 | Diálogos nativos de archivos (open/save) |
| **git2** | 0.19 | Integración con repositorios Git (libgit2) |
| **confy** | 0.6 | Persistencia de configuración multiplataforma |
| **walkdir** | 2.5 | Recorrido recursivo del sistema de archivos |
| **egui_commonmark** | 0.18.0 | Renderizado de Markdown en tiempo real |

### Características Principales

- Gestión de múltiples colecciones de contenido (blog, docs, proyectos)
- Edición visual de frontmatter YAML con campos tipados
- Vista previa en tiempo real de Markdown/MDX
- Sincronización con GitHub vía Git
- Interfaz visual estilo VS Code (tema oscuro/claro)

---

## Arquitectura del Código

El proyecto sigue una arquitectura modular inspirada en el patrón **MVC (Model-View-Controller)**, separando claramente las responsabilidades:

```
src/
├── main.rs           # Punto de entrada y configuración de ventana
├── app.rs            # Estado global y ciclo de vida (Controller)
├── models.rs         # Estructuras de datos (Model)
├── utils.rs          # Funciones auxiliares
├── services/         # Lógica de negocio
│   ├── mod.rs
│   ├── content.rs    # Parsing de frontmatter/body
│   ├── files.rs      # Operaciones de sistema de archivos
│   └── git.rs        # Sincronización con GitHub
└── ui/               # Componentes visuales (View)
    ├── mod.rs
    ├── dashboard.rs  # Vista de tarjetas de publicaciones
    ├── dialogs.rs    # Ventanas modales
    ├── editor.rs     # Editor de contenido
    ├── panels.rs     # Paneles laterales
    ├── preview.rs    # Vista previa Markdown
    ├── splash.rs     # Pantalla de inicio
    └── toolbar.rs    # Barra de herramientas superior
```

### Descripción de Módulos

#### `main.rs` - Punto de Entrada

Responsabilidades mínimas:

- Declaración de módulos (`mod models`, `mod services`, etc.)
- Configuración de la ventana nativa (tamaño, icono, drag & drop)
- Inicialización del bucle de eventos de eframe

```rust
fn main() -> eframe::Result {
    let options = eframe::NativeOptions { /* ... */ };
    eframe::run_native("Interstellar Writer", options,
        Box::new(|cc| Ok(Box::new(InterstellarApp::new(cc)))),
    )
}
```

#### `models.rs` - Estructuras de Datos

Define los tipos centrales del dominio:

| Estructura | Descripción |
|------------|-------------|
| `FieldType` | Enum con tipos de campo (String, Boolean, Date, Image, List, Categories, Number) |
| `FieldDef` | Definición de un campo: nombre, tipo, valor por defecto, opciones |
| `FileEntry` | Metadatos de un archivo: nombre, título, fecha, draft, imagen |
| `CollectionDef` | Configuración de una colección con sus campos |
| `ProjectConfig` | Configuración específica de un proyecto |
| `Config` | Configuración global persistente (rutas, token GitHub, tema) |

#### `app.rs` - Estado y Controlador

Contiene `InterstellarApp`, la estructura principal que:

- Mantiene todo el estado de la aplicación
- Implementa `eframe::App` con el método `update()` para el renderizado
- Orquesta la comunicación entre UI y servicios
- Gestiona el ciclo de vida (carga, guardado, sincronización)

**Campos principales de estado:**

```rust
pub struct InterstellarApp {
    config: Config,                    // Configuración persistente
    collections: Vec<String>,          // Colecciones detectadas
    selected_collection: Option<String>,
    files: Vec<FileEntry>,             // Archivos de la colección
    selected_file: Option<String>,
    frontmatter: serde_yaml::Mapping,  // Datos YAML parseados
    body: String,                      // Contenido Markdown
    // ... flags de visibilidad de UI
}
```

#### `services/` - Lógica de Negocio

##### `content.rs`

- `parse_content(&str) -> ParsedContent`: Separa frontmatter YAML del body Markdown
- `serialize_content(mapping, body) -> String`: Reconstruye el archivo MDX

##### `files.rs`

- `scan_collections(path, content_dir) -> Vec<String>`: Lista subdirectorios
- `scan_files(path, content_dir, collection) -> Vec<FileEntry>`: Lista archivos MDX
- `read_file(...)` / `write_file(...)`: Operaciones de lectura/escritura
- `delete_file(...)`: Elimina archivos del disco

##### `git.rs`

- `sync_to_github(repo_path, message, token) -> Result<String>`: Ejecuta add, commit, push
- Manejo de autenticación vía token personal

#### `ui/` - Componentes Visuales

| Componente | Función |
|------------|---------|
| `toolbar.rs` | Barra superior con acciones globales (guardar, sync, tema) |
| `panels.rs` | Panel izquierdo (colecciones/archivos) y derecho (metadatos) |
| `dashboard.rs` | Vista de tarjetas tipo CMS para las publicaciones |
| `editor.rs` | `TextEdit` multilinea para el contenido Markdown |
| `preview.rs` | Renderizado en tiempo real con `egui_commonmark` |
| `dialogs.rs` | Modales: configuración, nuevo archivo, confirmar eliminar |
| `splash.rs` | Pantalla de bienvenida animada |

#### `utils.rs` - Utilidades

- `load_icon() -> IconData`: Carga el favicon PNG embebido
- `apply_visuals(ctx, dark_mode)`: Configura el tema visual (VS Code style)
- `pick_image_file()` / `pick_folder()`: Wrappers para diálogos nativos

---

## Diagramas

Los diagramas de arquitectura están disponibles en archivos separados para facilitar su mantenimiento y visualización:

| Diagrama | Descripción | Archivo |
|----------|-------------|---------|
| **Diagrama de Clases** | Relaciones entre `InterstellarApp`, `Config`, `FieldDef`, `FileEntry` | [class-diagram.md](diagrams/class-diagram.md) |
| **Diagrama de Flujo de Datos** | Flujo desde apertura hasta guardado/sincronización | [dataflow-diagram.md](diagrams/dataflow-diagram.md) |
| **Diagrama de Módulos** | Dependencias entre archivos del proyecto | [modules-diagram.md](diagrams/modules-diagram.md) |
| **Diagrama de Secuencia** | Interacción Usuario ↔ UI ↔ Sistema de archivos al guardar | [sequence-save.md](diagrams/sequence-save.md) |
| **Diagrama de Estados** | Ciclo de vida de un Post (Borrador → Editando → Guardado) | [state-post.md](diagrams/state-post.md) |

> **Nota:** Los archivos `.md` con bloques Mermaid se renderizan automáticamente en GitHub/GitLab.

---

## Análisis de Robustez

### Usos de `unwrap()` / `expect()` en el código

Se han identificado los siguientes puntos donde se usan métodos que pueden causar pánicos:

| Archivo | Línea | Código | Riesgo | Recomendación |
|---------|-------|--------|--------|---------------|
| `utils.rs` | 12 | `.expect("Error al cargar favicon.png")` | **Bajo** | Aceptable: el icono está embebido en tiempo de compilación |
| `ui/panels.rs` | 47 | `.clone().unwrap()` | **Medio** | Refactorizar con `if let Some(coll) = ...` |
| `app.rs` | 434 | `.file_name().unwrap()` | **Alto** | Usar `.and_then()` con fallback |
| `app.rs` | 435 | `.file_stem().unwrap()` | **Alto** | Usar `.and_then()` con fallback |
| `app.rs` | 723 | `.clone().unwrap()` | **Medio** | Ya está dentro de un bloque condicional |

### Estrategia de Mitigación

Para elevar la robustez a nivel de producción:

1. **Reemplazar `unwrap()` con manejo explícito:**

   ```rust
   // Antes (puede causar pánico)
   let name = path.file_name().unwrap().to_string_lossy();
   
   // Después (robusto)
   let name = path.file_name()
       .map(|n| n.to_string_lossy().into_owned())
       .unwrap_or_else(|| "unknown".to_string());
   ```

2. **Usar `anyhow` para propagación de errores:**

   ```rust
   use anyhow::{Context, Result};
   
   fn load_file(path: &Path) -> Result<String> {
       std::fs::read_to_string(path)
           .context(format!("Error leyendo {}", path.display()))
   }
   ```

3. **Validar entradas antes de operar:**

   ```rust
   if let Some(selected) = &self.selected_collection {
       // Operaciones seguras con `selected`
   }
   ```

---

## Testing

### Estrategia de Pruebas

El proyecto incluye tests unitarios en el módulo `services::content::tests` que cubren:

| Categoría | Tests | Cobertura |
|-----------|-------|-----------|
| **Parsing de frontmatter** | 5 | YAML válido, vacío, malformado, sin delimitadores |
| **Serialización** | 3 | Básico, eliminación de nulls, frontmatter vacío |
| **Extracción de atributos** | 4 | Encontrado, no encontrado, vacío, caracteres especiales |
| **Limpieza de imports** | 2 | Eliminación de imports, reemplazo de componentes |
| **Reparación de rutas** | 4 | String, mapping, sequence, sin assets |
| **Rutas relativas** | 1 | Cálculo básico |

### Ejecutar Tests

```bash
# Todos los tests
cargo test

# Tests específicos
cargo test content::tests

# Con output detallado
cargo test -- --nocapture

# Verificar compilación sin ejecutar
cargo test --no-run
```

### Añadir Nuevos Tests

Los tests se encuentran en cada módulo con el patrón:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nombre_descriptivo() {
        // Arrange
        let input = "...";
        
        // Act
        let result = funcion_a_probar(input);
        
        // Assert
        assert_eq!(result, expected);
    }
}
```

---

## Guía de Setup

### Requisitos Previos

- **Rust** 1.70+ (recomendado: última versión estable)
- **Git** instalado y configurado
- **Sistema operativo:** Windows, macOS o Linux

### Instalación

```bash
# 1. Clonar el repositorio
git clone https://github.com/tu-usuario/interstellar-writer-astro.git
cd interstellar-writer-astro

# 2. Compilar en modo desarrollo
cargo build

# 3. Ejecutar la aplicación
cargo run
```

### Compilación para Producción

```bash
# Compilación optimizada (sin consola en Windows)
cargo build --release

# El ejecutable estará en:
# - Windows: target/release/interstellar-writer-astro.exe
# - Linux/macOS: target/release/interstellar-writer-astro
```

### Desarrollo

```bash
# Ejecutar con logs de depuración
RUST_LOG=debug cargo run

# Verificar código sin compilar completamente
cargo check

# Ejecutar tests (si existen)
cargo test

# Formatear código
cargo fmt

# Análisis estático
cargo clippy
```

### Estructura de un Proyecto Astro Compatible

La aplicación espera la siguiente estructura en el proyecto destino:

```
mi-proyecto-astro/
├── src/
│   └── content/           # content_dir (configurable)
│       ├── blog/          # Colección "blog"
│       │   ├── post-1.mdx
│       │   └── post-2.mdx
│       └── docs/          # Colección "docs"
│           └── intro.mdx
└── public/
    └── images/            # assets_dir (configurable)
        └── covers/
```

---

## Convenciones de Código

- **Documentación:** Todos los módulos y funciones públicas incluyen `///` doc comments
- **Nombrado:** `snake_case` para funciones y variables, `PascalCase` para tipos
- **Errores:** Usar `anyhow::Result` para propagación de errores con contexto
- **UI:** Los componentes de UI reciben `&mut egui::Ui` y retornan acciones como enums

### Ejemplo de Documentación

```rust
/// Parsea el contenido de un archivo MDX.
///
/// Separa el frontmatter YAML del body Markdown.
///
/// # Argumentos
///
/// * `content` - Contenido completo del archivo
///
/// # Retorno
///
/// Estructura `ParsedContent` con frontmatter y body separados.
///
/// # Errores
///
/// No retorna errores directamente; si el YAML es inválido,
/// retorna un frontmatter vacío.
///
/// # Ejemplo
///
/// ```
/// let parsed = parse_content("---\ntitle: Test\n---\n\n# Body");
/// assert!(parsed.frontmatter.contains_key("title"));
/// ```
pub fn parse_content(content: &str) -> ParsedContent {
    // implementación
}
```

---

## Guía de Contribución

### Proceso de Contribución

1. **Fork** del repositorio en GitHub
2. **Clonar** tu fork localmente
3. **Crear rama** para tu feature:

   ```bash
   git checkout -b feature/nombre-descriptivo
   ```

4. **Desarrollar** siguiendo las convenciones de código
5. **Añadir tests** para nueva funcionalidad
6. **Verificar** que todo compila y los tests pasan:

   ```bash
   cargo fmt
   cargo clippy
   cargo test
   ```

7. **Commit** con mensajes descriptivos:

   ```bash
   git commit -m "feat: añadir soporte para campo de tipo Select"
   ```

8. **Push** y crear **Pull Request**

### Convenciones de Commits

Seguimos [Conventional Commits](https://www.conventionalcommits.org/):

| Prefijo | Uso |
|---------|-----|
| `feat:` | Nueva funcionalidad |
| `fix:` | Corrección de bug |
| `docs:` | Cambios en documentación |
| `refactor:` | Refactorización sin cambios funcionales |
| `test:` | Añadir o modificar tests |
| `chore:` | Tareas de mantenimiento |

### Checklist para Pull Requests

- [ ] El código compila sin warnings (`cargo clippy`)
- [ ] El código está formateado (`cargo fmt`)
- [ ] Los tests existentes pasan (`cargo test`)
- [ ] Se añadieron tests para nueva funcionalidad
- [ ] La documentación está actualizada
- [ ] El PR tiene una descripción clara del cambio

---

*Documentación generada para Interstellar Writer v1.0.0*
