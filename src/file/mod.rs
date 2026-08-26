pub mod extension;
pub mod open_files;
pub mod image_file;

pub(crate) use open_files::OpenFiles;
pub(crate) use image_file::ImageFile;

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
    let saved_rate = if size == 0 {
        0.00f32
    } else {
        (calc_size as f32 / size as f32 * 100.0) * if is_minus { -1.0 } else { 1.0 }
    };

    saved_rate
}
