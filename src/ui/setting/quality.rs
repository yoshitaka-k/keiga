use crate::{config, ui};
use crate::optimize::options::PngPreset;
use crate::ui::setting;
use crate::ui::assets::svg;

/// 品質を表示
/// * `ui` - UI
/// * `config` - 設定
pub(crate) fn view(ui: &mut egui::Ui, config: &mut config::Config) {
    // ヘッダーパネルを表示
    setting::header_panel(ui, svg::COMPRESS, "Quality", None);

    ui.add_space(ui::HEADER_BOTTOM_SPACING);

    ui.separator();

    ui.add_space(ui::SETTING_ADD_SPACING);

    setting::header_panel(ui, svg::IMAGE, "JPEG", None);

    egui::Frame::group(ui.style()).show(ui, |ui| {
        // JPEG の圧縮方法を表示
        ui.horizontal(|ui| {
            ui::add_label(ui, "Compression:", ui::QUALITY_LABEL_WIDTH);
            ui.scope(|ui| {
                ui.spacing_mut().slider_width = setting::remaining_slider_width(ui);
                ui.radio_value(config.jpeg_lossy_mut(), false, "Lossless");
                ui.radio_value(config.jpeg_lossy_mut(), true, "Lossy");
            });
        });

        // JPEG の圧縮方法の注意書きを表示
        if *config.jpeg_lossy() {
            setting::warning_note(ui, "Re-encodes at the quality below..");
        } else {
            setting::warning_note(ui, "losNear-lossless re-encode. Still lossy; may not shrink.");
        }

        // JPEG のスライダーを表示
        ui.add_enabled_ui(*config.jpeg_lossy(), |ui| {
            ui.horizontal(|ui| {
                ui::add_label(ui, "Quality:", ui::QUALITY_LABEL_WIDTH);
                ui.scope(|ui| {
                    ui.spacing_mut().slider_width = setting::remaining_slider_width(ui);
                    ui.add(egui::Slider::new(config.jpeg_quality_mut(), config::JPEG_QUALITY_MIN..=config::JPEG_QUALITY_MAX));
                });
            });
        });
    });

    ui.add_space(ui::SETTING_ADD_SPACING);

    setting::header_panel(ui, svg::IMAGE, "PNG", None);

    egui::Frame::group(ui.style()).show(ui, |ui| {
        // PNG の圧縮方法を表示
        ui.horizontal(|ui| {
            ui::add_label(ui, "Compression:", ui::QUALITY_LABEL_WIDTH);
            ui.scope(|ui| {
                ui.spacing_mut().slider_width = setting::remaining_slider_width(ui);
                ui.radio_value(config.png_lossy_mut(), false, "Lossless");
                ui.radio_value(config.png_lossy_mut(), true, "Lossy");
            });
        });

        // PNG の圧縮方法の注意書きを表示
        if *config.png_lossy() {
            setting::warning_note(ui, "Reduces to 256 colors. Dithering below.");
        } else {
            setting::warning_note(ui, "Pixels unchanged. Uses the preset below.");
        }

        // PNG のプリセットを表示
        ui.add_enabled_ui(!*config.png_lossy(), |ui| {
            ui.horizontal(|ui| {
                ui::add_label(ui, "Preset:", ui::QUALITY_LABEL_WIDTH);
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
                ui::add_label(ui, "Dithering:", ui::QUALITY_LABEL_WIDTH);
                ui.scope(|ui| {
                    ui.spacing_mut().slider_width = setting::remaining_slider_width(ui);
                    ui.add(egui::Slider::new(config.png_dithering_mut(), config::PNG_DITHERING_MIN..=config::PNG_DITHERING_MAX));
                });
            });
        });
    });
}
