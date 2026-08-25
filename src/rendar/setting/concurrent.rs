use crate::app;
use crate::rendar::assets;
use crate::rendar::assets::{constants as assets_const, svg};
use crate::rendar::setting;

/// 並行処理数を表示
/// * `ui` - UI
/// * `app` - アプリ
pub(crate) fn view(ui: &mut egui::Ui, app: &mut app::App) {
    // デフォルトのスペースの幅を避けておく
    let spacing = ui.spacing().item_spacing.x;

    // ボタンの色を設定
    let icon_color = assets::icon_color(ui);

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;
        ui.add(egui::Image::new(svg::CYCLE).max_height(assets_const::CYCLE_ICON_SIZE).tint(icon_color));
        ui.spacing_mut().item_spacing.x = spacing;
        ui.label("Concurrent");
    });

    ui.add_space(2.0);

    ui.separator();

    ui.add_space(setting::SETTING_ADD_SPACING);

    egui::Frame::default().inner_margin(setting::PANEL_INNER_MARGIN).show(ui, |ui| {
        // 全ファイルの最適化数
        ui.horizontal(|ui| {
            setting::add_label(ui, "Concurrent All files:", setting::CONCURRENT_LABEL_WIDTH);
            ui.scope(|ui| {
                ui.spacing_mut().slider_width = setting::remaining_slider_width(ui);
                ui.add(egui::Slider::new(app.optimization_num_mut(), setting::OPTIMIZATION_NUM_MIN..=setting::OPTIMIZATION_NUM_MAX));
            });
        });

        ui.add_space(setting::SETTING_ADD_SPACING);

        // PNG の最適化数
        ui.horizontal(|ui| {
            setting::add_label(ui, "Concurrent PNG files:", setting::CONCURRENT_LABEL_WIDTH);
            ui.scope(|ui| {
                ui.spacing_mut().slider_width = setting::remaining_slider_width(ui);
                ui.add(egui::Slider::new(app.png_optimization_num_mut(), setting::PNG_OPTIMIZATION_NUM_MIN..=setting::PNG_OPTIMIZATION_NUM_MAX));
            });
        });

        // PNG の最適化数の注意書きを表示
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 3.0;
            ui.add(egui::Image::new(svg::WARNING).max_height(assets_const::WARNING_ICON_SIZE).tint(assets::warning_color(ui)));
            ui.spacing_mut().item_spacing.x = spacing;
            ui.add(egui::Label::new(
                egui::RichText::new(format!("PNG is included in All. ({} / {})", app.png_optimization_num(), app.optimization_num())).weak(),
            ));
        });
    });
}
