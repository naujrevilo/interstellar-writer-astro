//! Vista previa de Markdown con renderizado de componentes.

use eframe::egui;
use crate::services::content::{clean_imports_for_preview, extract_attr};

/// Renderiza el cuerpo del documento procesando componentes personalizados.
pub fn render_body_preview(
    ui: &mut egui::Ui, 
    content: &str,
    commonmark_cache: &mut egui_commonmark::CommonMarkCache,
) {
    let mut current_pos = 0;
    
    while let Some(start_tag_pos) = content[current_pos..].find("<Notice") {
        let absolute_start_tag = current_pos + start_tag_pos;
        
        // Renderizar texto antes del tag
        if absolute_start_tag > current_pos {
            let text_before = &content[current_pos..absolute_start_tag];
            let clean_text = clean_imports_for_preview(text_before);
            if !clean_text.trim().is_empty() {
                egui_commonmark::CommonMarkViewer::new()
                    .show(ui, commonmark_cache, &clean_text);
            }
        }
        
        // Encontrar el final del tag de apertura ">"
        if let Some(tag_end_relative) = content[absolute_start_tag..].find('>') {
            let absolute_tag_end = absolute_start_tag + tag_end_relative;
            let tag_content = &content[absolute_start_tag..absolute_tag_end];
            
            // Encontrar el tag de cierre correspondiente </Notice>
            if let Some(end_notice_relative) = content[absolute_tag_end..].find("</Notice>") {
                let absolute_notice_end = absolute_tag_end + end_notice_relative;
                let notice_body = &content[absolute_tag_end + 1 .. absolute_notice_end];
                
                let n_type = extract_attr(tag_content, "type").unwrap_or_else(|| "info".to_string());
                let n_title = extract_attr(tag_content, "title").unwrap_or_default();
                
                render_notice_ui(ui, &n_type, &n_title, notice_body, commonmark_cache);
                
                current_pos = absolute_notice_end + "</Notice>".len();
            } else {
                // Tag no cerrado, avanzar para evitar bucle
                current_pos = absolute_tag_end + 1;
            }
        } else {
            current_pos = absolute_start_tag + 1;
        }
    }
    
    // Renderizar el resto del texto
    if current_pos < content.len() {
        let remaining = &content[current_pos..];
        let clean_text = clean_imports_for_preview(remaining);
        if !clean_text.trim().is_empty() {
            egui_commonmark::CommonMarkViewer::new()
                .show(ui, commonmark_cache, &clean_text);
        }
    }
}

/// Dibuja un aviso (Notice) con estilo visual Astro/Tailwind en egui.
fn render_notice_ui(
    ui: &mut egui::Ui, 
    n_type: &str, 
    title: &str, 
    body: &str,
    commonmark_cache: &mut egui_commonmark::CommonMarkCache,
) {
    let (bg_color, border_color, text_color, icon) = match n_type {
        "tip" => (
            if ui.visuals().dark_mode { egui::Color32::from_rgba_premultiplied(88, 28, 135, 40) } else { egui::Color32::from_rgb(250, 245, 255) },
            egui::Color32::from_rgb(155, 89, 182),
            if ui.visuals().dark_mode { egui::Color32::from_rgb(233, 213, 255) } else { egui::Color32::from_rgb(88, 28, 135) },
            "\u{1F4A1}" // 💡
        ),
        "warning" => (
            if ui.visuals().dark_mode { egui::Color32::from_rgba_premultiplied(113, 63, 18, 40) } else { egui::Color32::from_rgb(254, 252, 232) },
            egui::Color32::from_rgb(241, 196, 15),
            if ui.visuals().dark_mode { egui::Color32::from_rgb(254, 249, 195) } else { egui::Color32::from_rgb(113, 63, 18) },
            "\u{26A0}" // ⚠️
        ),
        "danger" => (
            if ui.visuals().dark_mode { egui::Color32::from_rgba_premultiplied(127, 29, 29, 40) } else { egui::Color32::from_rgb(254, 242, 242) },
            egui::Color32::from_rgb(231, 76, 60),
            if ui.visuals().dark_mode { egui::Color32::from_rgb(254, 226, 226) } else { egui::Color32::from_rgb(127, 29, 29) },
            "\u{1F6AB}" // 🚫
        ),
        "success" => (
            if ui.visuals().dark_mode { egui::Color32::from_rgba_premultiplied(6, 78, 59, 40) } else { egui::Color32::from_rgb(240, 253, 244) },
            egui::Color32::from_rgb(46, 204, 113),
            if ui.visuals().dark_mode { egui::Color32::from_rgb(187, 247, 208) } else { egui::Color32::from_rgb(6, 78, 59) },
            "\u{2714}" // ✔
        ),
        "note" | "info" => (
            if ui.visuals().dark_mode { egui::Color32::from_rgba_premultiplied(30, 58, 138, 40) } else { egui::Color32::from_rgb(239, 246, 255) },
            egui::Color32::from_rgb(52, 152, 219),
            if ui.visuals().dark_mode { egui::Color32::from_rgb(191, 219, 254) } else { egui::Color32::from_rgb(30, 58, 138) },
            if n_type == "note" { "\u{1F4DD}" } else { "\u{2139}" } // 📝 or ℹ
        ),
        _ => (
            ui.visuals().widgets.noninteractive.bg_fill,
            ui.visuals().widgets.noninteractive.bg_stroke.color,
            ui.visuals().text_color(),
            "\u{2139}"
        )
    };

    egui::Frame::group(ui.style())
        .fill(bg_color)
        .stroke(egui::Stroke::new(1.0, border_color))
        .rounding(8.0)
        .inner_margin(12.0)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(icon).size(20.0).color(border_color));
                if !title.is_empty() {
                    ui.label(egui::RichText::new(title).strong().size(16.0).color(text_color));
                }
            });
            ui.add_space(4.0);
            
            // Contenido dentro del aviso, renderizado como Markdown
            egui_commonmark::CommonMarkViewer::new()
                .show(ui, commonmark_cache, body.trim());
        });
    ui.add_space(10.0);
}

/// Muestra la ventana flotante de vista previa.
pub fn show_preview_window(
    ctx: &egui::Context,
    showing: &mut bool,
    body: &str,
    commonmark_cache: &mut egui_commonmark::CommonMarkCache,
) {
    egui::Window::new("👁 Vista Previa")
        .id(egui::Id::new("preview_window"))
        .open(showing)
        .resizable(true)
        .default_width(400.0)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                render_body_preview(ui, body, commonmark_cache);
            });
        });
}
