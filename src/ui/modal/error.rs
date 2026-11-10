use crate::ui;
use crate::ui::ErrorToken;
use crate::ui::assets::{constants, svg};
use crate::ui::modal::heading_icon;

/// エラーモーダルを表示
/// * `show_modal` - モーダルを表示するかどうか
/// * `ctx` - コンテキスト
/// * `error_token` - エラーモーダルを表示するためのトークン
pub(crate) fn error(ctx: &egui::Context, error_token: &mut ErrorToken) {
    if error_token.value.is_none() {
        return;
    }

    // モーダルを表示
    let modal = egui::Modal::new(egui::Id::new("modal_error")).show(ctx, |ui| {
        ui.set_width(ui::MODAL_WINDOW_WIDTH);

        // 見出し部分
        ui.horizontal(|ui| {
            ui.add(heading_icon(svg::ERROR, constants::MODAL_ERROR_ICON_SIZE, egui::Color32::RED));
            ui.heading("An error occurred");
        });

        ui.separator();

        ui.add_space(ui::MODAL_WINDOW_SPACING);

        // エラー内容を表示
        if let Some(error) = &error_token.value {
            egui::Frame::default().inner_margin(ui::PANEL_INNER_MARGIN).show(ui, |ui| {
                ui.label(error.to_string());
            });
        }

        ui.add_space(ui::MODAL_WINDOW_SPACING);
    });

    // モーダルを閉じたらモーダルを非表示にする
    if modal.should_close() {
        error_token.open = false;
        error_token.value = None;
    }
}
