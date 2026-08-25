use crate::app::{self, UpdateJob};
use crate::rendar;
use crate::rendar::assets::{self, constants, svg};
use crate::rendar::setting;

/// バージョンを表示
/// * `ui` - UI
/// * `update_job` - アップデートジョブ
pub(crate) fn view(ui: &mut egui::Ui, update_job: &mut UpdateJob) {
    // ヘッダーパネルを表示
    setting::header_panel(ui, svg::INFO, "About", Some(update_job));

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
