use crate::file::open_files;
use crate::event::button;
use crate::rendar::SettingToken;
use crate::rendar::assets;
use crate::rendar::assets::{constants, svg};

/// 上部ボタンを表示
/// * `ui` - UI
/// * `files` - ドロップされたファイル
/// * `open_dialog` - ファイルダイアログを開くタイミングをずらす
pub(crate) fn view(
    ui: &mut egui::Ui,
    _files: &mut open_files::OpenFiles,
    open_dialog: &mut bool,
    setting_token: &mut SettingToken,
) {
    // ボタンの色を設定
    let icon_color = assets::icon_color(ui);
    let button_color = assets::button_icon_color(ui);

    ui.horizontal(|ui| {
        ui.horizontal(|ui| {
            ui.add(egui::Image::new(svg::UPLOAD_FILE).max_height(constants::UPLOAD_FILE_ICON_SIZE).tint(icon_color));
            ui.label("Folders or Files to Optimize Drag & Drop");
        });

        // 開くボタンとクリアボタンを右寄せ
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // 設定ボタン
            let settings_button = egui::Image::new(svg::SETTINGS).max_height(constants::BUTTON_SETTINGS_ICON_SIZE).tint(button_color);
            if ui.button(settings_button).on_hover_text("Settings").clicked() {
                button::setting_open(ui, setting_token);
            }

            // 開くボタン
            let open_button = egui::Image::new(svg::FOLDER_OPEN).max_height(constants::BUTTON_OPEN_ICON_SIZE).tint(button_color);
            if ui.button(open_button).on_hover_text("Files Open").clicked() {
                button::files_open(ui, open_dialog);
            }
        });
    });
}
