use std::path::Path;
use crate::{error, optimize};

/// PNG Lossless 最適化
///
/// oxipng crate で最適化
///
/// * `path` - 最適化する PNG のパス
/// * `options` - 最適化オプション
/// * `return` - 元のファイルサイズとエンコードされたデータ
pub(crate) fn optimize(
    path: &Path,
    options: optimize::PngOptions
) -> error::Result<(usize, Vec<u8>)> {
    // 先にファイルを読み込んでおく
    let input = std::fs::read(path).map_err(|e| {
        error::KeigaError::optimized_error(e.to_string(), path.to_path_buf())
    })?;

    // oxipng でロスレス最適化（パレット維持・ビット深度削減・再圧縮）
    let output = oxipng::optimize_from_memory(&input, &options.options).map_err(|e| {
        error::KeigaError::optimized_error(e.to_string(), path.to_path_buf())
    })?;

    // 元ファイルのファイルサイズを取得
    let size = input.len();

    Ok((size, output))
}
