//! Sistema de tema y tokens visuales reutilizables.

use eframe::egui;

/// Tipos de avisos visuales soportados por el editor.
#[derive(Clone, Copy, Debug)]
pub enum NoticeKind {
    Note,
    Tip,
    Info,
    Warning,
    Danger,
    Success,
}

/// Tokens visuales base del sistema de diseño.
#[derive(Clone, Copy, Debug)]
pub struct UiTokens {
    pub radius_sm: f32,
    pub radius_md: f32,
    pub radius_pill: f32,
    pub spacing_xs: f32,
    pub spacing_sm: f32,
    pub spacing_md: f32,
    pub font_size_sm: f32,
    pub font_size_md: f32,
    pub font_size_lg: f32,
    pub brand_primary: egui::Color32,
    pub brand_success: egui::Color32,
    pub brand_warning: egui::Color32,
    pub brand_danger: egui::Color32,
    pub panel_bg: egui::Color32,
    pub subtle_bg: egui::Color32,
    pub border_subtle: egui::Color32,
    pub text_muted: egui::Color32,
    pub text_default: egui::Color32,
    pub text_on_brand: egui::Color32,
}

impl UiTokens {
    /// Obtiene tokens adecuados al tema activo.
    pub fn for_mode(dark_mode: bool) -> Self {
        if dark_mode {
            Self {
                radius_sm: 4.0,
                radius_md: 8.0,
                radius_pill: 12.0,
                spacing_xs: 4.0,
                spacing_sm: 8.0,
                spacing_md: 12.0,
                font_size_sm: 12.0,
                font_size_md: 14.0,
                font_size_lg: 20.0,
                brand_primary: egui::Color32::from_rgb(0, 122, 204),
                brand_success: egui::Color32::from_rgb(46, 204, 113),
                brand_warning: egui::Color32::from_rgb(241, 196, 15),
                brand_danger: egui::Color32::from_rgb(231, 76, 60),
                panel_bg: egui::Color32::from_rgb(30, 30, 30),
                subtle_bg: egui::Color32::from_rgb(45, 45, 45),
                border_subtle: egui::Color32::from_gray(70),
                text_muted: egui::Color32::from_gray(155),
                text_default: egui::Color32::from_gray(235),
                text_on_brand: egui::Color32::WHITE,
            }
        } else {
            Self {
                radius_sm: 4.0,
                radius_md: 8.0,
                radius_pill: 12.0,
                spacing_xs: 4.0,
                spacing_sm: 8.0,
                spacing_md: 12.0,
                font_size_sm: 12.0,
                font_size_md: 14.0,
                font_size_lg: 20.0,
                brand_primary: egui::Color32::from_rgb(0, 122, 204),
                brand_success: egui::Color32::from_rgb(39, 174, 96),
                brand_warning: egui::Color32::from_rgb(243, 156, 18),
                brand_danger: egui::Color32::from_rgb(192, 57, 43),
                panel_bg: egui::Color32::from_rgb(243, 243, 243),
                subtle_bg: egui::Color32::from_rgb(236, 240, 241),
                border_subtle: egui::Color32::from_gray(195),
                text_muted: egui::Color32::from_gray(110),
                text_default: egui::Color32::from_gray(30),
                text_on_brand: egui::Color32::WHITE,
            }
        }
    }

    /// Color por tipo de aviso.
    pub fn notice_color(&self, kind: NoticeKind) -> egui::Color32 {
        match kind {
            NoticeKind::Note => self.brand_primary,
            NoticeKind::Tip => egui::Color32::from_rgb(155, 89, 182),
            NoticeKind::Info => self.brand_primary,
            NoticeKind::Warning => self.brand_warning,
            NoticeKind::Danger => self.brand_danger,
            NoticeKind::Success => self.brand_success,
        }
    }
}

/// Devuelve los tokens tomando el tema del `Ui` actual.
pub fn tokens_from_ui(ui: &egui::Ui) -> UiTokens {
    UiTokens::for_mode(ui.visuals().dark_mode)
}

/// Aplica el tema global de la aplicación usando tokens del sistema.
pub fn apply_theme(ctx: &egui::Context, dark_mode: bool) {
    let tokens = UiTokens::for_mode(dark_mode);

    let mut visuals = if dark_mode {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };

    visuals.window_rounding = tokens.radius_sm.into();
    visuals.widgets.active.rounding = tokens.radius_sm.into();
    visuals.widgets.inactive.rounding = tokens.radius_sm.into();
    visuals.widgets.hovered.rounding = tokens.radius_sm.into();
    visuals.widgets.noninteractive.bg_stroke.color = tokens.border_subtle;
    visuals.widgets.inactive.bg_stroke.color = tokens.border_subtle;
    visuals.widgets.hovered.bg_stroke.color = tokens.border_subtle;
    visuals.widgets.active.bg_stroke.color = tokens.border_subtle;
    // Selección: color dedicado para cada modo para garantizar contraste con texto de sintaxis.
    // Dark mode usa slate-blue oscuro (VS Code style: #264F78);
    // light mode usa un azul pastel suave.
    visuals.selection.bg_fill = if dark_mode {
        egui::Color32::from_rgb(38, 79, 120)
    } else {
        egui::Color32::from_rgb(173, 214, 255)
    };
    visuals.panel_fill = tokens.panel_bg;
    visuals.window_fill = if dark_mode {
        egui::Color32::from_rgb(37, 37, 38)
    } else {
        egui::Color32::WHITE
    };
    visuals.widgets.noninteractive.bg_fill = tokens.panel_bg;
    visuals.widgets.inactive.bg_fill = tokens.subtle_bg;

    ctx.set_visuals(visuals);

    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(tokens.spacing_sm, tokens.spacing_sm);
    style.spacing.button_padding = egui::vec2(tokens.spacing_sm, tokens.spacing_xs + 1.0);
    style.text_styles = [
        (
            egui::TextStyle::Small,
            egui::FontId::new(tokens.font_size_sm, egui::FontFamily::Proportional),
        ),
        (
            egui::TextStyle::Body,
            egui::FontId::new(tokens.font_size_md, egui::FontFamily::Proportional),
        ),
        (
            egui::TextStyle::Button,
            egui::FontId::new(tokens.font_size_md, egui::FontFamily::Proportional),
        ),
        (
            egui::TextStyle::Monospace,
            egui::FontId::new(tokens.font_size_md, egui::FontFamily::Monospace),
        ),
        (
            egui::TextStyle::Heading,
            egui::FontId::new(tokens.font_size_lg, egui::FontFamily::Proportional),
        ),
    ]
    .into();
    ctx.set_style(style);
}