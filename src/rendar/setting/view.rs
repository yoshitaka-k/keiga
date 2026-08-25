use crate::app::{self, UpdateJob, UpdatedToken};
use crate::rendar::{self, SettingTab, SettingToken};
use crate::rendar::assets::{self, constants, svg};
use crate::rendar::modal;
use crate::rendar::setting::{self, general, concurrent, quality, about};

/// 設定ウィンドウを表示
/// * `ctx` - コンテキスト
/// * `settings_window_open` - 設定ウィンドウを開いているかどうか
/// * `window_pos` - ウィンドウの表示位置
pub(crate) fn view(
    ctx: &egui::Context,
    app: &mut app::App,
    setting_token: &mut SettingToken,
    mut update_job: &mut UpdateJob,
    updated_token: &mut UpdatedToken,
) {
    // ウィンドウのIDを生成
    let window_id = egui::ViewportId::from_hash_of(setting::SETTING_WINDOW_ID);

    // 設定ウィンドウのオプションを設定
    let mut options = egui::ViewportBuilder::default()
        .with_title(setting::WINDOW_TITLE)
        .with_inner_size([setting::WINDOW_WIDTH, setting::WINDOW_HEIGHT])
        .with_maximize_button(false)
        .with_resizable(false);

    // ウィンドウの表示位置を指定
    // take()で、取り出して None にする（ボタン押下時だけ位置更新と前面化）
    if let Some(pos) = setting_token.pos.take() {
        // ウィンドウの位置を指定
        options = options.with_position(pos);

        // タブを初期化
        setting_token.tab = SettingTab::General;

        // 表示していたら、ウィンドウの位置を更新して前面に出す
        ctx.send_viewport_cmd_to(window_id, egui::ViewportCommand::OuterPosition(pos));
        ctx.send_viewport_cmd_to(window_id, egui::ViewportCommand::Focus);
    }

    // 設定ウィンドウを表示
    ctx.show_viewport_immediate(window_id, options, |ctx, _class| {
        // 更新結果を取得
        update_job.result(updated_token);

        // パネルのスタイルを設定
        let panel_style = rendar::panel_style(ctx, rendar::TOP_PANEL_INNER_MARGIN);

        // アイコンの色を取得
        let icon_color = assets::icon_color(ctx);

        // タブを表示
        egui::Panel::top("setting_top_taskbar").frame(panel_style).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add(egui::Image::new(svg::SETTINGS).max_height(constants::TOP_MENU_SETTINGS_ICON_SIZE).tint(icon_color));

                // タブの選択時の背景色を保存
                let selection_bg_fill = ui.style_mut().visuals.selection.bg_fill;

                // タブの選択時の背景色を設定
                ui.style_mut().visuals.selection.bg_fill = setting::tab_selected_color(ui);

                // タブを表示
                ui.selectable_value(&mut setting_token.tab, SettingTab::General, SettingTab::General.to_string());
                ui.selectable_value(&mut setting_token.tab, SettingTab::Concurrent, SettingTab::Concurrent.to_string());
                ui.selectable_value(&mut setting_token.tab, SettingTab::Quality, SettingTab::Quality.to_string());
                ui.selectable_value(&mut setting_token.tab, SettingTab::About, SettingTab::About.to_string());

                // タブの選択時の背景色をリセット
                ui.style_mut().visuals.selection.bg_fill = selection_bg_fill;
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // タブに応じて表示内容を切り替え
            match setting_token.tab {
                SettingTab::General => general::view(ui, app),
                SettingTab::Concurrent => concurrent::view(ui, app),
                SettingTab::Quality => quality::view(ui, app),
                SettingTab::About => about::view(ui, app, &mut update_job),
            }
        });

        // ウィンドウの閉じるボタンが押されたら閉じる
        if ctx.input(|input| input.viewport().close_requested()) {
            setting_token.open = false;
        }

        // 更新モーダルを表示
        if updated_token.open {
            modal::updated(ctx, updated_token);
        }
    });
}
