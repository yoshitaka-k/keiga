use crate::app::{UpdateCheck, UpdatedToken};
use crate::rendar::{self, ErrorToken};
use crate::rendar::assets::{constants, svg};

// モーダルのラベルの幅
pub(crate) const MODAL_LABEL_WIDTH: f32 = 130.0;

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
        ui.set_width(rendar::MODAL_WINDOW_WIDTH);

        // 見出し部分
        ui.horizontal(|ui| {
            ui.add(heading_icon(svg::ERROR, constants::MODAL_ERROR_ICON_SIZE, egui::Color32::RED));
            ui.heading("An error occurred");
        });

        ui.separator();

        ui.add_space(rendar::MODAL_WINDOW_SPACING);

        // エラー内容を表示
        if let Some(error) = &error_token.value {
            egui::Frame::default().inner_margin(rendar::PANEL_INNER_MARGIN).show(ui, |ui| {
                ui.label(error.to_string());
            });
        }

        ui.add_space(rendar::MODAL_WINDOW_SPACING);
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
        ui.set_width(rendar::MODAL_WINDOW_WIDTH);

        // 見出し部分
        ui.horizontal(|ui| {
            match &check {
                // アップデートがある場合
                UpdateCheck::Available { .. } => {
                    ui.add(heading_icon(svg::UPDATE, constants::MODAL_UPDATE_ICON_SIZE, egui::Color32::GREEN));
                    ui.heading("Update available");
                }
                // アップデートが最新の場合
                UpdateCheck::Latest => {
                    ui.add(heading_icon(svg::UPDATE, constants::MODAL_UPDATE_ICON_SIZE, ui.visuals().text_color()));
                    ui.heading("Update not available");
                }
                // アップデートが取得できなかった場合
                UpdateCheck::Failed => {
                    ui.add(heading_icon(svg::ERROR, constants::MODAL_ERROR_ICON_SIZE, egui::Color32::RED));
                    ui.heading("Couldn't check for updates.");
                }
            }
        });

        ui.separator();

        ui.add_space(rendar::MODAL_WINDOW_SPACING);

        // 内容部分
        match &check {
            // アップデートがある場合
            UpdateCheck::Available { version, url } => {
                egui::Frame::default().inner_margin(rendar::PANEL_INNER_MARGIN).show(ui, |ui| {
                    rendar::add_label(ui, &format!("New version: {}", version), MODAL_LABEL_WIDTH);
                    ui.add(egui::Label::new(
                        egui::RichText::new(format!("Current version: v{}", env!("CARGO_PKG_VERSION"))).weak(),
                    ));
                    ui.add_space(rendar::MODAL_WINDOW_SPACING);
                });

                ui.separator();

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Download").clicked() {
                        ui.ctx().open_url(egui::OpenUrl::new_tab(url));
                    }
                });
            }
            // アップデートが最新の場合
            UpdateCheck::Latest => {
                egui::Frame::default().inner_margin(rendar::PANEL_INNER_MARGIN).show(ui, |ui| {
                    ui.label(format!("Current version: v{}", env!("CARGO_PKG_VERSION")));
                    ui.add_space(rendar::MODAL_WINDOW_SPACING);
                });
            }
            // アップデートが取得できなかった場合
            UpdateCheck::Failed => {
                egui::Frame::default().inner_margin(rendar::PANEL_INNER_MARGIN).show(ui, |ui| {
                    ui.add_space(rendar::MODAL_WINDOW_SPACING);
                });
            }
        }
    });

    // モーダルを閉じたらモーダルを非表示にする
    if modal.should_close() {
        updated_token.open = false;
        updated_token.check = None;
    }
}

/// 見出し横のアイコン
/// * `icon` - アイコン
/// * `size` - アイコンのサイズ
/// * `tint` - アイコンの色
/// * `return` - アイコンウィジェット
fn heading_icon(icon: egui::ImageSource<'static>, size: f32, tint: egui::Color32) -> egui::Image<'static> {
    // horizontal の行高 (interact_size.y = 18) に丸められるため、実サイズ (size) を指定する
    egui::Image::new(icon).fit_to_exact_size(egui::vec2(size, size)).tint(tint)
}
