use crate::{config, ui, event, lang};
use crate::ui::{main, assets};
use crate::ui::assets::{size, svg};

/// 上部ボタンを表示
/// * `ui` - UI
/// * `config` - 設定
/// * `pending_actions` - 処理予約されたアクション
pub(crate) fn view(
    ui: &mut egui::Ui,
    config: &config::Config,
    pending_actions: &mut Vec<event::EventAction>,
) {
    // パネルのスタイルを設定
    let top_panel_style = ui::panel_style(config, ui::TOP_PANEL_INNER_MARGIN);
    // ボタンの色を設定
    let button_color = assets::button_icon_color(config);

    // パネルを表示
    egui::Panel::top("top_taskbar").frame(top_panel_style).show(ui, |ui| {
        // 左右分割のレイアウトで、左にタイトル、右にボタンを配置する
        egui::Sides::new().shrink_left().truncate().show(ui,
            |ui| {
                ui.add(main::icon_widget(svg::COMPRESS, size::BUTTON_SETTINGS_ICON_SIZE, assets::icon_color(ui)));
                ui.label(format!("{}:", lang::current().setting_optimize.quality));

                ui.label(format!("{}:", lang::current().optimize.jpeg));
                ui.label(if *config.jpeg_lossy() {
                    lang::current().optimize.lossy.to_string()
                } else {
                    lang::current().optimize.lossless.to_string()
                });
                if *config.jpeg_lossy() {
                    ui.label(format!("({}%)", config.jpeg_quality()));
                }

                ui.label("|");

                ui.label(format!("{}:", lang::current().optimize.png));
                ui.label(if *config.png_lossy() {
                    lang::current().optimize.lossy
                } else {
                    lang::current().optimize.lossless
                });
                if *config.png_lossy() {
                    ui.label(format!("({}%)", config.png_dithering()));
                } else {
                    ui.label(format!("({})", config.png_preset()));
                }
            },
            |ui| {
                // 設定ボタン
                let settings_button = egui::Image::new(svg::SETTINGS).max_height(size::BUTTON_SETTINGS_ICON_SIZE).tint(button_color);
                if ui.button(settings_button).on_hover_text(lang::current().main_top.settings).clicked() {
                    let pos = ui.ctx().input(|input| {
                        input.viewport().outer_rect.map(|rect| rect.min)
                    });
                    pending_actions.push(event::EventAction::SettingOpen(pos));
                }

                // フォルダダイアログを開くボタン
                #[cfg(target_os = "macos")]
                {
                    let hover_text = lang::current().main_top.file_or_folder_open;
                    let open_button = egui::Image::new(svg::UPLOAD).max_height(size::BUTTON_OPEN_ICON_SIZE).tint(button_color);
                    if ui.button(open_button).on_hover_text(hover_text).clicked() {
                        pending_actions.push(event::EventAction::FolderOpen);
                    }
                }

                // フォルダダイアログを開くボタン
                #[cfg(not(target_os = "macos"))]
                {
                    let hover_text = lang::current().main_top.folder_open;
                    let open_button = egui::Image::new(svg::FOLDER_OPEN).max_height(size::BUTTON_OPEN_ICON_SIZE).tint(button_color);
                    if ui.button(open_button).on_hover_text(hover_text).clicked() {
                        pending_actions.push(event::EventAction::FolderOpen);
                    }
                }

                // ファイルダイアログを開くボタン
                // Macではフォルダダイアログでもファイルを開けるため、表示しないようにする
                #[cfg(not(target_os = "macos"))]
                {
                    let open_button = egui::Image::new(svg::FILE_OPEN).max_height(size::BUTTON_OPEN_ICON_SIZE).tint(button_color);
                    if ui.button(open_button).on_hover_text(lang::current().main_top.files_open).clicked() {
                        pending_actions.push(event::EventAction::FileOpen);
                    }
                }
            }
        );
    });
}
