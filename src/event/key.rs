use std::path::Path;
use crate::{file, error};
use crate::optimize::OptimizeJob;

/// 上キーが押されたら選択を上に移動する
/// * `files` - ファイル一覧
/// * `id` - 選択されているファイルの ID
pub(crate) fn up(files: &mut file::OpenFiles, id: u64) {
    let min_id = files.get_min_id();

    // 選択されているファイルの ID が最小の ID より大きい場合
    if id > min_id {
        files.set_selected_id(Some(id - 1));
    } else {
        files.set_selected_id(Some(min_id));
    }
}

/// 下キーが押されたら選択を下に移動する
/// * `files` - ファイル一覧
/// * `id` - 選択されているファイルの ID
pub(crate) fn down(files: &mut file::OpenFiles, id: u64) {
    let max_id = files.get_max_id();

    // 選択されているファイルの ID が最大の ID より小さい場合
    if id < max_id {
        files.set_selected_id(Some(id + 1));
    } else {
        files.set_selected_id(Some(max_id));
    }
}

/// バックスペースキーが押されたらファイルをキャンセルする
/// * `files` - ファイル一覧
/// * `optimize_job` - 最適化ジョブ
pub fn backspace(
    files: &mut file::OpenFiles,
    optimize_job: &mut OptimizeJob
) -> error::Result<()> {
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
        return Err(error::KeigaError::file_not_found(path.to_path_buf()));
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
