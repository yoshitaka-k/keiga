mod view;
mod general;
mod optimize;
mod output;
mod concurrent;
mod about;

pub(crate) use view::view;

// インポート
use crate::{config, ui, lang};
use crate::event::button;
use crate::ui::assets;
use crate::ui::assets::{size, svg};
use crate::update::UpdateJob;

/// タブの選択時の背景色を取得
/// * `ui` - UI
/// * `return` - タブの選択時の背景色
fn tab_selected_color(config: &config::Config) -> egui::Color32 {
    if *config.theme() == ui::Theme::Dark {
        ui::DARK_TAB_SELECTED_COLOR
    } else {
        ui::LIGHT_TAB_SELECTED_COLOR
    }
}

/// ラベル後の残り幅に合わせてスライダーのレール幅を決める
/// * `ui` - UI
/// * `return` - スライダーのレール幅
fn remaining_slider_width(ui: &egui::Ui) -> f32 {
    let spacing = ui.spacing();
    (ui.available_width() - spacing.item_spacing.x - spacing.interact_size.x).max(0.0)
}

/// ヘッダーパネルを表示
/// * `ui` - UI
/// * `icon` - アイコン
/// * `label` - ラベル
/// * `update_job` - アップデート確認ボタン
fn header_panel(
    ui: &mut egui::Ui,
    icon: egui::ImageSource<'static>,
    label: &str,
    update_job: Option<&mut UpdateJob>,
) {
    let spacing = ui.spacing().item_spacing.x;

    // ボタンの色を設定
    let icon_color = assets::icon_color(ui);

    ui.horizontal(|ui| {
        ui.add_space(ui::SETTING_INDENT_SPACING);
        ui.spacing_mut().item_spacing.x = ui::HEADER_ICON_SPACING;
        ui.add(egui::Image::new(icon).max_height(size::SETTINGS_ICON_SIZE).tint(icon_color));
        ui.spacing_mut().item_spacing.x = spacing;
        ui.label(label);

        // アップデート確認ボタン
        if let Some(update_job) = update_job {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(lang::current().setting_about.check_for_updates).clicked() {
                    button::check_for_update(update_job);
                }
            });
        }
    });
}

/// 警告ノートを表示
/// * `ui` - UI
/// * `config` - 設定
/// * `text` - 警告ノートのテキスト
/// * `return` - 警告ノートのリッチテキスト
fn warning_note(
    ui: &mut egui::Ui,
    config: &config::Config,
    text: &str
) {
    // デフォルトのスペースの幅を避けておく
    let spacing = ui.spacing().item_spacing.x;

    // 出力パスの注意書きを表示
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = ui::WARNING_ICON_SPACING;
        ui.add(egui::Image::new(svg::WARNING).max_height(size::WARNING_ICON_SIZE).tint(assets::warning_color(config)));
        ui.spacing_mut().item_spacing.x = spacing;
        ui.add(egui::Label::new(
            egui::RichText::new(text).weak(),
        ));
    });
}
