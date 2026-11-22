use crate::{error, event, state, lang};
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
            let label = lang::current().file_dialog.image_files;
            open::file(&jobs.scan_job, label);
            Ok(())
        }
        event::EventAction::FolderOpen => {
            let label = lang::current().file_dialog.image_files;
            open::folder(&jobs.scan_job, label);
            Ok(())
        }
        event::EventAction::DropOpen(files) => {
            open::drop(ctx, &jobs.scan_job, files);
            Ok(())
        }
        event::EventAction::Clear => {
            button::cancel_and_clear(&mut state.files, &mut jobs.scan_job, &mut jobs.optimize_job)
        }
        event::EventAction::SettingOpen(pos) => {
            button::setting_open(&mut state.setting_token, pos);
            Ok(())
        }
        event::EventAction::SettingClose => {
            state.setting_token.open = false;
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
        event::EventAction::CheckForUpdate => {
            button::check_for_update(&mut jobs.update_job, &mut state.updated_token);
            Ok(())
        }
    }
}
