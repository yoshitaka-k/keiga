use crate::rendar::assets::{svg, constants};
use crate::rendar::ErrorToken;

/// エラーモーダルを表示
/// * `show_modal` - モーダルを表示するかどうか
/// * `ctx` - コンテキスト
/// * `error_token` - エラーモーダルを表示するためのトークン
pub(crate) fn error(ctx: &egui::Context, error_token: &mut ErrorToken) {
    if error_token.value.is_none() {
        return;
    }

    // モーダルを表示
    let modal = egui::Modal::new(egui::Id::new("error")).show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.add(egui::Image::new(svg::ERROR).max_height(constants::MODAL_ERROR_ICON_SIZE).tint(egui::Color32::RED));
            ui.heading("An error occurred");
        });

        if let Some(error) = &error_token.value {
            ui.label(error.to_string());
        }
    });

    // モーダルを閉じたらモーダルを非表示にする
    if modal.should_close() {
        error_token.open = false;
        error_token.value = None;
    }
}
