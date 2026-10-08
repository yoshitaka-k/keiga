use crate::event::button;
use crate::app;
use crate::ui::{main, SettingToken, OpenDialogToken};
use crate::ui::assets::{self, constants, svg};

/// 上部ボタンを表示
/// * `ui` - UI
/// * `open_dialog` - ファイルダイアログを開くタイミングをずらす
/// * `setting_token` - 設定モーダルを表示するためのトークン
pub(crate) fn view(
    ui: &mut egui::Ui,
    app: &app::App,
    open_dialog_token: &mut OpenDialogToken,
    setting_token: &mut SettingToken,
) {
    // ボタンの色を設定
    let button_color = assets::button_icon_color(ui);

    // 左右分割のレイアウトで、左にタイトル、右にボタンを配置する
    egui::Sides::new().shrink_left().truncate().show(ui,
        |ui| {
            ui.add(main::icon_widget(svg::COMPRESS, constants::BUTTON_SETTINGS_ICON_SIZE, assets::icon_color(ui)));
            ui.label("Quality: ");

            ui.label("JPEG: ");
            ui.label(format!("{}", if *app.jpeg_lossy() { "Lossy" } else { "Lossless" }));
            if *app.jpeg_lossy() {
                ui.label(format!("({}%)", app.jpeg_quality()));
            }

            ui.label("|");

            ui.label("PNG: ");
            ui.label(format!("{}", if *app.png_lossy() { "Lossy" } else { "Lossless" }));
            if *app.png_lossy() {
                ui.label(format!("({}%)", app.png_dithering()));
            } else {
                ui.label(format!("({})", app.png_preset().to_string()));
            }
        },
        |ui| {
            // 開くボタンと設定ボタンを右寄せに配置
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // 設定ボタン
                let settings_button = egui::Image::new(svg::SETTINGS).max_height(constants::BUTTON_SETTINGS_ICON_SIZE).tint(button_color);
                if ui.button(settings_button).on_hover_text("Settings").clicked() {
                    button::setting_open(ui, setting_token);
                }

                #[cfg(target_os = "macos")]
                let hover_text = "File or Folder Open";

                #[cfg(not(target_os = "macos"))]
                let hover_text = "Folder Open";

                // フォルダダイアログを開くボタン
                let open_button = egui::Image::new(svg::FOLDER_OPEN).max_height(constants::BUTTON_OPEN_ICON_SIZE).tint(button_color);
                if ui.button(open_button).on_hover_text(hover_text).clicked() {
                    button::folder_open(ui, open_dialog_token);
                }

                // ファイルダイアログを開くボタン
                // Macではフォルダダイアログでもファイルを開けるため、表示しないようにする
                #[cfg(not(target_os = "macos"))]
                {
                    let open_button = egui::Image::new(svg::FILE_OPEN).max_height(constants::BUTTON_OPEN_ICON_SIZE).tint(button_color);
                    if ui.button(open_button).on_hover_text("File Open").clicked() {
                        button::file_open(ui, open_dialog_token);
                    }
                }
            });
        }
    );
}
