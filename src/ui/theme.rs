//! Design tokens and theme system — Bear + iA Writer warm palette (v3).

use eframe::egui;

/// Visual notice types supported by the editor.
#[derive(Clone, Copy, Debug)]
pub enum NoticeKind {
    Note,
    Tip,
    Info,
    Warning,
    Danger,
    Success,
}

/// Design-system tokens derived from the v3 Pencil mockups.
#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
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
    pub panel_sidebar_bg: egui::Color32,
    pub subtle_bg: egui::Color32,
    pub border_subtle: egui::Color32,
    pub divider: egui::Color32,
    pub text_primary: egui::Color32,
    pub text_body: egui::Color32,
    pub text_muted: egui::Color32,
    pub text_faint: egui::Color32,
    pub text_default: egui::Color32,
    pub text_on_brand: egui::Color32,
    pub status_published: egui::Color32,
    pub status_draft: egui::Color32,
}

impl UiTokens {
    /// Returns tokens matching the active theme.
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
                // Keep brand_primary from the logo identity
                brand_primary: egui::Color32::from_rgb(0, 122, 204),
                brand_success: egui::Color32::from_rgb(74, 154, 106),
                brand_warning: egui::Color32::from_rgb(154, 122, 58),
                brand_danger: egui::Color32::from_rgb(180, 65, 55),
                // Warm dark palette
                panel_bg: egui::Color32::from_rgb(28, 27, 26),       // #1C1B1A
                panel_sidebar_bg: egui::Color32::from_rgb(23, 23, 26), // #17171A
                subtle_bg: egui::Color32::from_rgb(46, 44, 42),     // #2E2C2A
                border_subtle: egui::Color32::from_rgb(58, 56, 53), // #3A3835
                divider: egui::Color32::from_rgb(37, 35, 32),       // #252320
                // Typography colors — warm grays
                text_primary: egui::Color32::from_rgb(232, 228, 223), // #E8E4DF
                text_body: egui::Color32::from_rgb(197, 192, 186),    // #C5C0BA
                text_muted: egui::Color32::from_rgb(122, 117, 112),   // #7A7570
                text_faint: egui::Color32::from_rgb(90, 85, 80),      // #5A5550
                text_default: egui::Color32::from_rgb(232, 228, 223), // alias text_primary
                text_on_brand: egui::Color32::WHITE,
                status_published: egui::Color32::from_rgb(74, 154, 106), // #4A9A6A
                status_draft: egui::Color32::from_rgb(154, 122, 58),     // #9A7A3A
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
                brand_success: egui::Color32::from_rgb(58, 138, 90),
                brand_warning: egui::Color32::from_rgb(138, 122, 58),
                brand_danger: egui::Color32::from_rgb(180, 55, 45),
                // Warm light palette
                panel_bg: egui::Color32::from_rgb(245, 243, 240),       // #F5F3F0
                panel_sidebar_bg: egui::Color32::from_rgb(236, 234, 231), // #ECEAE7
                subtle_bg: egui::Color32::from_rgb(236, 234, 231),      // #ECEAE7
                border_subtle: egui::Color32::from_rgb(221, 217, 213),  // #DDD9D5
                divider: egui::Color32::from_rgb(221, 217, 213),        // same as border
                text_primary: egui::Color32::from_rgb(28, 27, 26),      // #1C1B1A
                text_body: egui::Color32::from_rgb(74, 69, 64),         // #4A4540
                text_muted: egui::Color32::from_rgb(181, 176, 171),     // #B5B0AB
                text_faint: egui::Color32::from_rgb(181, 176, 171),     // same
                text_default: egui::Color32::from_rgb(28, 27, 26),      // alias text_primary
                text_on_brand: egui::Color32::WHITE,
                status_published: egui::Color32::from_rgb(58, 138, 90),  // #3A8A5A
                status_draft: egui::Color32::from_rgb(138, 122, 58),     // #8A7A3A
            }
        }
    }

    /// Color by notice type.
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

/// Shorthand: derive tokens from the current Ui's visuals.
pub fn tokens_from_ui(ui: &egui::Ui) -> UiTokens {
    UiTokens::for_mode(ui.visuals().dark_mode)
}

/// Apply the global application theme using the warm v3 palette.
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

    // Selection: warm tones instead of VS Code cold blue
    visuals.selection.bg_fill = if dark_mode {
        egui::Color32::from_rgb(58, 56, 53) // warm #3A3835
    } else {
        egui::Color32::from_rgb(221, 217, 213) // warm #DDD9D5
    };

    visuals.panel_fill = tokens.panel_bg;
    visuals.window_fill = if dark_mode {
        egui::Color32::from_rgb(28, 27, 26) // #1C1B1A
    } else {
        egui::Color32::WHITE
    };
    visuals.widgets.noninteractive.bg_fill = tokens.panel_bg;
    visuals.widgets.inactive.bg_fill = tokens.subtle_bg;

    // Text colors: warm instead of pure gray
    visuals.widgets.noninteractive.fg_stroke.color = tokens.text_body;
    visuals.widgets.inactive.fg_stroke.color = tokens.text_muted;
    visuals.widgets.hovered.fg_stroke.color = tokens.text_primary;
    visuals.widgets.active.fg_stroke.color = tokens.text_primary;
    visuals.override_text_color = Some(tokens.text_body);

    // Separator/divider
    visuals.widgets.noninteractive.bg_stroke.width = 0.5;

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