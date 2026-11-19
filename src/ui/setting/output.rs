use crate::{config, ui, lang};
use crate::ui::setting;
use crate::ui::assets::svg;

/// 並行処理数を表示
/// * `ui` - UI
/// * `config` - 設定
pub(crate) fn view(
    ui: &mut egui::Ui,
    config: &mut config::Config,
) {
    // グループヘッダーパネルを表示
    setting::header_panel(ui, svg::OUTPUT, lang::current().setting_output.output, None);

    // フレームを表示
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.take_available_width();

        ui.horizontal(|ui| {
            ui::add_label(ui, &format!("{}:", lang::current().setting_output.path), ui::OUTPUT_LABEL_WIDTH);
            ui.add(
                egui::TextEdit::singleline(config.output_path_mut())
                    .desired_width(ui.available_width())
                    .interactive(false),
            );
        });
        ui.horizontal(|ui| {
            ui.add_space(ui::GENERAL_LABEL_WIDTH + ui.spacing().item_spacing.x);
            if ui.button(lang::current().setting_output.browse).clicked()
                && let Some(path) = rfd::FileDialog::new().pick_folder() {
                    *config.output_path_mut() = path.to_string_lossy().into_owned();
                }
            if ui.button(lang::current().setting_output.clear).clicked() {
                config.output_path_mut().clear();
            }
        });

        // 出力パスの注意書きを表示
        setting::warning_note(ui, config, lang::current().setting_output.output_path_warning);

        ui.add_space(ui::SETTING_ADD_SPACING);
        ui.separator();
        ui.add_space(ui::SETTING_ADD_SPACING);

        // 同じパスはスキップ
        ui.horizontal(|ui| {
            ui::add_label(ui, &format!("{}:", lang::current().setting_output.skip_same_path), ui::GENERAL_LABEL_WIDTH);
            ui.radio_value(config.skip_same_path_mut(), true, lang::current().setting_output.skip_same_path_yes);
            ui.radio_value(config.skip_same_path_mut(), false, lang::current().setting_output.skip_same_path_no);
        });

        // 同じパスはスキップの注意書きを表示
        setting::warning_note(ui, config, lang::current().setting_output.skip_same_path_warning);
    });
}
