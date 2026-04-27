# Changelog

Todos los cambios importantes de este proyecto se documentan en este archivo.

El formato se inspira en [Keep a Changelog](https://keepachangelog.com/es-ES/1.1.0/) y el versionado sigue [SemVer](https://semver.org/lang/es/).

## [Unreleased]

### Pendiente

- _(sin cambios todavía)_

## [1.1.0] - 2026-04-26

### Added

- Fase 1 del sistema de diseño iniciada con nuevos módulos:
  - `src/ui/theme.rs` para tokens visuales y aplicación de tema global.
  - `src/ui/components.rs` para componentes base reutilizables (botones, badges y bloques de sección).

### Changed

- `toolbar` migra botones de avisos (`Notice`) a componentes base con colores por token.
- `toolbar` separa acciones de edición y controles de vista/foco en dos filas para evitar solapamientos en anchos ajustados.
- `toolbar` agrupa `Convertir`, `Foco` y `Markdown/Editor` en la fila superior usando componentes base (`primary`, `secondary`, `toggle`).
- `dashboard` migra acciones y badges a componentes reutilizables y elimina hardcodeos visuales clave.
- `panels` migra acciones rápidas (refrescar, nueva entrada, recalcular rutas) a botones base del sistema de diseño.
- `panels` migra botones de import MDX y chips editables de categorías/listas para eliminar colores hardcodeados.
- `dialogs` migra confirmaciones y acciones de configuración a componentes base (`primary`, `secondary`, `success`, `danger`).
- Barra superior principal aplica botones base para acciones `Commit & Push`, `Guardar` y `Eliminar`.
- `app` migra paneles laterales y bloque de metadatos activo a tokens/componentes para unificar botones, chips y acciones de import MDX.
- `app` migra la barra de recuperación de borrador y breadcrumbs de archivo a tokens (`brand_primary`, `text_muted`).
- `preview` migra bloques `Notice` a paleta de tokens (`NoticeKind`, radio, espaciado y tipografía) para coherencia entre editor y vista previa.
- El theming global de la app se centraliza en `ui::theme::apply_theme`.
- El sistema de tokens incorpora escala tipográfica, color de borde sutil y color de texto base para reducir hardcodeos de estilo.

### Fixed

- Se corrige un panic de arranque en `egui` al definir `text_styles` sin `TextStyle::Monospace`.
- Se corrige el contraste de selección de texto en modo oscuro: `selection.bg_fill` usa `#264F78` (slate-blue) en dark y azul pastel en light, separado de `brand_primary` para evitar que los headings sean ilegibles al seleccionarlos.
- Se mejora el color de headings en el editor: `rgb(100,180,255)` en dark (más brillante y legible sobre fondo oscuro y sobre la selección nueva) y `rgb(0,80,180)` en light.

### Documentation

- Se actualiza `README.md` con estado final de Fase 1 y política de release por fase.
- Se actualiza `DOCS/DEVELOPMENT.md` con nuevos módulos de UI (`theme`, `components`), sección del Sistema de Diseño (tokens, colores de selección, primitivas) y tabla de robustez actualizada.
- Se actualiza `DOCS/diagrams/modules-diagram.md` para incluir `theme.rs` y `components.rs` con sus dependencias.
- Se actualiza `DOCS/diagrams/class-diagram.md` con `UiTokens` y `NoticeKind` como clases de primer nivel.
