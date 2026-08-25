use std::path::PathBuf;

/// ダブルクリックで Finder でファイルを選択表示する
/// * `path` - ファイルのパス
/// * `return` - エラーが発生したかどうか
pub fn double_click(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if !path.exists() {
        return Err("File does not exist".into());
    }

    if let Some(path) = path.to_str() {
        reveal_file_command(path)?;
        return Ok(());
    }

    return Err("Path is not valid".into());
}

/// Finder でファイルを選択表示する
/// * `path` - ファイルのパス
/// * `return` - エラーが発生したかどうか
#[cfg(target_os = "macos")]
fn reveal_file_command(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    std::process::Command::new("open").args(["-R", path]).status()?;
    Ok(())
}

/// Explorer でファイルを選択表示する
/// * `path` - ファイルのパス
/// * `return` - エラーが発生したかどうか
#[cfg(target_os = "windows")]
fn reveal_file_command(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    std::process::Command::new("explorer").args(["/select,", path]).status()?;
    Ok(())
}

/// XDG でファイルを選択表示する
/// * `path` - ファイルのパス
/// * `return` - エラーが発生したかどうか
#[cfg(target_os = "linux")]
fn reveal_file_command(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    std::process::Command::new("xdg-open").args(["--reveal", path]).status()?;
    Ok(())
}
