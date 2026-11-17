use crate::{error, event, state};
use crate::event::{open, button, key, click};

/// イベントを処理
/// * `ctx` - コンテキスト
/// * `state` - 状態
/// * `jobs` - ジョブ
/// * `action` - イベント
/// * `return` - エラーが発生したかどうか
pub(crate) fn handle(
    ctx: &egui::Context,
    state: &mut state::State,
    jobs: &mut state::Jobs,
    action: event::EventAction,
) -> error::Result<()> {
    match action {
        event::EventAction::FileOpen => {
            open::file(&state.config, &mut state.files)
        }
        event::EventAction::FolderOpen => {
            open::folder(&state.config, &mut state.files)
        }
        event::EventAction::DropOpen(files) => {
            open::drop(
                ctx,
                &files,
                &state.config,
                &mut state.files,
            )
        }
        event::EventAction::SettingOpen(pos) => {
            button::setting_open(&mut state.setting_token, pos);
            Ok(())
        }
        event::EventAction::Click { index } => {
            click::click(&mut state.files, index);
            Ok(())
        }
        event::EventAction::DoubleClick(path) => {
            click::double_click(&path)
        }
        event::EventAction::Up => {
            key::up(&mut state.files, &mut state.scroll_token);
            Ok(())
        }
        event::EventAction::Down => {
            key::down(&mut state.files, &mut state.scroll_token);
            Ok(())
        }
        event::EventAction::Enter { path } => {
            click::double_click(&path)
        }
        event::EventAction::Space { path } => {
            key::space(&path)
        }
        event::EventAction::Backspace => {
            key::backspace(&mut state.files, &mut jobs.optimize_job)
        }
    }
}
