use crate::app;
use crate::optimize::options::PngPreset;
use crate::rendar;
use crate::rendar::assets::{self, constants, svg};
use crate::rendar::setting;

/// 品質を表示
/// * `ui` - UI
/// * `app` - アプリ
pub(crate) fn view(ui: &mut egui::Ui, app: &mut app::App) {
    // デフォルトのスペースの幅を避けておく
    let spacing = ui.spacing().item_spacing.x;

    // ボタンの色を設定
    let icon_color = assets::icon_color(ui);

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = setting::HEADER_ICON_SPACING;
        ui.add(egui::Image::new(svg::COMPRESS).max_height(constants::COMPRESS_ICON_SIZE).tint(icon_color));
        ui.spacing_mut().item_spacing.x = spacing;
        ui.label("Quality");
    });

    ui.add_space(setting::HEADER_BOTTOM_SPACING);

    ui.separator();

    ui.add_space(setting::SETTING_ADD_SPACING);

    egui::Frame::default().inner_margin(rendar::PANEL_INNER_MARGIN).show(ui, |ui| {
        // JPEG のスライダーを表示
        ui.horizontal(|ui| {
            rendar::add_label(ui, "JPEG Quality:", setting::QUALITY_LABEL_WIDTH);
            ui.scope(|ui| {
                ui.spacing_mut().slider_width = setting::remaining_slider_width(ui);
                ui.add(egui::Slider::new(app.jpeg_quality_mut(), setting::JPEG_QUALITY_MIN..=setting::JPEG_QUALITY_MAX));
            });
        });

        // JPEG の品質の注意書きを表示
        setting::warning_note(ui, &format!("JPEG is lossy compression."));
    });

    ui.add_space(setting::SETTING_ADD_SPACING);

    ui.separator();

    ui.add_space(setting::SETTING_ADD_SPACING);

    egui::Frame::default().inner_margin(rendar::PANEL_INNER_MARGIN).show(ui, |ui| {
        // PNG のプリセットを表示
        ui.horizontal(|ui| {
            rendar::add_label(ui, "PNG Preset:", setting::QUALITY_LABEL_WIDTH);
            ui.radio_value(app.png_preset_mut(), PngPreset::Min, PngPreset::Min.to_string());
            ui.radio_value(app.png_preset_mut(), PngPreset::Fast, PngPreset::Fast.to_string());
            ui.radio_value(app.png_preset_mut(), PngPreset::Default, PngPreset::Default.to_string());
            ui.radio_value(app.png_preset_mut(), PngPreset::Best, PngPreset::Best.to_string());
            ui.radio_value(app.png_preset_mut(), PngPreset::Max, PngPreset::Max.to_string());
        });

        // JPEG の品質の注意書きを表示
        setting::warning_note(ui, &format!("PNG is lossless compression."));
    });
}
