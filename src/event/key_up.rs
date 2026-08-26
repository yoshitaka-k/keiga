use std::path::{Path, PathBuf};
use crate::file;
use crate::optimize::OptimizeJob;

/// バックスペースキーが押されたらファイルをキャンセルする
/// * `files` - ファイル一覧
/// * `optimize_job` - 最適化ジョブ
pub fn backspace(files: &mut file::OpenFiles, optimize_job: &mut OptimizeJob) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(id) = files.selected_id() {
        optimize_job.add_canceled_id(*id)?;
        files.set_status_canceled(*id);
    }
    Ok(())
}

/// スペースキーが押されたらファイルを選択表示する
/// * `path` - ファイルのパス
pub fn space(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if let Err(err) = quicklook_command(path) {
        return Err(format!("Error revealing file by QuickLook: {}", err).into());
    }
    Ok(())
}

/// QuickLook でファイルを表示する
/// * `path` - ファイルのパス
/// * `return` - エラーが発生したかどうか
fn quicklook_command(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if !path.exists() {
        return Err("File does not exist".into());
    }

    // ファイルをプレビュー表示
    return preview_file_command(&path.as_path());
}

/// QuickLook でファイルを表示する
#[cfg(target_os = "macos")]
fn preview_file_command(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    std::process::Command::new("qlmanage").arg("-p").arg(path).spawn()?;
    Ok(())
}

/// Windows では QuickLook を使用しない
/// * `path` - ファイルのパス
/// * `return` - エラーが発生したかどうか
#[cfg(target_os = "windows")]
fn preview_file_command(_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

/// Linux では QuickLook を使用しない
/// * `path` - ファイルのパス
/// * `return` - エラーが発生したかどうか
#[cfg(target_os = "linux")]
fn preview_file_command(_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
