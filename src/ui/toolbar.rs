//! Barra de herramientas del editor.

use eframe::egui;

use crate::ui::{components, theme};

/// Acciones que puede disparar la barra de herramientas.
#[derive(Default)]
pub struct ToolbarActions {
    pub insert_h1: bool,
    pub insert_h2: bool,
    pub insert_h3: bool,
    pub insert_bold: bool,
    pub insert_italic: bool,
    pub insert_link: bool,
    pub insert_color: bool,
    pub insert_image: bool,
    pub insert_table: bool,
    pub insert_code: bool,
    pub insert_youtube: bool,
    pub insert_cta: bool,
    pub insert_notice_note: bool,
    pub insert_notice_tip: bool,
    pub insert_notice_info: bool,
    pub insert_notice_warning: bool,
    pub insert_notice_danger: bool,
    pub insert_notice_success: bool,
    pub convert_to_mdx: bool,
    pub toggle_preview: bool,
    pub toggle_focus_mode: bool,
}

/// Renderiza la barra de herramientas del editor.
pub fn show_toolbar(
    ui: &mut egui::Ui,
    is_md_file: bool,
    showing_markdown_mode: &mut bool,
    focus_mode: bool,
) -> ToolbarActions {
    let mut actions = ToolbarActions::default();

    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;

            // Acciones de fila superior: conversión + foco + modo vista
            if is_md_file {
                if components::primary_button(ui, "Convertir a .mdx")
                    .on_hover_text("Necesario para usar componentes Astro")
                    .clicked()
                {
                    actions.convert_to_mdx = true;
                }
            }

            let focus_label = if focus_mode {
                "🔲 Salir Foco"
            } else {
                "📖 Foco"
            };
            if components::secondary_button(ui, focus_label)
                .on_hover_text("Modo enfoque (F11)")
                .clicked()
            {
                actions.toggle_focus_mode = true;
            }

            let mode_text = if *showing_markdown_mode {
                "📝 Editor"
            } else {
                "👁 Markdown"
            };
            if components::mode_toggle_button(ui, mode_text, *showing_markdown_mode).clicked() {
                *showing_markdown_mode = !*showing_markdown_mode;
                actions.toggle_preview = true;
            }
        });

        ui.add_space(2.0);

        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            ui.spacing_mut().item_spacing.y = 6.0;

            // Grupo: Encabezados
            ui.group(|ui| {
                let h_style = egui::TextStyle::Heading;
                if ui
                    .button(egui::RichText::new("H1").text_style(h_style.clone()))
                    .clicked()
                {
                    actions.insert_h1 = true;
                }
                if ui
                    .button(egui::RichText::new("H2").text_style(h_style.clone()))
                    .clicked()
                {
                    actions.insert_h2 = true;
                }
                if ui
                    .button(egui::RichText::new("H3").text_style(h_style.clone()))
                    .clicked()
                {
                    actions.insert_h3 = true;
                }
            });

            // Grupo: Formato básico
            ui.group(|ui| {
                if ui
                    .button(egui::RichText::new("B").strong())
                    .on_hover_text("Negrita")
                    .clicked()
                {
                    actions.insert_bold = true;
                }
                if ui
                    .button(egui::RichText::new("I").italics())
                    .on_hover_text("Cursiva")
                    .clicked()
                {
                    actions.insert_italic = true;
                }
                if ui.button("🔗").on_hover_text("Enlace").clicked() {
                    actions.insert_link = true;
                }
                if ui.button("🎨").on_hover_text("Color de texto").clicked() {
                    actions.insert_color = true;
                }
                if ui.button("🖼").on_hover_text("Imagen").clicked() {
                    actions.insert_image = true;
                }
                if ui.button("📊").on_hover_text("Tabla").clicked() {
                    actions.insert_table = true;
                }
            });

            // Grupo: Código y Especiales
            ui.group(|ui| {
                if ui.button("</>").on_hover_text("Código").clicked() {
                    actions.insert_code = true;
                }
                if ui.button("📺").on_hover_text("YouTube").clicked() {
                    actions.insert_youtube = true;
                }
                if ui.button("📢").on_hover_text("Anuncio (CTA)").clicked() {
                    actions.insert_cta = true;
                }
            });

            // Grupo: Avisos (Notice)
            ui.group(|ui| {
                if components::notice_icon_button(ui, "📝", theme::NoticeKind::Note, "Nota")
                    .clicked()
                {
                    actions.insert_notice_note = true;
                }

                if components::notice_icon_button(ui, "💡", theme::NoticeKind::Tip, "Tip").clicked()
                {
                    actions.insert_notice_tip = true;
                }

                if components::notice_icon_button(ui, "\u{2139}", theme::NoticeKind::Info, "Info")
                    .clicked()
                {
                    actions.insert_notice_info = true;
                }

                if components::notice_icon_button(
                    ui,
                    "\u{26A0}",
                    theme::NoticeKind::Warning,
                    "Aviso",
                )
                .clicked()
                {
                    actions.insert_notice_warning = true;
                }

                if components::notice_icon_button(
                    ui,
                    "\u{1F6AB}",
                    theme::NoticeKind::Danger,
                    "Peligro",
                )
                .clicked()
                {
                    actions.insert_notice_danger = true;
                }

                if components::notice_icon_button(
                    ui,
                    "\u{2714}",
                    theme::NoticeKind::Success,
                    "Éxito",
                )
                .clicked()
                {
                    actions.insert_notice_success = true;
                }
            });
        });
    });

    actions
}
