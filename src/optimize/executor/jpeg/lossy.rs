use std::path::Path;
use crate::{error, optimize};
use crate::optimize::executor::jpeg::common;

/// JPEG Lossy 最適化
///
/// common::encode_with を使用して最適化する準備を行う
///
/// * `path` - 最適化する JPEG のパス
/// * `options` - 最適化オプション
/// * `return` - 元のファイルサイズとエンコードされたデータ
pub(crate) fn optimize(
    path: &Path,
    options: optimize::JpegOptions
) -> error::Result<(usize, Vec<u8>)> {
    let configure = |comp: &mut mozjpeg::Compress, color_space: mozjpeg::ColorSpace| {
        // 品質を設定
        comp.set_quality(options.quality as f32);
        // YCbCr の場合はサンプリングを 4:2:0 に設定
        if color_space == mozjpeg::ColorSpace::JCS_YCbCr {
            comp.set_chroma_sampling_pixel_sizes((2, 2), (2, 2));
        }
        // プログレッシブ JPEG を使用
        comp.set_progressive_mode();
        // プログレッシブJPEGの各スキャン構成の自動最適化を有効化
        comp.set_optimize_scans(true);
    };

    common::encode_with(path, configure)
}
