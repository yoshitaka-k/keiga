use std::path::PathBuf;
use crate::optimize::OptimizeJob;
use crate::file;

/// バックスペースキーが押されたらファイルをキャンセルする
/// * `files` - ファイル一覧
/// * `optimize_job` - 最適化ジョブ
pub fn backspace_key(files: &mut file::OpenFiles, optimize_job: &mut OptimizeJob) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(id) = files.selected_id() {
        optimize_job.add_canceled_id(*id)?;
        files.set_status_canceled(*id);
    }
    Ok(())
}

/// スペースキーが押されたらファイルを選択表示する
/// * `path` - ファイルのパス
pub fn space_key(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if let Err(err) = quicklook_command(path) {
        return Err(format!("Error revealing file by QuickLook: {}", err).into());
    }
    Ok(())
}

/// QuickLook でファイルを表示する
/// * `path` - ファイルのパス
/// * `return` - エラーが発生したかどうか
#[cfg(target_os = "macos")]
fn quicklook_command(path: &PathBuf) -> Result<(), std::io::Error> {
    std::process::Command::new("qlmanage")
        .args(["-p", path.to_str().unwrap()]).spawn()?;
    Ok(())
}

/// Windows では QuickLook を使用しない
/// * `path` - ファイルのパス
/// * `return` - エラーが発生したかどうか
#[cfg(target_os = "windows")]
fn quicklook_command(_path: &PathBuf) -> Result<(), std::io::Error> {
    Ok(())
}

/// Linux では QuickLook を使用しない
/// * `path` - ファイルのパス
/// * `return` - エラーが発生したかどうか
#[cfg(target_os = "linux")]
fn quicklook_command(_path: &PathBuf) -> Result<(), std::io::Error> {
    Ok(())
}
