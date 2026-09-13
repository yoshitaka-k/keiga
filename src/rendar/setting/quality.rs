use crate::app;
use crate::optimize::options::PngPreset;
use crate::rendar;
use crate::rendar::assets::svg;
use crate::rendar::setting;

/// 品質を表示
/// * `ui` - UI
/// * `app` - アプリ
pub(crate) fn view(ui: &mut egui::Ui, app: &mut app::App) {
    // ヘッダーパネルを表示
    setting::header_panel(ui, svg::COMPRESS, "Quality", None);

    ui.add_space(setting::HEADER_BOTTOM_SPACING);

    ui.separator();

    ui.add_space(setting::SETTING_ADD_SPACING);

    setting::header_panel(ui, svg::IMAGE, "JPEG", None);

    egui::Frame::default().inner_margin(rendar::PANEL_INNER_MARGIN).show(ui, |ui| {
        // JPEG の圧縮方法を表示
        ui.horizontal(|ui| {
            rendar::add_label(ui, "Compression:", setting::QUALITY_LABEL_WIDTH);
            ui.scope(|ui| {
                ui.spacing_mut().slider_width = setting::remaining_slider_width(ui);
                ui.radio_value(app.jpeg_lossy_mut(), false, "Lossless");
                ui.radio_value(app.jpeg_lossy_mut(), true, "Lossy");
            });
        });

        // JPEG の圧縮方法の注意書きを表示
        if *app.jpeg_lossy() {
            setting::warning_note(ui, "Re-encodes at the quality below..");
        } else {
            setting::warning_note(ui, "losNear-lossless re-encode. Still lossy; may not shrink.");
        }

        // JPEG のスライダーを表示
        ui.add_enabled_ui(*app.jpeg_lossy(), |ui| {
            ui.horizontal(|ui| {
                rendar::add_label(ui, "Quality:", setting::QUALITY_LABEL_WIDTH);
                ui.scope(|ui| {
                    ui.spacing_mut().slider_width = setting::remaining_slider_width(ui);
                    ui.add(egui::Slider::new(app.jpeg_quality_mut(), setting::JPEG_QUALITY_MIN..=setting::JPEG_QUALITY_MAX));
                });
            });
        });
    });

    ui.add_space(setting::SETTING_ADD_SPACING);

    ui.separator();

    ui.add_space(setting::SETTING_ADD_SPACING);

    setting::header_panel(ui, svg::IMAGE, "PNG", None);

    egui::Frame::default().inner_margin(rendar::PANEL_INNER_MARGIN).show(ui, |ui| {
        // PNG のプリセットを表示
        ui.horizontal(|ui| {
            rendar::add_label(ui, "Preset:", setting::QUALITY_LABEL_WIDTH);
            ui.radio_value(app.png_preset_mut(), PngPreset::Min, PngPreset::Min.to_string());
            ui.radio_value(app.png_preset_mut(), PngPreset::Fast, PngPreset::Fast.to_string());
            ui.radio_value(app.png_preset_mut(), PngPreset::Default, PngPreset::Default.to_string());
            ui.radio_value(app.png_preset_mut(), PngPreset::Best, PngPreset::Best.to_string());
            ui.radio_value(app.png_preset_mut(), PngPreset::Max, PngPreset::Max.to_string());
        });

        // PNG の品質の注意書きを表示
        setting::warning_note(ui, "PNG is lossless compression.");
    });
}
