//! Componentes base reutilizables para la UI.

use eframe::egui;

use crate::ui::theme::{self, NoticeKind};

/// Botón primario con estilo consistente.
pub fn primary_button(ui: &mut egui::Ui, label: impl Into<String>) -> egui::Response {
    let tokens = theme::tokens_from_ui(ui);
    let btn = egui::Button::new(egui::RichText::new(label.into()).color(tokens.text_on_brand))
        .fill(tokens.brand_primary)
        .rounding(tokens.radius_sm);
    ui.add(btn)
}

/// Botón secundario para acciones neutrales.
pub fn secondary_button(ui: &mut egui::Ui, label: impl Into<String>) -> egui::Response {
    let tokens = theme::tokens_from_ui(ui);
    let btn = egui::Button::new(egui::RichText::new(label.into()).color(tokens.text_default))
        .fill(tokens.subtle_bg)
        .rounding(tokens.radius_sm);
    ui.add(btn)
}

/// Botón de acción positiva (guardar/confirmar).
pub fn success_button(ui: &mut egui::Ui, label: impl Into<String>) -> egui::Response {
    let tokens = theme::tokens_from_ui(ui);
    let btn = egui::Button::new(egui::RichText::new(label.into()).color(tokens.text_on_brand))
        .fill(tokens.brand_success)
        .rounding(tokens.radius_sm);
    ui.add(btn)
}

/// Botón de acción destructiva.
pub fn danger_button(ui: &mut egui::Ui, label: impl Into<String>) -> egui::Response {
    let tokens = theme::tokens_from_ui(ui);
    let btn = egui::Button::new(egui::RichText::new(label.into()).color(tokens.text_on_brand))
        .fill(tokens.brand_danger)
        .rounding(tokens.radius_sm);
    ui.add(btn)
}

/// Botón para acciones de avisos (notice) por tipo.
pub fn notice_icon_button(
    ui: &mut egui::Ui,
    icon: &str,
    kind: NoticeKind,
    tooltip: &str,
) -> egui::Response {
    let tokens = theme::tokens_from_ui(ui);
    let btn = egui::Button::new(egui::RichText::new(icon).color(tokens.text_on_brand))
        .fill(tokens.notice_color(kind))
        .rounding(tokens.radius_sm);
    ui.add(btn).on_hover_text(tooltip)
}

/// Badge de estado para listas y tarjetas.
pub fn status_badge(ui: &mut egui::Ui, text: &str, color: egui::Color32) -> egui::Response {
    let tokens = theme::tokens_from_ui(ui);
    let btn = egui::Button::new(
        egui::RichText::new(text)
            .small()
            .strong()
            .color(tokens.text_on_brand),
    )
    .fill(color)
    .rounding(tokens.radius_pill)
    .frame(true);
    ui.add(btn)
}

/// Sección visual con heading y contenido encapsulado.
#[allow(dead_code)]
pub fn section_card(ui: &mut egui::Ui, title: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
    let tokens = theme::tokens_from_ui(ui);
    egui::Frame::group(ui.style())
        .inner_margin(egui::Margin::symmetric(
            tokens.spacing_md,
            tokens.spacing_sm,
        ))
        .rounding(tokens.radius_md)
        .show(ui, |ui| {
            ui.label(egui::RichText::new(title).strong());
            ui.add_space(tokens.spacing_sm);
            add_contents(ui);
        });
}

/// Toggle visual para modo activo/inactivo con tokens del sistema.
pub fn mode_toggle_button(
    ui: &mut egui::Ui,
    label: impl Into<String>,
    selected: bool,
) -> egui::Response {
    let tokens = theme::tokens_from_ui(ui);
    let (fill, text_color) = if selected {
        (tokens.brand_primary, tokens.text_on_brand)
    } else {
        (tokens.subtle_bg, tokens.text_default)
    };

    let btn = egui::Button::new(egui::RichText::new(label.into()).color(text_color))
        .fill(fill)
        .rounding(tokens.radius_sm);
    ui.add(btn)
}
