use std::path::Path;
use crate::error;
use crate::optimize::executor::jpeg::common;

const LOSSLESS_QUALITY: f32 = 100.0;

/// JPEG Lossless 最適化
///
/// common::encode_with を使用して最適化する準備を行う
///
/// * `path` - 最適化する JPEG のパス
/// * `return` - 元のファイルサイズとエンコードされたデータ
pub(crate) fn optimize(
    path: &Path,
) -> error::Result<(usize, Vec<u8>)> {
    let configure = |comp: &mut mozjpeg::Compress, color_space: mozjpeg::ColorSpace| {
        // 品質を設定
        comp.set_quality(LOSSLESS_QUALITY);
        // YCbCr の場合はサンプリングを 4:4:4 に設定
        if color_space == mozjpeg::ColorSpace::JCS_YCbCr {
            comp.set_chroma_sampling_pixel_sizes((1, 1), (1, 1));
        }
    };

    common::encode_with(path, configure)
}
