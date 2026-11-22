#![warn(clippy::all, rust_2018_idioms)]

mod config;
mod file;
mod state;
mod ui;
mod optimize;
mod event;
mod error;
mod sounds;
mod update;
mod render;
mod lang;

pub use config::Config;
pub use render::Render;

/// アプリケーション名
pub const APP_NAME: &str = "Keiga";

/// GitHub リポジトリ URL
pub(crate) const GITHUB_URL: &str = "https://github.com/{repository}";

/// アップデート確認リクエスト URL
pub(crate) const REQUEST_URL: &str = "https://api.github.com/repos/{repository}/releases/latest";

/// ファイルサイズをフォーマットするマクロ
pub fn filesize_format(size: u64) -> String {
    if size < 1024 {
        format!("{:.2} B", size as f64)
    } else if size < 1024 * 1024 {
        format!("{:.2} KB", size as f64 / 1024.0)
    } else if size < 1024 * 1024 * 1024 {
        format!("{:.2} MB", size as f64 / 1024.0 / 1024.0)
    } else {
        format!("{:.2} GB", size as f64 / 1024.0 / 1024.0 / 1024.0)
    }
}

/// 最適化時間をフォーマットするマクロ
pub fn duration_format(duration: u64) -> String {
    if duration < 1000 {
        format!("{:.2} ms", duration as f64)
    } else if duration < 60 * 1000 {
        format!("{:.2} s", duration as f64 / 1000.0)
    } else if duration < 60 * 60 * 1000 {
        format!("{:.2} m", duration as f64 / 60.0 / 1000.0)
    } else {
        format!("{:.2} h", duration as f64 / 60.0 / 60.0 / 1000.0)
    }
}
