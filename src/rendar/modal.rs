use crate::app::{UpdateCheck, UpdatedToken};
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
    let modal = egui::Modal::new(egui::Id::new("modal_error")).show(ctx, |ui| {
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

/// 更新モーダルを表示
/// * `ctx` - コンテキスト
/// * `updated_token` - 更新モーダルを表示するためのトークン
pub(crate) fn updated(ctx: &egui::Context, updated_token: &mut UpdatedToken) {
    let Some(check) = updated_token.check.clone() else {
        return;
    };

    // モーダルを表示
    let modal = egui::Modal::new(egui::Id::new("modal_updated")).show(ctx, |ui| {
        ui.set_width(280.0);

        // 見出し部分
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            match &check {
                // アップデートがある場合
                UpdateCheck::Available { .. } => {
                    ui.add(egui::Image::new(svg::UPDATE).max_height(constants::MODAL_UPDATE_ICON_SIZE).tint(egui::Color32::GREEN));
                    ui.heading("Update available");
                }
                // アップデートが取得できなかった場合
                UpdateCheck::Failed => {
                    ui.add(egui::Image::new(svg::ERROR).max_height(constants::MODAL_ERROR_ICON_SIZE).tint(egui::Color32::RED));
                    ui.heading("Couldn't check for updates.");
                }
                // アップデートが最新の場合
                UpdateCheck::Latest => {
                    ui.add(egui::Image::new(svg::UPDATE).max_height(constants::MODAL_UPDATE_ICON_SIZE).tint(ui.visuals().text_color()));
                    ui.heading("Update not available");
                }
            }
        });

        ui.separator();
        ui.add_space(8.0);

        // 内容部分
        match &check {
            // アップデートがある場合
            UpdateCheck::Available { version, url } => {
                ui.label(format!("New version: {}", version));
                ui.add_space(8.0);
                ui.separator();
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Download").clicked() {
                            ui.ctx().open_url(egui::OpenUrl::new_tab(url));
                        }
                    });
                });
            }
            // アップデートが最新の場合
            UpdateCheck::Latest => {
                ui.label(format!("Current version: {}", env!("CARGO_PKG_VERSION")));
                ui.add_space(8.0);
            }
            // アップデートが取得できなかった場合
            UpdateCheck::Failed => {
                ui.add_space(8.0);
            }
        }
    });

    // モーダルを閉じたらモーダルを非表示にする
    if modal.should_close() {
        updated_token.open = false;
        updated_token.check = None;
    }
}
