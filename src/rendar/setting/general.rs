use crate::app;
use crate::rendar::assets;
use crate::rendar::assets::{constants as assets_const, svg};

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
        ui.add(egui::Image::new(svg::SETTINGS).max_height(assets_const::SETTINGS_ICON_SIZE).tint(icon_color));
        ui.spacing_mut().item_spacing.x = spacing;
        ui.label("General");
    });

    ui.separator();

    // 同じパスはスキップ
    ui.horizontal(|ui| {
        ui.label("Skip same path:");

        ui.radio_value(app.skip_same_path_mut(), true, "Skip same path");
        ui.radio_value(app.skip_same_path_mut(), false, "Don't skip same path");
    });

    // 同じパスはスキップの注意書きを表示
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 3.0;
        ui.add(egui::Image::new(svg::WARNING).max_height(assets_const::WARNING_ICON_SIZE).tint(assets::warning_color(ui)));
        ui.spacing_mut().item_spacing.x = spacing;
        ui.add(egui::Label::new(
            egui::RichText::new("Skip if already completed. Retry canceled and errors.").weak(),
        ));
    });
}
