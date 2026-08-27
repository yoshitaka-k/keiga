#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod file;
mod rendar;
mod optimize;
mod event;
mod error;

pub use app::App;
pub use error::KeigaError;
pub use file::open_files::OpenFiles;
pub use rendar::Rendar;
pub use optimize::Jpeg;

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

/// バージョンを比較するマクロ
pub fn version_compare(new: &str, old: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let new = new.replace("v", "");
    let old = old.replace("v", "");

    // 新しいバージョンをメジャー、マイナー、パッチに分割
    let parts: Vec<i32> = new.split(".")
        .map(|x| x.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;
    let [new_major, new_minor, new_patch]: [i32; 3] = parts.try_into()
        .map_err(|_| "Invalid latest version")?;

    // 古いバージョンをメジャー、マイナー、パッチに分割
    let parts: Vec<i32> = old.split(".")
        .map(|x| x.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;
    let [old_major, old_minor, old_patch]: [i32; 3] = parts.try_into()
        .map_err(|_| "Invalid current version")?;

    // メジャー、マイナー、パッチを比較
    if new_major != old_major {
        // メジャー番号が異なる場合は、メジャー番号の比較
        Ok(new_major > old_major)
    } else if new_minor != old_minor {
        // マイナー番号が異なる場合は、マイナー番号を比較
        Ok(new_minor > old_minor)
    } else {
        // 上記以外の場合は、パッチ番号を比較
        Ok(new_patch > old_patch)
    }
}
