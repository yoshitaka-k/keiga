use crate::{ui, lang};
use crate::ui::assets::{size, svg};
use crate::ui::modal::heading_icon;
use crate::update::{UpdateCheck, UpdateToken};

/// 更新モーダルを表示
/// * `ctx` - コンテキスト
/// * `update_token` - 更新モーダルを表示するためのトークン
pub(crate) fn updated(ctx: &egui::Context, update_token: &mut UpdateToken) {
    let Some(check) = update_token.check.clone() else {
        return;
    };

    // モーダルを表示
    let modal = egui::Modal::new(egui::Id::new("modal_updated")).show(ctx, |ui| {
        ui.set_width(ui::MODAL_WINDOW_WIDTH);

        // 見出し部分
        ui.horizontal(|ui| {
            match &check {
                // アップデートがある場合
                UpdateCheck::Available { .. } => {
                    ui.add(heading_icon(svg::UPDATE, size::MODAL_UPDATE_ICON_SIZE, egui::Color32::GREEN));
                    ui.heading(lang::current().modal_update.available);
                }
                // アップデートが最新の場合
                UpdateCheck::Latest => {
                    ui.add(heading_icon(svg::UPDATE, size::MODAL_UPDATE_ICON_SIZE, ui.visuals().text_color()));
                    ui.heading(lang::current().modal_update.latest);
                }
                // アップデートが取得できなかった場合
                UpdateCheck::Failed => {
                    ui.add(heading_icon(svg::ERROR, size::MODAL_ERROR_ICON_SIZE, egui::Color32::RED));
                    ui.heading(lang::current().modal_update.failed);
                }
            }
        });

        ui.separator();

        ui.add_space(ui::MODAL_WINDOW_SPACING);

        // 内容部分
        match &check {
            // アップデートがある場合
            UpdateCheck::Available { version, url } => {
                egui::Frame::default().inner_margin(ui::PANEL_INNER_MARGIN).show(ui, |ui| {
                    ui::add_label(ui, &format!("{}: {}", lang::current().modal_update.new_version, version), ui::MODAL_LABEL_WIDTH);
                    ui.add(egui::Label::new(
                        egui::RichText::new(format!("{}: v{}", lang::current().modal_update.current_version, env!("CARGO_PKG_VERSION"))).weak(),
                    ));
                    ui.add_space(ui::MODAL_WINDOW_SPACING);
                });

                ui.separator();

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(lang::current().modal_update.download).clicked() {
                        ui.ctx().open_url(egui::OpenUrl::new_tab(url));
                    }
                });
            }
            // アップデートが最新の場合
            UpdateCheck::Latest => {
                egui::Frame::default().inner_margin(ui::PANEL_INNER_MARGIN).show(ui, |ui| {
                    ui.label(format!("{}: v{}", lang::current().modal_update.current_version, env!("CARGO_PKG_VERSION")));
                    ui.add_space(ui::MODAL_WINDOW_SPACING);
                });
            }
            // アップデートが取得できなかった場合
            UpdateCheck::Failed => {
                egui::Frame::default().inner_margin(ui::PANEL_INNER_MARGIN).show(ui, |ui| {
                    ui.add_space(ui::MODAL_WINDOW_SPACING);
                });
            }
        }
    });

    // モーダルを閉じたらモーダルを非表示にする
    if modal.should_close() {
        update_token.open = false;
        update_token.check = None;
    }
}
