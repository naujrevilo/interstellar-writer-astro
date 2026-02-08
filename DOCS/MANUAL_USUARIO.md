# Manual de Usuario - Interstellar Writer

Interstellar Writer es un editor de contenido especializado para proyectos Astro y MDX. Su objetivo es facilitar la creación de posts, gestión de metadatos e inserción de componentes personalizados en un entorno visual y eficiente.

---

## 1. Inicio y Dashboard

Al abrir la aplicación, verás una pantalla de carga galáctica seguida del **Dashboard**.

[CAPTURA: DASHBOARD PRINCIPAL]

### Funciones del Dashboard:
- **Tarjetas de Publicación**: Cada post se muestra como una tarjeta. Las azules son publicadas y las grises son borradores.
- **Iconos de Estado**: 🚀 indica que está publicado, 📝 indica que es un borrador.
- **Búsqueda y Filtro**: Puedes ver todas las publicaciones de la colección seleccionada.
- **➕ Nueva Publicación**: Abre un diálogo para crear un archivo. Puedes usar rutas como `posts/mi-nuevo-post` para crear subcarpetas automáticamente.

---

## 2. El Editor de Contenido

El panel central es donde ocurre la magia. Soporta dos modos:

1.  **Modo Editor**: Texto plano con resaltado para Markdown/MDX.
2.  **Modo Markdown**: Vista previa renderizada que muestra cómo se verá el contenido final.

[CAPTURA: VISTA DEL EDITOR]

### Barra de Herramientas (Botones)

Ubicada en la parte superior, cada icono realiza una función específica:

- **H1, H2, H3**: Inserta encabezados de nivel 1, 2 y 3.
- **B (Negrita)**: Envuelve el texto seleccionado en `**`.
- **I (Cursiva)**: Envuelve el texto seleccionado en `*`.
- **🔗 (Enlace)**: Inserta un enlace `[texto](url)`.
- **🎨 (Color)**: Envuelve el texto en una etiqueta `<span />` con un color específico (útil para resaltar frases).
- **🖼 (Imagen)**: Abre un selector de archivos, copia la imagen a tu carpeta de assets y la inserta como un componente `<Image />` de Astro.
- **📊 (Tabla)**: Inserta una estructura de tabla Markdown básica.
- **</> (Código)**: Inserta un bloque de código cercado con triple comilla.
- **📺 (YouTube)**: Inserta un componente de video de YouTube (lite-youtube).
- **📢 (Anuncio)**: Inserta el componente `CTABox` para llamadas a la acción.
- **Avisos (Notice)**: Botones coloreados para identificar rápidamente el tipo de aviso:
    - 📝 **Nota (Azul)**: Para aclaraciones generales.
    - 💡 **Tip (Púrpura)**: Consejos y trucos.
    - ℹ️ **Info (Azul)**: Información importante.
    - ⚠️ **Aviso (Amarillo)**: Advertencias moderadas.
    - 🛑 **Peligro (Rojo)**: Alertas críticas.
    - ✅ **Éxito (Verde)**: Confirmaciones positivas.

### Resaltado de Sintaxis
El editor ahora cuenta con resaltado visual básico:
- Los **Encabezados (#)** aparecen en azul y con un tamaño mayor.
- Las **Importaciones e inicio de archivo (---)** aparecen en gris para no distraer.
- Los **Componentes MDX (<... />)** aparecen en púrpura.

---

## 3. Panel de Metadatos (Derecha)

Aquí gestionas el **Frontmatter** de tu post.

[CAPTURA: PANEL DE METADATOS]

### Secciones principales:
- **General**: Título, descripción, estado de borrador y fechas. El icono 📅 actualiza la fecha a "ahora".
- **Taxonomía**: Categorías (vía desplegable) y etiquetas.
- **Imágenes**: Selecciona la imagen principal y la imagen para redes sociales pulsando el icono 🖼.
- **Componentes MDX**: Botones para importar manualmente los componentes necesarios. Si insertas un aviso y no se ve, pulsa **"📦 Importar Notice"**.

**Botón 🔧 (Reparar Rutas)**: Si mueves un archivo de carpeta, púlsalo para arreglar automáticamente todas las rutas de imágenes e importaciones.

---

## 4. Gestión de Archivos (Izquierda)

- **Colecciones**: Cambia entre `blog`, `docs` o cualquier otra carpeta configurada.
- **Explorador de Archivos**: Lista todos los archivos de la colección. Los iconos indican si son borradores o están listos.

---

## 5. Menú de Ayuda y Soporte

Ubicado en la barra superior, el menú **Ayuda** permite:
- **Manual de Usuario**: Abre este documento.
- **Acerca de...**: Muestra información sobre la versión de Interstellar Writer y créditos del autor.

---

## 6. Sincronización con GitHub

Pulsando el botón **"📤 Commit & Push"** en la barra superior:
1. La aplicación guarda todos los cambios locales.
2. Solicita un mensaje de commit (ej: "Añadido nuevo post sobre Rust").
3. Realiza un `git push` a tu repositorio remoto automáticamente.

[CAPTURA: DIÁLOGO DE COMMIT]

---

## Consejos Avanzados

- **Extensión .mdx**: Los componentes como `Notice` o `CTABox` solo funcionan en archivos `.mdx`. Si tu archivo es `.md`, usa el botón **"Convertir a .mdx"** en la barra de herramientas.
- **Configuración**: Puedes añadir nuevos campos o colecciones en el icono ⚙ (Configuración avanzada).
- **Modo Oscuro**: Alterna entre el tema claro y oscuro pulsando 🌙/☀️ en la barra superior.
