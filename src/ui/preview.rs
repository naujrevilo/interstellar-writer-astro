//! Vista previa de Markdown con renderizado de componentes.

use eframe::egui;
use crate::services::content::{clean_imports_for_preview, extract_attr};
use crate::ui::theme::{self, NoticeKind};

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
    let tokens = theme::tokens_from_ui(ui);
    let (notice_kind, icon) = match n_type {
        "tip" => (NoticeKind::Tip, "\u{1F4A1}"),      // 💡
        "warning" => (NoticeKind::Warning, "\u{26A0}"), // ⚠️
        "danger" => (NoticeKind::Danger, "\u{1F6AB}"),  // 🚫
        "success" => (NoticeKind::Success, "\u{2714}"), // ✔
        "note" => (NoticeKind::Note, "\u{1F4DD}"),      // 📝
        _ => (NoticeKind::Info, "\u{2139}"),             // ℹ
    };

    let border_color = tokens.notice_color(notice_kind);
    let bg_alpha = if ui.visuals().dark_mode { 56 } else { 24 };
    let bg_color = egui::Color32::from_rgba_premultiplied(
        border_color.r(),
        border_color.g(),
        border_color.b(),
        bg_alpha,
    );
    let text_color = tokens.text_default;

    egui::Frame::group(ui.style())
        .fill(bg_color)
        .stroke(egui::Stroke::new(1.0, border_color))
        .rounding(tokens.radius_md)
        .inner_margin(tokens.spacing_md)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(icon).size(tokens.font_size_lg).color(border_color));
                if !title.is_empty() {
                    ui.label(egui::RichText::new(title).strong().size(tokens.font_size_md + 2.0).color(text_color));
                }
            });
            ui.add_space(tokens.spacing_xs);
            
            // Contenido dentro del aviso, renderizado como Markdown
            egui_commonmark::CommonMarkViewer::new()
                .show(ui, commonmark_cache, body.trim());
        });
    ui.add_space(tokens.spacing_md - 2.0);
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
