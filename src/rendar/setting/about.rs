use crate::app::{self, UpdateJob};
use crate::event::button;
use crate::rendar;
use crate::rendar::assets::{self, constants, svg};
use crate::rendar::setting;

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
        ui.spacing_mut().item_spacing.x = setting::HEADER_ICON_SPACING;
        ui.add(egui::Image::new(svg::INFO).max_height(constants::INFO_ICON_SIZE).tint(icon_color));
        ui.spacing_mut().item_spacing.x = spacing;
        ui.label("About");

        // アップデート確認ボタン
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("Check for updates.").clicked() {
                button::check_for_update(update_job);
            }
        });
    });

    ui.add_space(setting::HEADER_BOTTOM_SPACING);

    ui.separator();

    egui::Frame::default().inner_margin(rendar::PANEL_INNER_MARGIN).show(ui, |ui| {
        // アプリのアイコンを表示
        ui.add(egui::Image::new(assets::APP_ICON).max_height(constants::APP_ICON_SIZE));

        // バージョンを表示
        ui.label(format!("Keiga v{}", env!("CARGO_PKG_VERSION")));

        // 説明を表示
        ui.label(env!("CARGO_PKG_DESCRIPTION"));

        ui.label("");

        // ライセンスを表示
        ui.label(format!("License: {}", env!("CARGO_PKG_LICENSE")));

        // リポジトリを表示
        ui.horizontal(|ui| {
            ui.label("Repository:");
            ui.hyperlink_to(env!("CARGO_PKG_REPOSITORY"), app::GITHUB_URL.replace("{repository}", env!("CARGO_PKG_REPOSITORY")));
        });
    });
}
