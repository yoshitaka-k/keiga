use crate::{config, ui, lang};
use crate::ui::setting;
use crate::ui::assets::svg;

/// 並行処理数を表示
/// * `ui` - UI
/// * `config` - 設定
pub(crate) fn view(ui: &mut egui::Ui, config: &mut config::Config) {
    // 言語選択のヘッダーパネルを表示
    setting::header_panel(ui, theme_icon(config), lang::current().setting_general.language, None);

    // 言語選択のフレームを表示
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.take_available_width();

        ui.horizontal(|ui| {
            for lang in lang::LIST {
                if ui.radio_value(config.language_mut(), lang.code.to_string(), lang.name.to_string()).changed() {
                    lang::set(lang.code);
                }
            }
        });
    });

    ui.add_space(ui::SETTING_ADD_SPACING);

    // テーマ選択のヘッダーパネルを表示
    setting::header_panel(ui, theme_icon(config), lang::current().setting_general.theme, None);

    // テーマ選択のフレームを表示
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.take_available_width();

        ui.horizontal(|ui| {
            if ui.radio_value(config.theme_mut(), ui::Theme::Dark, ui::Theme::Dark.to_string()).changed() {
                ui.ctx().set_visuals(config.theme().visuals());
            }
            if ui.radio_value(config.theme_mut(), ui::Theme::Light, ui::Theme::Light.to_string()).changed() {
                ui.ctx().set_visuals(config.theme().visuals());
            }
        });
    });

    ui.add_space(ui::SETTING_ADD_SPACING);

    // グループヘッダーパネルを表示
    setting::header_panel(ui, volume_icon(config), lang::current().setting_general.sound, None);

    // フレームを表示
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.take_available_width();

        // 効果音鳴らす？
        ui.horizontal(|ui| {
            ui::add_label(ui, &format!("{}:", lang::current().setting_general.sound_effects), ui::GENERAL_LABEL_WIDTH);
            ui.radio_value(config.play_sound_mut(), true, lang::current().setting_general.play_sound);
            ui.radio_value(config.play_sound_mut(), false, lang::current().setting_general.dont_play_sound);
        });

        ui.add_space(ui::SETTING_ADD_SPACING);

        ui.horizontal(|ui| {
            ui::add_label(ui, &format!("{}:", lang::current().setting_general.volume), ui::GENERAL_LABEL_WIDTH);
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
