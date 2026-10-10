use std::path::Path;
use crate::{file, error};
use crate::optimize::OptimizeJob;

/// バックスペースキーが押されたらファイルをキャンセルする
/// * `files` - ファイル一覧
/// * `optimize_job` - 最適化ジョブ
pub fn backspace(files: &mut file::OpenFiles, optimize_job: &mut OptimizeJob) -> error::Result<()> {
    // 選択中のファイルをキャンセル
    if let Some(id) = files.selected_id() {
        optimize_job.add_canceled_id(*id)?;
        files.set_status_canceled(*id);
    }
    Ok(())
}

/// スペースキーが押されたらファイルを選択表示する
/// * `path` - ファイルのパス
pub fn space(path: &Path) -> error::Result<()> {
    if !path.exists() {
        return Err(error::KeigaError::file_not_found("File does not exist", path.to_path_buf()));
    }

    // ファイルをプレビュー表示
    preview_file_command(path)
}

/// QuickLook でファイルを表示する
#[cfg(target_os = "macos")]
fn preview_file_command(path: &Path) -> error::Result<()> {
    std::process::Command::new("qlmanage").arg("-p").arg(path).spawn()?;
    Ok(())
}

/// Windows では QuickLook を使用しない
/// * `path` - ファイルのパス
/// * `return` - エラーが発生したかどうか
#[cfg(target_os = "windows")]
fn preview_file_command(_path: &Path) -> error::Result<()> {
    Ok(())
}

/// Linux では QuickLook を使用しない
/// * `path` - ファイルのパス
/// * `return` - エラーが発生したかどうか
#[cfg(target_os = "linux")]
fn preview_file_command(_path: &Path) -> error::Result<()> {
    Ok(())
}
