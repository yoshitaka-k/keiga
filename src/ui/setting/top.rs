use crate::{config, state, ui};
use crate::ui::assets;
use crate::ui::assets::{size, svg};

/// 設定ウィンドウの上部パネルを描画
/// * `ui` - UI
pub(crate) fn view(
    ui: &mut egui::Ui,
    config: &config::Config,
    setting_token: &mut state::SettingToken,
) {
    // パネルのスタイルを設定
    let style = ui::panel_style(config, ui::TOP_PANEL_INNER_MARGIN);
    // アイコンの色を取得
    let icon_color = ui::icon_color(ui);

    // タブを表示
    egui::Panel::top("setting_top_taskbar").frame(style).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui::add_icon(ui, svg::SETTINGS, size::TOP_MENU_SETTINGS_ICON_SIZE, icon_color);

            // タブの選択時の背景色を保存
            let selection_bg_fill = ui.style_mut().visuals.selection.bg_fill;

            // タブの選択時の背景色を設定
            ui.style_mut().visuals.selection.bg_fill = assets::tab_selected_color(config);

            // タブを表示
            ui.selectable_value(&mut setting_token.tab, state::SettingTab::General, state::SettingTab::General.to_string());
            ui.selectable_value(&mut setting_token.tab, state::SettingTab::Optimize, state::SettingTab::Optimize.to_string());
            ui.selectable_value(&mut setting_token.tab, state::SettingTab::Output, state::SettingTab::Output.to_string());
            ui.selectable_value(&mut setting_token.tab, state::SettingTab::Concurrent, state::SettingTab::Concurrent.to_string());
            ui.selectable_value(&mut setting_token.tab, state::SettingTab::About, state::SettingTab::About.to_string());

            // タブの選択時の背景色をリセット
            ui.style_mut().visuals.selection.bg_fill = selection_bg_fill;
        });
    });
}
