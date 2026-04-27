# 🚀 Interstellar Writer

**Editor de escritorio para contenido MDX/Astro** — Gestiona tu blog o documentación de proyectos Astro con una interfaz visual intuitiva.

![Rust](https://img.shields.io/badge/Rust-1.70+-orange?logo=rust)
![License](https://img.shields.io/badge/License-AGPL--3.0-blue)
![Platform](https://img.shields.io/badge/Platform-Windows%20|%20macOS%20|%20Linux-lightgrey)

---

## ✨ Características

- 📁 **Gestión de colecciones**: Organiza contenido en carpetas (blog, docs, proyectos)
- 📝 **Editor visual de Frontmatter**: Campos tipados (texto, fecha, booleano, lista, imagen)
- 👁️ **Vista previa en tiempo real**: Renderizado Markdown con CommonMark
- 🔄 **Sincronización con GitHub**: Commit y push directamente desde la app
- 🎨 **Tema oscuro/claro**: Interfaz inspirada en VS Code
- 📊 **Dashboard de publicaciones**: Vista de tarjetas estilo CMS
- 🎛️ **Sistema de diseño base (Fase 1)**: Tokens visuales y componentes reutilizables para consistencia UI

---

## 🛠️ Stack Tecnológico

| Tecnología | Versión | Propósito |
|------------|---------|-----------|
| **Rust** | 1.70+ | Lenguaje de programación |
| **eframe/egui** | 0.29.1 | Framework GUI nativo |
| **serde** | 1.0 | Serialización/deserialización |
| **serde_yaml** | 0.9 | Parsing de Frontmatter YAML |
| **git2** | 0.19 | Integración con Git (libgit2) |
| **anyhow** | 1.0 | Manejo de errores |
| **rfd** | 0.15 | Diálogos nativos de archivos |
| **egui_commonmark** | 0.18 | Renderizado Markdown |

---

## 📦 Instalación

### Requisitos previos

- [Rust](https://rustup.rs/) 1.70 o superior
- Git instalado y configurado

#### Windows

- Visual Studio Build Tools con **C++ Desktop Development**

#### macOS

- Xcode Command Line Tools:

  ```bash
  xcode-select --install
  ```

#### Linux (Debian/Ubuntu)

```bash
sudo apt update
sudo apt install -y build-essential libxcb-render0-dev libxcb-shape0-dev \
  libxcb-xfixes0-dev libxkbcommon-dev libssl-dev pkg-config
```

#### Linux (Fedora)

```bash
sudo dnf install gcc gcc-c++ libxcb-devel libxkbcommon-devel openssl-devel
```

#### Linux (Arch)

```bash
sudo pacman -S base-devel libxcb libxkbcommon openssl
```

### Desde código fuente

```bash
# 1. Clonar el repositorio
git clone https://github.com/tu-usuario/interstellar-writer-astro.git
cd interstellar-writer-astro

# 2. Compilar y ejecutar
cargo run

# 3. (Opcional) Compilar para producción
cargo build --release
```

El ejecutable estará en:

- **Windows**: `target/release/interstellar-writer-astro.exe`
- **Linux/macOS**: `target/release/interstellar-writer-astro`

---

## 🚀 Uso Rápido

### 1. Configurar un proyecto

Al iniciar la aplicación por primera vez:

1. Click en **⚙️ Configuración**
2. Selecciona la carpeta raíz de tu proyecto Astro
3. (Opcional) Añade tu token de GitHub para sincronización
4. Guarda la configuración

### 2. Gestionar contenido

```
┌─────────────────────────────────────────────────────────┐
│  📂 Colecciones  │        Editor / Preview              │
│  ----------------│                                       │
│  > blog          │  ---                                  │
│  > docs          │  title: Mi Post                       │
│  > projects      │  date: 2024-01-15                     │
│                  │  draft: false                         │
│  📄 Archivos     │  ---                                  │
│  ----------------│                                       │
│  ✅ post-1.mdx   │  # Contenido del post                 │
│  📝 borrador.mdx │                                       │
│                  │  Escribe aquí tu Markdown...          │
└─────────────────────────────────────────────────────────┘
```

### 3. Sincronizar con GitHub

1. Edita y guarda tus archivos (💾)
2. Click en **🔄 Sync**
3. Introduce un mensaje de commit
4. ¡Listo! Tus cambios están en el repositorio remoto

---

## 📁 Estructura de Proyecto Esperada

La aplicación funciona con proyectos Astro que sigan esta estructura:

```
mi-proyecto-astro/
├── src/
│   └── content/           # Directorio de contenido (configurable)
│       ├── blog/          # Colección "blog"
│       │   ├── post-1.mdx
│       │   └── post-2.mdx
│       └── docs/          # Colección "docs"
│           └── intro.mdx
└── public/
    └── images/            # Directorio de assets (configurable)
```

---

## 🧪 Testing

```bash
# Ejecutar todos los tests
cargo test

# Ejecutar tests con output detallado
cargo test -- --nocapture

# Verificar código sin ejecutar
cargo check
```

---

## 📚 Documentación

- [DEVELOPMENT.md](DOCS/DEVELOPMENT.md) — Documentación técnica y arquitectura
- [MANUAL_USUARIO.md](DOCS/MANUAL_USUARIO.md) — Guía de usuario
- [CHANGELOG.md](CHANGELOG.md) — Historial de cambios y versionado

### Estado de roadmap

- ✅ Base de editor/gestión de contenido estable
- ✅ **Fase 1 completada:** tokens de diseño + componentes base aplicados en toolbar, dashboard, paneles, diálogos, app activa y preview.
- ⏭️ Próximo hito: **Fase 2** (refactor visual integral de editor/paneles con jerarquía y densidad mejorada)

### To-Do próximas fases

#### Fase 2 — Refinamiento visual y consistencia avanzada

- [ ] Rediseñar jerarquía visual de editor y paneles (espaciado, tipografías y agrupación por contexto).
- [ ] Homogeneizar estados interactivos (hover, active, selected, disabled) en toda la app.
- [ ] Mejorar accesibilidad de contraste en dark/light para texto, selección y badges.
- [ ] Eliminar hardcodeos residuales de color/estilo fuera de `theme.rs` y `components.rs`.

#### Fase 3 — Productividad de edición

- [ ] Completar flujo de atajos de teclado en acciones frecuentes (guardar, preview, foco, comando rápido).
- [ ] Mejorar navegación entre colección/archivo con feedback contextual y menor fricción.
- [ ] Fortalecer manejo de borradores y recuperación ante cierre inesperado.
- [ ] Mejorar indicadores de estado (cambios sin guardar, sync pendiente, errores de validación).

#### Fase 4 — Calidad, distribución y operación

- [ ] Expandir cobertura de pruebas para servicios, parsing de contenido y rutas críticas de UI.
- [ ] Estandarizar checklist de release (versionado, changelog, artefactos, notas de publicación).
- [ ] Automatizar validaciones previas a release (lint/check/tests) en CI.
- [ ] Sincronizar documentación técnica y diagramas con cada corte de versión.

### Diagramas de Arquitectura

| Diagrama | Descripción |
|----------|-------------|
| [Clases](DOCS/diagrams/class-diagram.md) | Relaciones entre estructuras |
| [Flujo de datos](DOCS/diagrams/dataflow-diagram.md) | Flujo de la aplicación |
| [Módulos](DOCS/diagrams/modules-diagram.md) | Dependencias entre archivos |
| [Secuencia - Guardar](DOCS/diagrams/sequence-save.md) | Proceso de guardado |
| [Estados - Post](DOCS/diagrams/state-post.md) | Ciclo de vida de un post |

---

## 🤝 Contribución

1. Fork del repositorio
2. Crea una rama: `git checkout -b feature/nueva-funcionalidad`
3. Haz commits atómicos con mensajes descriptivos
4. Abre un Pull Request hacia `main`

Ver [DEVELOPMENT.md](DOCS/DEVELOPMENT.md) para convenciones de código.

### Política de release por fase

Cada fase aprobada incluye obligatoriamente:

- actualización de documentación (`README`, `DOCS/`),
- actualización de `CHANGELOG.md`,
- actualización de versión (SemVer) al momento de corte de release,
- sincronización de artefactos públicos (GitHub release y landing page).

---

## 📄 Licencia

Este proyecto está bajo la licencia **AGPL-3.0**. Ver [LICENSE](LICENSE) para más detalles.

---

## 👤 Autor

**Juan Oliver**

---

*Hecho con ❤️ y Rust*
