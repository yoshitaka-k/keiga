use crate::app;
use crate::rendar;
use crate::rendar::assets::svg;
use crate::rendar::setting;

/// 並行処理数を表示
/// * `ui` - UI
/// * `app` - アプリ
pub(crate) fn view(ui: &mut egui::Ui, app: &mut app::App) {
    // ヘッダーパネルを表示
    setting::header_panel(ui, svg::SETTINGS, "General", None);

    ui.add_space(setting::HEADER_BOTTOM_SPACING);

    ui.separator();

    ui.add_space(setting::SETTING_ADD_SPACING);

    // フレームを表示
    egui::Frame::default().inner_margin(rendar::PANEL_INNER_MARGIN).show(ui, |ui| {
        ui.horizontal(|ui| {
            rendar::add_label(ui, "Output path:", setting::GENERAL_LABEL_WIDTH);
            ui.add(
                egui::TextEdit::singleline(app.output_path_mut())
                    .desired_width(ui.available_width())
                    .interactive(false),
            );
        });
        ui.horizontal(|ui| {
            ui.add_space(setting::GENERAL_LABEL_WIDTH + ui.spacing().item_spacing.x);
            if ui.button("Browse").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    *app.output_path_mut() = path.to_string_lossy().into_owned();
                }
            }
            if ui.button("Clear").clicked() {
                app.output_path_mut().clear();
            }
        });

        // 出力パスの注意書きを表示
        setting::warning_note(ui, "Leave empty to overwrite the original files.");
    });

    ui.add_space(setting::SETTING_ADD_SPACING);

    ui.separator();

    ui.add_space(setting::SETTING_ADD_SPACING);

    egui::Frame::default().inner_margin(rendar::PANEL_INNER_MARGIN).show(ui, |ui| {
        // 同じパスはスキップ
        ui.horizontal(|ui| {
            rendar::add_label(ui, "Skip same path:", setting::GENERAL_LABEL_WIDTH);
            ui.radio_value(app.skip_same_path_mut(), true, "Skip same path");
            ui.radio_value(app.skip_same_path_mut(), false, "Don't skip same path");
        });

        // 同じパスはスキップの注意書きを表示
        setting::warning_note(ui, "Skip if already completed. Retry canceled and errors.");
    });

    ui.add_space(setting::SETTING_ADD_SPACING);

    ui.separator();

    ui.add_space(setting::SETTING_ADD_SPACING);

    egui::Frame::default().inner_margin(rendar::PANEL_INNER_MARGIN).show(ui, |ui| {
        // 効果音鳴らす？
        ui.horizontal(|ui| {
            rendar::add_label(ui, "Sound effects:", setting::GENERAL_LABEL_WIDTH);
            ui.radio_value(app.play_sound_mut(), true, "Play sound");
            ui.radio_value(app.play_sound_mut(), false, "Don't play sound");
        });

        ui.add_space(setting::SETTING_ADD_SPACING);

        ui.horizontal(|ui| {
            rendar::add_label(ui, "Volume:", setting::GENERAL_LABEL_WIDTH);
            ui.scope(|ui| {
                ui.spacing_mut().slider_width = setting::remaining_slider_width(ui);
                ui.add(egui::Slider::new(app.sound_volume_mut(), setting::SOUND_VOLUME_MIN..=setting::SOUND_VOLUME_MAX));
            });
        });
    });
}
