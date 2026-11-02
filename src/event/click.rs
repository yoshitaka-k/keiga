use std::path::Path;
use crate::error;

/// ダブルクリックで Finder でファイルを選択表示する
/// * `path` - ファイルのパス
/// * `return` - エラーが発生したかどうか
pub fn double_click(path: &Path) -> error::Result<()> {
    if !path.exists() {
        return Err(error::KeigaError::file_not_found("File does not exist", path.to_path_buf()));
    }

    // ファイルを選択表示
    reveal_file_command(path)
}

/// Finder でファイルを選択表示する
/// * `path` - ファイルのパス
#[cfg(target_os = "macos")]
fn reveal_file_command(path: &Path) -> error::Result<()> {
    std::process::Command::new("open")
        .arg("-R")
        .arg(path)
        .status()?;
    Ok(())
}

/// Explorer でファイルを選択表示する
/// * `path` - ファイルのパス
#[cfg(target_os = "windows")]
fn reveal_file_command(path: &Path) -> error::Result<()> {
    use std::os::windows::process::CommandExt;
    std::process::Command::new("explorer")
        .raw_arg(format!("/select,\"{}\"", path.display()))
        .status()?;
    Ok(())
}

/// XDG でファイルを選択表示する
/// * `path` - ファイルのパス
#[cfg(target_os = "linux")]
fn reveal_file_command(path: &Path) -> error::Result<()> {
    let uri = file_uri(path);

    let result = std::process::Command::new("dbus-send")
        .arg("--session")
        .arg("--print-reply")
        .arg("--reply-timeout=2000")
        .arg("--dest=org.freedesktop.FileManager1")
        .arg("--type=method_call")
        .arg("/org/freedesktop/FileManager1")
        .arg("org.freedesktop.FileManager1.ShowItems")
        .arg(format!("array:string:\"{uri}\""))
        .arg("string:\"\"")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();

    // ShowItems が成功しなかった場合は親ディレクトリを開く
    if !matches!(result, Ok(status) if status.success()) {
        let parent = path.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(path);
        std::process::Command::new("xdg-open")
            .arg(parent)
            .status()?;
    }
    Ok(())
}

/// ファイルの URI を生成する
/// * `path` - ファイルのパス
/// * 戻り値 - ファイルの URI
#[cfg(target_os = "linux")]
fn file_uri(path: &Path) -> String {
    use std::os::unix::ffi::OsStrExt;

    let mut uri = String::from("file://");
    for &byte in path.as_os_str().as_bytes() {
        // パスの各バイトを URI エンコード
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' | b'/' => {
                uri.push(byte as char);
            }
            _ => uri.push_str(&format!("%{byte:02X}")),
        }
    }
    uri
}
