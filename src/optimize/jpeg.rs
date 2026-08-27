use std::path::PathBuf;
use image::ImageReader;
use image::codecs::jpeg::JpegEncoder;
use crate::{error, optimize};

/// JPEG 最適化を行う構造体
pub struct Jpeg;

impl optimize::Optimizer for Jpeg {
    type Options = optimize::JpegOptions;

    /// JPEG ファイルを最適化
    /// * `path` - 最適化する JPEG のパス
    /// * `options` - 最適化オプション
    /// * `return` - エンコードされたファイルのサイズとデータ
    fn encode(
        path: &PathBuf,
        options: Self::Options,
    ) -> error::Result<(usize, Vec<u8>)> {
        // メモリ上にバッファを作成して最適化
        let mut buffer = Vec::new();
        {
            // ファイルを読み込む
            let file_image = ImageReader::open(&path).map_err(|e| {
                error::KeigaError::OptimizedError(e.to_string(), path.clone())
            })?.decode().map_err(|e| {
                error::KeigaError::OptimizedError(e.to_string(), path.clone())
            })?;

            // JPEG エンコーダーを作成して最適化
            let mut encoder = JpegEncoder::new_with_quality(&mut buffer, options.quality);
            encoder.encode_image(&file_image).map_err(|e| {
                error::KeigaError::OptimizedError(e.to_string(), path.clone())
            })?;
        }

        // ファイルサイズを取得
        let size = path.metadata().map_err(|e| {
            error::KeigaError::OptimizedError(e.to_string(), path.clone())
        })?.len() as usize;

        Ok((size, buffer))
    }
}
