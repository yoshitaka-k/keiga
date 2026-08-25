use std::path::PathBuf;
use image::ImageReader;
use image::codecs::jpeg::JpegEncoder;
use crate::optimize;

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
    ) -> Result<(usize, Vec<u8>), Box<dyn std::error::Error>> {
        // メモリ上にバッファを作成して最適化
        let mut buffer = Vec::new();
        {
            // ファイルを読み込む
            let file_image = ImageReader::open(&path)?.decode()?;

            // JPEG エンコーダーを作成して最適化
            let mut encoder = JpegEncoder::new_with_quality(&mut buffer, options.quality);
            encoder.encode_image(&file_image)?;
        }

        // 最適化後のサイズが元のサイズより大きい場合は最適化しない
        let size = path.metadata()?.len() as usize;

        Ok((size, buffer))
    }
}
