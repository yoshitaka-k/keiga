use crate::{state, ui, lang, event, update, APP_NAME};
use crate::event::input;
use crate::ui::modal;
use crate::ui::setting::{top, general, optimize, output, concurrent, about};

/// 設定ウィンドウを表示
/// * `ctx` - コンテキスト
/// * `settings_window_open` - 設定ウィンドウを開いているかどうか
/// * `window_pos` - ウィンドウの表示位置
pub(crate) fn view(
    ctx: &egui::Context,
    state: &mut state::State,
    update_job: &mut update::UpdateJob,
    pending_actions: &mut Vec<event::EventAction>,
) {
    // ウィンドウのIDを生成
    let window_id = egui::ViewportId::from_hash_of(ui::SETTING_WINDOW_ID);

    // 設定ウィンドウのオプションを設定
    let mut options = egui::ViewportBuilder::default()
        .with_title(format!("{} - {}", APP_NAME, lang::current().setting.heading))
        .with_inner_size([ui::WINDOW_WIDTH, ui::WINDOW_HEIGHT])
        .with_maximize_button(false)
        .with_resizable(false);

    // ウィンドウの表示位置を指定
    // take()で、取り出して None にする（ボタン押下時だけ位置更新と前面化）
    if let Some(pos) = state.setting_token.pos.take() {
        // ウィンドウの位置を指定
        options = options.with_position(pos);

        // 表示していたら、ウィンドウの位置を更新して前面に出す
        ctx.send_viewport_cmd_to(window_id, egui::ViewportCommand::OuterPosition(pos));
        ctx.send_viewport_cmd_to(window_id, egui::ViewportCommand::Focus);
    }

    // 設定ウィンドウを表示
    ctx.show_viewport_immediate(window_id, options, |ui, _class| {
        // 上部パネルを表示
        top::view(ui, &state.config, &mut state.setting_token);

        egui::CentralPanel::default().show(ui, |ui| {
            // タブに応じて表示内容を切り替え
            match state.setting_token.tab {
                state::SettingTab::General => general::view(ui, &mut state.config),
                state::SettingTab::Optimize => optimize::view(ui, &mut state.config),
                state::SettingTab::Output => output::view(ui, &mut state.config),
                state::SettingTab::Concurrent => concurrent::view(ui, &mut state.config),
                state::SettingTab::About => about::view(ui, update_job),
            }
        });

        // Command + W キーが押されたら閉じる
        input::command_w(ui, pending_actions);

        // ウィンドウの閉じるボタンが押されたら閉じる
        if ui.input(|input| input.viewport().close_requested()) {
            pending_actions.push(event::EventAction::SettingClose);
        }

        // 更新モーダルを表示
        if state.updated_token.open {
            modal::updated(ui, &mut state.updated_token);
        }
    });
}
