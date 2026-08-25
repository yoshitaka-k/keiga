use crate::app;
use crate::app::UpdateJob;
use crate::event::button;
use crate::rendar::assets;
use crate::rendar::assets::{constants as assets_const, svg};

/// バージョンを表示
/// * `ui` - UI
/// * `app` - アプリ
pub(crate) fn view(
    ui: &mut egui::Ui,
    _app: &mut app::App,
    update_job: &mut UpdateJob,
) {
    // デフォルトのスペースの幅を避けておく
    let spacing = ui.spacing().item_spacing.x;

    // ボタンの色を設定
    let icon_color = assets::icon_color(ui);

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;
        ui.add(egui::Image::new(svg::INFO).max_height(assets_const::INFO_ICON_SIZE).tint(icon_color));
        ui.spacing_mut().item_spacing.x = spacing;
        ui.label("About");

        // アップデート確認ボタン
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("Check for updates.").clicked() {
                button::check_for_update(update_job);
            }
        });
    });

    ui.add_space(2.0);

    ui.separator();

    ui.add(egui::Image::new(assets::APP_ICON).max_height(assets_const::APP_ICON_SIZE));

    ui.label(format!("Keiga v{}", env!("CARGO_PKG_VERSION")));
    ui.label(env!("CARGO_PKG_DESCRIPTION"));

    ui.label("");

    ui.label(format!("License: {}", env!("CARGO_PKG_LICENSE")));

    ui.horizontal(|ui| {
        ui.label("Repository:");
        ui.hyperlink_to(env!("CARGO_PKG_REPOSITORY"), app::GITHUB_URL.replace("{repository}", env!("CARGO_PKG_REPOSITORY")));
    });
}
