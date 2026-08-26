use std::path::{Path, PathBuf};

/// ダブルクリックで Finder でファイルを選択表示する
/// * `path` - ファイルのパス
/// * `return` - エラーが発生したかどうか
pub fn double_click(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if !path.exists() {
        return Err("File does not exist".into());
    }

    // ファイルを選択表示
    return reveal_file_command(&path.as_path());
}

/// Finder でファイルを選択表示する
/// * `path` - ファイルのパス
/// * `return` - エラーが発生したかどうか
#[cfg(target_os = "macos")]
fn reveal_file_command(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    std::process::Command::new("open").arg("-R").arg(path).status()?;
    Ok(())
}

/// Explorer でファイルを選択表示する
/// * `path` - ファイルのパス
/// * `return` - エラーが発生したかどうか
#[cfg(target_os = "windows")]
fn reveal_file_command(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    std::process::Command::new("explorer").arg("/select,{path}").status()?;
    Ok(())
}

/// XDG でファイルを選択表示する
/// * `path` - ファイルのパス
/// * `return` - エラーが発生したかどうか
#[cfg(target_os = "linux")]
fn reveal_file_command(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    std::process::Command::new("xdg-open").arg("--reveal").arg(path).status()?;
    Ok(())
}
