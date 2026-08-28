use crate::event::button;
use crate::rendar::{SettingToken, OpenDialogToken};
use crate::rendar::assets::{self, constants, svg};

/// 上部ボタンを表示
/// * `ui` - UI
/// * `open_dialog` - ファイルダイアログを開くタイミングをずらす
/// * `setting_token` - 設定モーダルを表示するためのトークン
pub(crate) fn view(
    ui: &mut egui::Ui,
    open_dialog_token: &mut OpenDialogToken,
    setting_token: &mut SettingToken,
) {
    // ボタンの色を設定
    let icon_color = assets::icon_color(ui);
    let button_color = assets::button_icon_color(ui);

    ui.horizontal(|ui| {
        ui.horizontal(|ui| {
            ui.add(egui::Image::new(svg::UPLOAD_FILE).max_height(constants::TOP_MENU_UPLOAD_FILE_ICON_SIZE).tint(icon_color));
            ui.label("Folders or Files to Optimize Drag & Drop");
        });

        // 開くボタンとクリアボタンを右寄せ
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
    });
}
