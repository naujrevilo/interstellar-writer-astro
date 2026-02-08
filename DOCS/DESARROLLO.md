# Documentación de Desarrollo - Interstellar Writer

Interstellar Writer está construido utilizando el lenguaje de programación **Rust** y el framework de interfaz gráfica **egui** (vía `eframe`). Esta arquitectura permite una interfaz fluida, un consumo mínimo de recursos y una integración nativa con el sistema de archivos y Git.

## Requisitos de Desarrollo

- **Rust**: Mínimo v1.75 (recomendado estable).
- **Cargo**: Gestor de paquetes de Rust.
- **Git**: Para las funciones de sincronización (utiliza la librería `git2`).
- **Dependencias del Sistema**: En Linux, se requieren librerías de desarrollo para `libssh2` y `openssl`.

## Estructura del Proyecto

- `src/main.rs`: Núcleo de la aplicación. Contiene el estado (`InterstellarApp`), la lógica de UI y el manejo de eventos.
- `Cargo.toml`: Definición de dependencias, como `egui`, `serde_yaml`, `git2`, e `image`.
- `DOCS/`: Carpeta con manuales para usuario y desarrollador.
- `favicon.svg`: Icono vectorial original del proyecto.
- `favicon.png`: Icono de la aplicación (256x256px), generado desde el SVG usando `resvg`.

## Arquitectura Técnica

### 1. El Estado Global (`InterstellarApp`)
La aplicación utiliza el patrón de **UI Inmediata**, lo que significa que la interfaz se redibuja en cada frame (normalmente a 60 FPS) basándose en el estado actual.

```rust
struct InterstellarApp {
    config: Config,               // Configuración persistente
    body: String,                 // Contenido del editor
    frontmatter: Mapping,         // Datos YAML parseados
    splash_start_time: Option<Instant>, // Control de pantalla de inicio
    // ... otros campos de estado
}
```

### 2. Gestión de Inserciones y Estilo
Para insertar componentes o formato, se utiliza el método `insert_replacement`. Además, el editor utiliza un **Layouter personalizado** en el `TextEdit` para proporcionar resaltado de sintaxis básico (headers, components, frontmatter) sin penalizar el rendimiento.

```rust
fn insert_replacement(&mut self, before: &str, after: &str) {
    let (start, end) = self.selection.unwrap_or_default();
    // Lógica para envolver el texto seleccionado o insertar en el cursor
    // ...
    self.pending_selection = Some((new_pos, new_pos)); // Forzar cursor
}
```

### 3. Sincronización con Git (Multihilo)
Las operaciones de red (fetch, push) se realizan en un hilo separado para no bloquear la interfaz. Se utiliza un canal (`mpsc`) para comunicar el resultado a la UI.

```rust
std::thread::spawn(move || {
    let result = (|| -> anyhow::Result<String> {
        let repo = Repository::open(&path)?;
        // ... operaciones de git2 ...
        Ok("Sincronizado".to_string())
    })();
    let _ = tx.send(result);
});
```

## Conceptos Clave

### Gestión del Frontmatter
El editor separa el contenido de los archivos en dos partes utilizando delimitadores `---`. 
- Al **cargar**: Se usa `serde_yaml` para convertir el bloque superior en un mapa manipulable.
- Al **guardar**: Se reconstruye el archivo uniendo el YAML con el cuerpo, asegurando que haya exactamente una línea en blanco entre ellos para evitar errores en Astro/MDX.

### Sistema de Rutas Relativas
La función `calculate_rel_path` es crucial. Astro requiere rutas relativas precisas (`../../assets/...`) para resolver imágenes e imports. El editor calcula estas rutas dinámicamente comparando la ubicación del post con la del recurso.

## Cómo Extender el Editor

### Añadir un nuevo tipo de Aviso (Notice)
1. Asegúrate de que tu componente `Notice.astro` soporte el nuevo tipo.
2. En `src/main.rs`, añade un botón en la barra de herramientas:
   ```rust
   if ui.button("⭐").clicked() { self.insert_notice("special", "Especial"); }
   ```

### Añadir un nuevo campo al Frontmatter
1. Modifica la configuración en el diálogo de configuración avanzada de la app.
2. Los campos nuevos aparecerán automáticamente en el panel de Metadatos gracias a la iteración dinámica sobre `def.fields`.

## Compilación y Release

Para generar un ejecutable optimizado sin consola en Windows:

```bash
cargo build --release
```

El binario resultante se encontrará en `target/release/interstellar-writer-astro.exe`.

### Configuración del Icono (Favicon)

La aplicación incluye un icono personalizado que aparece en la barra de título y la barra de tareas:

1. **Icono fuente**: `favicon.svg` - Diseño vectorial original con círculos concéntricos en azul oscuro (#262262), azul (#1c75bc), rojo (#b52733) y amarillo (#e68e27).

2. **Conversión a PNG**: El icono se convierte a formato PNG usando `resvg`:
   ```bash
   cargo install resvg
   resvg favicon.svg -w 256 -h 256 favicon.png
   ```

3. **Carga en la aplicación**: La función `load_icon()` en `src/main.rs` carga el PNG en tiempo de compilación usando `include_bytes!()` y lo convierte a `egui::IconData` con la librería `image`.

4. **Si necesitas actualizar el icono**:
   - Modifica `favicon.svg`
   - Regenera el PNG con el comando `resvg` mostrado arriba
   - Recompila la aplicación
