mod open_files;
mod image;
mod status;
mod token;
mod scan_job;

pub(crate) use open_files::OpenFiles;
pub(crate) use image::Image;
pub(crate) use status::Status;
pub(crate) use token::ScanToken;
pub(crate) use scan_job::ScanJob;

use std::path::{Path, PathBuf};

/// ファイルスキャン結果
/// * `Found` - ファイルが見つかった
/// * `Failed` - ファイルが見つからなかった
pub(crate) enum ScanResult {
    Found { path: PathBuf, base_dir: PathBuf },
    Failed { path: PathBuf, base_dir: PathBuf, error: String },
}

/// ファイルの拡張子を管理する列挙型
#[derive(Clone)]
pub(crate) enum Extension {
    Jpeg,
    Png,
    None,
}

/// 読み込める拡張子
/// * `return` - 読み込める拡張子
pub(crate) fn allowed_extensions() -> &'static [&'static str] {
    &["png", "jpg", "jpeg"]
}

/// ファイルの拡張子が許可されているかどうかを確認
/// * `path` - ファイルのパス
/// * `return` - 許可されているかどうか
fn is_allowed_extension(path: &Path) -> bool {
    let extensions = allowed_extensions();
    if let Some(ext) = path.extension()
        && let Some(ext) = ext.to_str()
        && extensions.iter().any(|e| e.eq_ignore_ascii_case(ext))
    {
        return true;
    }
    false
}

/// 最適化済みのファイル数を管理する構造体
#[derive(Clone, Copy)]
struct OptimizedLen {
    jpeg: u32,
    png: u32,
}

impl OptimizedLen {
    pub fn new() -> Self {
        Self {
            jpeg: 0,
            png: 0,
        }
    }
    pub fn total(&self) -> u32 {
        self.jpeg + self.png
    }
}

/// 最適化時間を管理する構造体
#[derive(Clone, Copy)]
struct DurationInfo {
    jpeg: u64,
    png: u64,
}

impl DurationInfo {
    pub fn new() -> Self {
        Self {
            jpeg: 0,
            png: 0,
        }
    }
    pub fn total(&self) -> u64 {
        self.jpeg + self.png
    }
}

/// ファイル情報を管理する構造体
#[derive(Clone, Copy)]
struct FileInfo {
    standby_len: u32,
    optimizing_len: u32,
    optimized_len: OptimizedLen,
    unchanged_len: u32,
    skipped_len: u32,
    canceled_len: u32,
    error_len: u32,
    completed_len: u32,
    total_size: u64,
    total_new_size: u64,
    total_duration: DurationInfo,
    total_saved_rate: f32,
    average_duration: u64,
    jpeg_average_duration: u64,
    png_average_duration: u64,
}

impl FileInfo {
    pub fn new() -> Self {
        Self {
            standby_len: 0,
            optimizing_len: 0,
            optimized_len: OptimizedLen::new(),
            unchanged_len: 0,
            skipped_len: 0,
            canceled_len: 0,
            error_len: 0,
            completed_len: 0,
            total_size: 0,
            total_new_size: 0,
            total_duration: DurationInfo::new(),
            total_saved_rate: 0.00,
            average_duration: 0,
            jpeg_average_duration: 0,
            png_average_duration: 0,
        }
    }
}

/// 節約率を計算
/// * `size` - 元のサイズ
/// * `new_size` - 最適化後のサイズ
/// * `return` - 節約率
pub(crate) fn calc_saved_rate(size: u64, new_size: u64) -> f32 {
    // ファイルサイズの比較
    let is_minus = size >= new_size;

    // 最適化後のファイルサイズと元のファイルサイズの差を計算
    let calc_size = if is_minus {
        size - new_size
    } else {
        new_size - size
    };

    // 節約率を計算


    if size == 0 {
        0.00f32
    } else {
        (calc_size as f32 / size as f32 * 100.0) * if is_minus { -1.0 } else { 1.0 }
    }
}
