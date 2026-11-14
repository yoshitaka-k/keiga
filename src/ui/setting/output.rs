use crate::{config, ui};
use crate::ui::setting;
use crate::ui::assets::svg;

/// 並行処理数を表示
/// * `ui` - UI
/// * `config` - 設定
pub(crate) fn view(ui: &mut egui::Ui, config: &mut config::Config) {
    // グループヘッダーパネルを表示
    setting::header_panel(ui, svg::OUTPUT, "Output", None);

    // フレームを表示
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.take_available_width();

        ui.horizontal(|ui| {
            ui::add_label(ui, "Path:", ui::OUTPUT_LABEL_WIDTH);
            ui.add(
                egui::TextEdit::singleline(config.output_path_mut())
                    .desired_width(ui.available_width())
                    .interactive(false),
            );
        });
        ui.horizontal(|ui| {
            ui.add_space(ui::GENERAL_LABEL_WIDTH + ui.spacing().item_spacing.x);
            if ui.button("Browse").clicked()
                && let Some(path) = rfd::FileDialog::new().pick_folder() {
                    *config.output_path_mut() = path.to_string_lossy().into_owned();
                }
            if ui.button("Clear").clicked() {
                config.output_path_mut().clear();
            }
        });

        // 出力パスの注意書きを表示
        setting::warning_note(ui, "Leave empty to overwrite the original files.");

        ui.add_space(ui::SETTING_ADD_SPACING);
        ui.separator();
        ui.add_space(ui::SETTING_ADD_SPACING);

        // 同じパスはスキップ
        ui.horizontal(|ui| {
            ui::add_label(ui, "Skip same path:", ui::GENERAL_LABEL_WIDTH);
            ui.radio_value(config.skip_same_path_mut(), true, "Skip same path");
            ui.radio_value(config.skip_same_path_mut(), false, "Don't skip same path");
        });

        // 同じパスはスキップの注意書きを表示
        setting::warning_note(ui, "Skip if already completed. Retry canceled and errors.");
    });
}
