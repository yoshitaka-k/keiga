use std::path::Path;
use crate::{file, error};
use crate::optimize::OptimizeJob;

/// 上キーが押されたら選択を上に移動する
/// * `files` - ファイル一覧
pub(crate) fn up(files: &mut file::OpenFiles) {
    files.index_sub();
}

/// 下キーが押されたら選択を下に移動する
/// * `files` - ファイル一覧
pub(crate) fn down(files: &mut file::OpenFiles) {
    files.index_add();
}

/// バックスペースキーが押されたらファイルをキャンセルする
/// * `files` - ファイル一覧
/// * `optimize_job` - 最適化ジョブ
pub fn backspace(
    files: &mut file::OpenFiles,
    optimize_job: &mut OptimizeJob
) -> error::Result<()> {
    let Some(image) = files.selected_image() else {
        return Ok(());
    };

    // 選択中のファイルをキャンセル
    // 最適化中のファイルはジョブに中断を伝える
    // 待機中・最適化中以外のファイルはキャンセルできない
    if image.is_optimizing() {
        optimize_job.add_canceled_id(*image.id())?;
    } else if !image.is_standby() {
        return Ok(());
    }

    // 選択中のファイルをキャンセル
    files.set_status_canceled(*image.id());

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
