use std::path::PathBuf;
use std::panic;
use image::{
    ImageReader,
    codecs::jpeg::JpegEncoder,
};
use mozjpeg;
use crate::{error, optimize};

/// JPEG 最適化を行う構造体
pub struct Jpeg;

impl optimize::Optimizer for Jpeg {
    type Options = optimize::JpegOptions;

    /// JPEG ファイルを最適化
    /// * `path` - 最適化する JPEG のパス
    /// * `options` - 最適化オプション
    /// * `return` - 元のファイルサイズとエンコードされたデータ
    fn encode(
        path: &PathBuf,
        options: Self::Options,
    ) -> error::Result<(usize, Vec<u8>)> {
        if options.lossy {
            Self::lossy(path, options)
        } else {
            Self::lossless(path, options)
        }
    }

    /// JPEG Lossy 最適化
    /// image crate で最適化
    /// * `path` - 最適化する JPEG のパス
    /// * `options` - 最適化オプション
    /// * `return` - 元のファイルサイズとエンコードされたデータ
    fn lossy(
        path: &PathBuf,
        options: Self::Options
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

    /// JPEG Lossless 最適化
    /// mozjpeg crate で最適化
    /// * `path` - 最適化する JPEG のパス
    /// * `options` - 最適化オプション
    /// * `return` - 元のファイルサイズとエンコードされたデータ
    fn lossless(
        path: &PathBuf,
        _options: Self::Options
    ) -> error::Result<(usize, Vec<u8>)> {
        // ファイルを読み込む
        let input = std::fs::read(path).map_err(|e| {
            error::KeigaError::OptimizedError(e.to_string(), path.clone())
        })?;

        // mozjpegの処理全体を catch_unwind 内に閉じ込める
        let result = panic::catch_unwind(|| -> std::io::Result<Vec<u8>> {
            // 画像を読み込む
            let d = mozjpeg::Decompress::new_mem(&input)?;
            let (width, height) = d.size();
            let mut image = d.to_colorspace(mozjpeg::ColorSpace::JCS_YCbCr)?;
            let pixels = image.read_scanlines::<u8>()?;

            // mozjpeg のコンパレッサーを作成
            let mut comp = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_YCbCr);
            comp.set_size(width, height);
            // 擬似可逆圧縮を使用
            comp.set_quality(100.0);
            // サンプリングを 4:4:4 に設定
            comp.set_chroma_sampling_pixel_sizes((1, 1), (1, 1));


            // コンパレッサーを開始
            let mut comp_start = comp.start_compress(Vec::new())?;
            comp_start.write_scanlines(&pixels)?;
            let output = comp_start.finish()?;

            Ok(output)
        });

        match result {
            Ok(Ok(output)) => {
                // ファイルサイズを取得
                let size = input.len();

                Ok((size, output))
            }
            // 通常のエラー
            Ok(Err(e)) => Err(error::KeigaError::OptimizedError(e.to_string(), path.clone())),
            // パニック
            Err(payload) => {
                let msg = payload
                    .downcast_ref::<String>()
                    .cloned()
                    .unwrap_or_else(|| "mozjpeg error".to_string());
                Err(error::KeigaError::OptimizedError(msg, path.clone()))
            },
        }
    }
}
