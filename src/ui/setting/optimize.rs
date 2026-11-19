use crate::{config, ui, lang};
use crate::optimize::options::PngPreset;
use crate::ui::setting;
use crate::ui::assets::svg;

/// 品質を表示
/// * `ui` - UI
/// * `config` - 設定
pub(crate) fn view(
    ui: &mut egui::Ui,
    config: &mut config::Config,
) {
    setting::header_panel(ui, svg::IMAGE, lang::current().optimize.jpeg, None);

    egui::Frame::group(ui.style()).show(ui, |ui| {
        // JPEG の圧縮方法を表示
        ui.horizontal(|ui| {
            ui::add_label(ui, &format!("{}:", lang::current().setting_optimize.compression), ui::QUALITY_LABEL_WIDTH);
            ui.scope(|ui| {
                ui.spacing_mut().slider_width = setting::remaining_slider_width(ui);
                ui.radio_value(config.jpeg_lossy_mut(), false, lang::current().optimize.lossless);
                ui.radio_value(config.jpeg_lossy_mut(), true, lang::current().optimize.lossy);
            });
        });

        // JPEG の圧縮方法の注意書きを表示
        if *config.jpeg_lossy() {
            setting::warning_note(ui, config, lang::current().setting_optimize.jpeg_lossy_warning);
        } else {
            setting::warning_note(ui, config, lang::current().setting_optimize.jpeg_lossless_warning);
        }

        // JPEG のスライダーを表示
        ui.add_enabled_ui(*config.jpeg_lossy(), |ui| {
            ui.horizontal(|ui| {
                ui::add_label(ui, &format!("{}:", lang::current().setting_optimize.quality), ui::QUALITY_LABEL_WIDTH);
                ui.scope(|ui| {
                    ui.spacing_mut().slider_width = setting::remaining_slider_width(ui);
                    ui.add(egui::Slider::new(config.jpeg_quality_mut(), config::JPEG_QUALITY_MIN..=config::JPEG_QUALITY_MAX));
                });
            });
        });
    });

    ui.add_space(ui::SETTING_ADD_SPACING);

    setting::header_panel(ui, svg::IMAGE, lang::current().optimize.png, None);

    egui::Frame::group(ui.style()).show(ui, |ui| {
        // PNG の圧縮方法を表示
        ui.horizontal(|ui| {
            ui::add_label(ui, &format!("{}:", lang::current().setting_optimize.compression), ui::QUALITY_LABEL_WIDTH);
            ui.scope(|ui| {
                ui.spacing_mut().slider_width = setting::remaining_slider_width(ui);
                ui.radio_value(config.png_lossy_mut(), false, lang::current().optimize.lossless);
                ui.radio_value(config.png_lossy_mut(), true, lang::current().optimize.lossy);
            });
        });

        // PNG の圧縮方法の注意書きを表示
        if *config.png_lossy() {
            setting::warning_note(ui, config, lang::current().setting_optimize.png_lossy_warning);
        } else {
            setting::warning_note(ui, config, lang::current().setting_optimize.png_lossless_warning);
        }

        // PNG のプリセットを表示
        ui.add_enabled_ui(!*config.png_lossy(), |ui| {
            ui.horizontal(|ui| {
                ui::add_label(ui, &format!("{}:", lang::current().setting_optimize.preset), ui::QUALITY_LABEL_WIDTH);
                ui.radio_value(config.png_preset_mut(), PngPreset::Min, PngPreset::Min.to_string());
                ui.radio_value(config.png_preset_mut(), PngPreset::Fast, PngPreset::Fast.to_string());
                ui.radio_value(config.png_preset_mut(), PngPreset::Default, PngPreset::Default.to_string());
                ui.radio_value(config.png_preset_mut(), PngPreset::Best, PngPreset::Best.to_string());
                ui.radio_value(config.png_preset_mut(), PngPreset::Max, PngPreset::Max.to_string());
            });
        });

        // PNG のディザリングのスライダーを表示
        ui.add_enabled_ui(*config.png_lossy(), |ui| {
            ui.horizontal(|ui| {
                ui::add_label(ui, &format!("{}:", lang::current().setting_optimize.dithering), ui::QUALITY_LABEL_WIDTH);
                ui.scope(|ui| {
                    ui.spacing_mut().slider_width = setting::remaining_slider_width(ui);
                    ui.add(egui::Slider::new(config.png_dithering_mut(), config::PNG_DITHERING_MIN..=config::PNG_DITHERING_MAX));
                });
            });
        });
    });
}
