use std::path::Path;
use std::panic;
use image::{self, ImageDecoder, DynamicImage};
use image::codecs::jpeg::JpegEncoder;
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
        path: &Path,
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
        path: &Path,
        options: Self::Options
    ) -> error::Result<(usize, Vec<u8>)> {
        // メモリ上にバッファを作成して最適化
        let mut buffer = Vec::new();
        {
            // 画像を読み込む
            let reader = image::ImageReader::open(path)
                .and_then(|r| r.with_guessed_format())
                .map_err(|e| error::KeigaError::optimized_error(e.to_string(), path.to_path_buf()))?;

            // 画像のフォーマットを取得
            // let format = reader.format().ok_or_else(|| {
            //     error::KeigaError::optimized_error("Unknown image format".to_string(), path.to_path_buf())
            // })?;

            // リーダーをデコーダーに変換
            let mut decoder = reader.into_decoder().map_err(|e| {
                error::KeigaError::optimized_error(e.to_string(), path.to_path_buf())
            })?;

            // 情報を取得
            // 向きのメタデータを取得
            let orientation = decoder.orientation().map_err(|e| {
                error::KeigaError::optimized_error(e.to_string(), path.to_path_buf())
            })?;
            // EXIF メタデータを取得
            let exif = decoder.exif_metadata().map_err(|e| {
                error::KeigaError::optimized_error(e.to_string(), path.to_path_buf())
            })?;

            // ICC プロファイルを取得
            // image は CMYK を RGB に変換してデコードするため、CMYK 用の ICC は付けない
            let icc = decoder.icc_profile().map_err(|e| {
                error::KeigaError::optimized_error(e.to_string(), path.to_path_buf())
            })?.filter(|icc| icc.get(16..20) != Some(b"CMYK"));


            // 画像をデコードからピクセルデータへ変換
            let mut image = DynamicImage::from_decoder(decoder).map_err(|e| {
                error::KeigaError::optimized_error(e.to_string(), path.to_path_buf())
            })?;

            // 画像の向きを適用
            image.apply_orientation(orientation);

            // JPEG エンコーダーを作成して最適化
            Self::write(&image, JpegEncoder::new_with_quality(&mut buffer, options.quality), exif, icc, path)?;
        }

        // ファイルサイズを取得
        let size = path.metadata().map_err(|e| {
            error::KeigaError::optimized_error(e.to_string(), path.to_path_buf())
        })?.len() as usize;

        Ok((size, buffer))
    }

    /// JPEG Lossless 最適化
    /// mozjpeg crate で最適化
    /// * `path` - 最適化する JPEG のパス
    /// * `options` - 最適化オプション
    /// * `return` - 元のファイルサイズとエンコードされたデータ
    fn lossless(
        path: &Path,
        _options: Self::Options
    ) -> error::Result<(usize, Vec<u8>)> {
        // ファイルを読み込む
        let input = std::fs::read(path).map_err(|e| {
            error::KeigaError::optimized_error(e.to_string(), path.to_path_buf())
        })?;

        // mozjpegの処理全体を catch_unwind 内に閉じ込める
        let result = panic::catch_unwind(|| -> std::io::Result<Vec<u8>> {
            // 画像を読み込む
            // EXIF, XMP, ICC を保持するため APP1, APP2 を指定
            let d = mozjpeg::Decompress::builder()
                 .with_markers(&[mozjpeg::Marker::APP(1), mozjpeg::Marker::APP(2)])
                 .from_mem(&input)?;

            // EXIF, XMP, ICC を取り出して保持
            let markers: Vec<(mozjpeg::Marker, Vec<u8>)> = d.markers()
                .map(|m| (m.marker, m.data.to_owned()))
                .collect();

            let (width, height) = d.size();
            let color_space = Self::colorspace(d.color_space());
            let mut image = d.to_colorspace(color_space)?;
            let pixels = image.read_scanlines::<u8>()?;

            // mozjpeg のコンプレッサーを作成
            let mut comp = mozjpeg::Compress::new(color_space);
            comp.set_size(width, height);
            // 擬似可逆圧縮を使用
            comp.set_quality(100.0);

            // YCbCr の場合はサンプリングを 4:4:4 に設定
            if color_space == mozjpeg::ColorSpace::JCS_YCbCr {
                comp.set_chroma_sampling_pixel_sizes((1, 1), (1, 1));
            }

            // コンプレッサーを開始
            let mut comp_start = comp.start_compress(Vec::new())?;

            // 保持している EXIF, XMP, ICC を書き込む
            for (marker, data) in &markers {
                comp_start.write_marker(*marker, data);
            }

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
            Ok(Err(e)) => Err(error::KeigaError::optimized_error(e.to_string(), path.to_path_buf())),
            // パニック
            Err(payload) => {
                let msg = payload
                    .downcast_ref::<String>()
                    .cloned()
                    .unwrap_or_else(|| "mozjpeg error".to_string());
                Err(error::KeigaError::optimized_error(msg, path.to_path_buf()))
            },
        }
    }
}

impl Jpeg {
    /// libjpeg が変換できる色空間へ揃える
    /// * `color_space` - 変換する色空間
    /// * `return` - 変換後の色空間
    fn colorspace(color_space: mozjpeg::ColorSpace) -> mozjpeg::ColorSpace {
        match color_space {
            mozjpeg::ColorSpace::JCS_GRAYSCALE => mozjpeg::ColorSpace::JCS_GRAYSCALE,
            mozjpeg::ColorSpace::JCS_CMYK => mozjpeg::ColorSpace::JCS_CMYK,
            mozjpeg::ColorSpace::JCS_YCCK => mozjpeg::ColorSpace::JCS_YCCK,
            _ => mozjpeg::ColorSpace::JCS_YCbCr,
        }
    }
}
