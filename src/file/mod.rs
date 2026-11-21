mod open_files;
mod info;
mod image;
mod status;
mod token;
mod scan_job;
mod meta_job;

pub(crate) use open_files::OpenFiles;
pub(crate) use info::{FileInfo, StandbyLen, OptimizingLen, OptimizedLen, DurationInfo, SizeInfo, NewSizeInfo};
pub(crate) use image::Image;
pub(crate) use status::Status;
pub(crate) use token::ScanToken;
pub(crate) use scan_job::ScanJob;
pub(crate) use meta_job::MetaJob;

use std::path::{Path, PathBuf};

/// ファイルスキャン結果
/// * `Found` - ファイルが見つかった
/// * `Failed` - ファイルが見つからなかった
pub(crate) enum ScanResult {
    Found { path: PathBuf, base_dir: PathBuf },
    Failed { path: PathBuf, base_dir: PathBuf, error: String },
}

/// 画像の情報
/// * `Pending` - まだ読んでいない
/// * `Loading` - 読んでいる最中
/// * `Ready` - 読めた（拡張子）
/// * `Unavailable` - 読めなかった
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Metadata {
    Pending,
    Loading,
    Ready(Extension),
    Unavailable,
}

/// ファイルの拡張子を管理する列挙型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// 節約率を計算
/// * `size` - 元のサイズ
/// * `new_size` - 最適化後のサイズ
/// * `return` - 節約率
pub(crate) fn calc_saved_rate(size: u64, new_size: u64) -> f32 {
    // サイズの比較
    let is_minus = size >= new_size;

    // 最適化後のサイズと元のサイズの差を計算
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

/// 平均値を計算
/// * `value` - 値
/// * `count` - 個数
/// * `return` - 平均値
pub(crate) fn calc_average_value(value: u64, count: u32) -> u64 {
    if count > 0 {
        value / count as u64
    } else {
        0
    }
}
