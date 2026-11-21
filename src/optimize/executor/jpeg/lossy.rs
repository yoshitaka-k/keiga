use std::path::Path;
use image::{ImageEncoder, DynamicImage};
use image::codecs::jpeg::JpegEncoder;
use crate::{error, optimize};
use crate::optimize::executor;

/// JPEG Lossy 最適化
///
/// image crate で最適化
///
/// * `path` - 最適化する JPEG のパス
/// * `options` - 最適化オプション
/// * `return` - 元のファイルサイズとエンコードされたデータ
pub(crate) fn optimize(
    path: &Path,
    options: optimize::JpegOptions
) -> error::Result<(usize, Vec<u8>)> {
    // メモリ上にバッファを作成して最適化
    let mut buffer = Vec::new();
    {
        // 画像を読み込む
        let executor::OpenImageResult { image, exif, icc } = executor::open_image(path)?;

        // JPEG エンコーダーを作成して最適化
        write(&image, JpegEncoder::new_with_quality(&mut buffer, options.quality), exif, icc, path)?;
    }

    // ファイルサイズを取得
    let size = path.metadata().map_err(|e| {
        error::KeigaError::optimized_error(e.to_string(), path.to_path_buf())
    })?.len() as usize;

    Ok((size, buffer))
}

/// 画像をエンコード
/// * `image` - 画像
/// * `encoder` - エンコーダー
/// * `exif` - EXIF 情報
/// * `icc` - ICC プロファイル
/// * `return` - 画像をエンコードしたかどうか
fn write<E: ImageEncoder>(
    image: &DynamicImage,
    mut encoder: E,
    exif: Option<Vec<u8>>,
    icc: Option<Vec<u8>>,
    path: &Path,
) -> error::Result<()> {
    // EXIF メタデータを設定
    if let Some(exif) = exif {
        encoder.set_exif_metadata(exif).unwrap_or_else(|e| {
            log::warn!("Failed to set EXIF metadata: {}", e);
        });
    }

    // ICC プロファイルを設定
    if let Some(icc) = icc {
        encoder.set_icc_profile(icc).unwrap_or_else(|e| {
            log::warn!("Failed to set ICC profile: {}", e);
        });
    }

    image.write_with_encoder(encoder).map_err(|e| {
        error::KeigaError::optimized_error(e.to_string(), path.to_path_buf())
    })
}
