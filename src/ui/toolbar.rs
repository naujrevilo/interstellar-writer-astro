//! Editor toolbar — hidden by default, toggled on demand.

use eframe::egui;

use crate::ui::{components, theme};

/// Actions the toolbar can trigger.
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
    pub toggle_toolbar: bool,
}

/// Render the toolbar. When `visible` is false, only a minimal toggle row is shown.
pub fn show_toolbar(
    ui: &mut egui::Ui,
    is_md_file: bool,
    showing_markdown_mode: &mut bool,
    focus_mode: bool,
    visible: bool,
) -> ToolbarActions {
    let mut actions = ToolbarActions::default();
    let tokens = theme::tokens_from_ui(ui);

    ui.vertical(|ui| {
        // Minimal top row: always visible — toggle + focus + view mode
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;

            // Toolbar visibility toggle
            let toggle_label = if visible { "Hide toolbar" } else { "Show toolbar" };
            if ui
                .add(
                    egui::Button::new(
                        egui::RichText::new(if visible { "▲" } else { "▼" })
                            .color(tokens.text_muted),
                    )
                    .fill(egui::Color32::TRANSPARENT),
                )
                .on_hover_text(toggle_label)
                .clicked()
            {
                actions.toggle_toolbar = true;
            }

            // Convert to MDX (always accessible if .md)
            if is_md_file {
                if ui
                    .add(
                        egui::Button::new(
                            egui::RichText::new(".mdx").color(tokens.brand_primary),
                        )
                        .fill(egui::Color32::TRANSPARENT),
                    )
                    .on_hover_text("Convert to .mdx for Astro components")
                    .clicked()
                {
                    actions.convert_to_mdx = true;
                }
            }

            let focus_label = if focus_mode {
                "Exit Focus"
            } else {
                "Focus"
            };
            if ui
                .add(
                    egui::Button::new(
                        egui::RichText::new(focus_label).color(if focus_mode {
                            tokens.brand_primary
                        } else {
                            tokens.text_muted
                        }),
                    )
                    .fill(egui::Color32::TRANSPARENT),
                )
                .on_hover_text("Focus mode (F11)")
                .clicked()
            {
                actions.toggle_focus_mode = true;
            }

            let mode_text = if *showing_markdown_mode {
                "Editor"
            } else {
                "Preview"
            };
            if components::mode_toggle_button(ui, mode_text, *showing_markdown_mode).clicked() {
                *showing_markdown_mode = !*showing_markdown_mode;
                actions.toggle_preview = true;
            }
        });

        // Full toolbar — only when visible
        if visible {
            ui.add_space(2.0);

            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                ui.spacing_mut().item_spacing.y = 6.0;

                // Group: Headings
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

                // Group: Basic formatting
                ui.group(|ui| {
                    if ui
                        .button(egui::RichText::new("B").strong())
                        .on_hover_text("Bold")
                        .clicked()
                    {
                        actions.insert_bold = true;
                    }
                    if ui
                        .button(egui::RichText::new("I").italics())
                        .on_hover_text("Italic")
                        .clicked()
                    {
                        actions.insert_italic = true;
                    }
                    if ui.button("Link").on_hover_text("Insert link").clicked() {
                        actions.insert_link = true;
                    }
                    if ui.button("Color").on_hover_text("Text color").clicked() {
                        actions.insert_color = true;
                    }
                    if ui.button("Image").on_hover_text("Insert image").clicked() {
                        actions.insert_image = true;
                    }
                    if ui.button("Table").on_hover_text("Insert table").clicked() {
                        actions.insert_table = true;
                    }
                });

                // Group: Code & Specials
                ui.group(|ui| {
                    if ui.button("</>").on_hover_text("Code block").clicked() {
                        actions.insert_code = true;
                    }
                    if ui.button("YT").on_hover_text("YouTube embed").clicked() {
                        actions.insert_youtube = true;
                    }
                    if ui.button("CTA").on_hover_text("Call to action").clicked() {
                        actions.insert_cta = true;
                    }
                });

                // Group: Notices
                ui.group(|ui| {
                    if components::notice_icon_button(
                        ui,
                        "Note",
                        theme::NoticeKind::Note,
                        "Note block",
                    )
                    .clicked()
                    {
                        actions.insert_notice_note = true;
                    }

                    if components::notice_icon_button(
                        ui,
                        "Tip",
                        theme::NoticeKind::Tip,
                        "Tip block",
                    )
                    .clicked()
                    {
                        actions.insert_notice_tip = true;
                    }

                    if components::notice_icon_button(
                        ui,
                        "Info",
                        theme::NoticeKind::Info,
                        "Info block",
                    )
                    .clicked()
                    {
                        actions.insert_notice_info = true;
                    }

                    if components::notice_icon_button(
                        ui,
                        "Warn",
                        theme::NoticeKind::Warning,
                        "Warning block",
                    )
                    .clicked()
                    {
                        actions.insert_notice_warning = true;
                    }

                    if components::notice_icon_button(
                        ui,
                        "Danger",
                        theme::NoticeKind::Danger,
                        "Danger block",
                    )
                    .clicked()
                    {
                        actions.insert_notice_danger = true;
                    }

                    if components::notice_icon_button(
                        ui,
                        "OK",
                        theme::NoticeKind::Success,
                        "Success block",
                    )
                    .clicked()
                    {
                        actions.insert_notice_success = true;
                    }
                });
            });
        }
    });

    actions
}
