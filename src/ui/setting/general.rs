use crate::{config, ui};
use crate::ui::setting;
use crate::ui::assets::svg;

/// 並行処理数を表示
/// * `ui` - UI
/// * `config` - 設定
pub(crate) fn view(ui: &mut egui::Ui, config: &mut config::Config) {
    // テーマ選択のヘッダーパネルを表示
    setting::header_panel(ui, theme_icon(config), "Theme", None);

    // テーマ選択のフレームを表示
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.take_available_width();

        ui.horizontal(|ui| {
            if ui.radio_value(config.theme_mut(), ui::Theme::Dark, "Dark").changed() {
                ui.ctx().set_visuals(config.theme().visuals());
            }
            if ui.radio_value(config.theme_mut(), ui::Theme::Light, "Light").changed() {
                ui.ctx().set_visuals(config.theme().visuals());
            }
        });
    });

    ui.add_space(ui::SETTING_ADD_SPACING);

    // グループヘッダーパネルを表示
    setting::header_panel(ui, volume_icon(config), "Sound", None);

    // フレームを表示
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.take_available_width();

        // 効果音鳴らす？
        ui.horizontal(|ui| {
            ui::add_label(ui, "Sound effects:", ui::GENERAL_LABEL_WIDTH);
            ui.radio_value(config.play_sound_mut(), true, "Play sound");
            ui.radio_value(config.play_sound_mut(), false, "Don't play sound");
        });

        ui.add_space(ui::SETTING_ADD_SPACING);

        ui.horizontal(|ui| {
            ui::add_label(ui, "Volume:", ui::GENERAL_LABEL_WIDTH);
            ui.scope(|ui| {
                ui.spacing_mut().slider_width = setting::remaining_slider_width(ui);
                ui.add(egui::Slider::new(config.sound_volume_mut(), config::SOUND_VOLUME_MIN..=config::SOUND_VOLUME_MAX));
            });
        });
    });
}

/// テーマアイコンを取得
/// * `config` - 設定
fn theme_icon(config: &config::Config) -> egui::ImageSource<'static> {
    if *config.theme() == ui::Theme::Dark {
        svg::DARK_MODE
    } else {
        svg::LIGHT_MODE
    }
}

/// 音量アイコンを取得
/// * `config` - 設定
fn volume_icon(config: &config::Config) -> egui::ImageSource<'static> {
    if *config.play_sound() {
        if *config.sound_volume() > config::SOUND_VOLUME_MAX / 2 - 1 {
            svg::VOLUME_UP
        } else {
            svg::VOLUME_DOWN
        }
    } else {
        svg::VOLUME_OFF
    }
}
