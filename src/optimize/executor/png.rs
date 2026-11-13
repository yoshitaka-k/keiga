use std::path::Path;
use image::GenericImageView;
use crate::{error, optimize};

struct LossyInfo {
    width: u32,
    height: u32,
    exif: Option<Vec<u8>>,
    icc: Option<Vec<u8>>,
}

/// PNG 最適化を行う構造体
pub struct Png;
impl optimize::Optimizer for Png {
    type Options = optimize::PngOptions;

    /// PNG ファイルを最適化
    /// * `path` - 最適化する PNG のパス
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

    /// PNG Lossy 最適化
    /// quantizr crate で最適化
    /// * `path` - 最適化する PNG のパス
    /// * `options` - 最適化オプション
    /// * `return` - 元のファイルサイズとエンコードされたデータ
    fn lossy(
        path: &Path,
        options: Self::Options
    ) -> error::Result<(usize, Vec<u8>)> {
        // 画像を読み込む
        let optimize::OpenImageResult { image, exif, icc } = Self::open_image(path)?;

        // 諸々情報を取得したり用意したりする
        let (width, height) = image.dimensions();
        let lossy_info = LossyInfo { width, height, exif, icc };

        // quantizr インスタンス
        let bytes = image.to_rgba8();
        let image = quantizr::Image::new(&bytes, lossy_info.width as usize, lossy_info.height as usize).map_err(|e| {
            error::KeigaError::optimized_error(e.to_string(), path.to_path_buf())
        })?;

        // quantizr オプション
        let mut opts = quantizr::Options::default();
        opts.set_max_colors(256).map_err(|e| {
            error::KeigaError::optimized_error(e.to_string(), path.to_path_buf())
        })?;

        // 最適化
        let mut result = quantizr::QuantizeResult::quantize(&image, &opts);
        let dithering_level = options.dithering as f32 / 100.0;
        result.set_dithering_level(dithering_level).map_err(|e| {
            error::KeigaError::optimized_error(e.to_string(), path.to_path_buf())
        })?;

        // インデックスを取得
        let mut indexes = vec![0u8; lossy_info.width as usize * lossy_info.height as usize];
        result.remap_image(&image, indexes.as_mut_slice()).map_err(|e| {
            error::KeigaError::optimized_error(e.to_string(), path.to_path_buf())
        })?;

        // パレットを取得
        let palette = result.get_palette();

        // 画像を保存
        let buffer = Self::write(palette, &indexes, lossy_info).map_err(|e| {
            error::KeigaError::optimized_error(e.to_string(), path.to_path_buf())
        })?;

        // 元ファイルのファイルサイズを取得
        let size = path.metadata().map_err(|e| {
            error::KeigaError::optimized_error(e.to_string(), path.to_path_buf())
        })?.len() as usize;

        Ok((size, buffer))
    }

    /// PNG Lossless 最適化
    /// oxipng crate で最適化
    /// * `path` - 最適化する PNG のパス
    /// * `options` - 最適化オプション
    /// * `return` - 元のファイルサイズとエンコードされたデータ
    fn lossless(
        path: &Path,
        options: Self::Options
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
}

impl Png {
    /// 画像を保存
    /// https://github.com/DarthSim/quantizr/blob/master/example/src/main.rs
    /// * `palette` - パレット
    /// * `indexes` - インデックス
    /// * `lossy_info` - PNG 情報
    /// * `return` - 保存結果
    fn write(
        palette: &quantizr::Palette,
        indexes: &[u8],
        lossy_info: LossyInfo,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {

        let mut rgb_palette = Vec::with_capacity((palette.count * 3) as usize);
        let mut trans = Vec::with_capacity(palette.count as usize);

        for e in palette.entries.iter().take(palette.count as usize) {
            rgb_palette.push(e.r);
            rgb_palette.push(e.g);
            rgb_palette.push(e.b);
            trans.push(e.a);
        }

        let mut buffer = Vec::new();
        {
            // 情報を作成
            let mut info = png::Info::with_size(lossy_info.width, lossy_info.height);

            // メタデータなんかを設定
            info.exif_metadata = lossy_info.exif.map(|e| e.into());
            info.icc_profile = lossy_info.icc.map(|i| i.into());

            // エンコーダーを作成
            let mut encoder = png::Encoder::with_info(&mut buffer, info)?;

            // 色々設定していく
            encoder.set_color(png::ColorType::Indexed);
            encoder.set_depth(png::BitDepth::Eight);
            encoder.set_palette(rgb_palette);
            encoder.set_trns(trans);

            // ヘッダーを書き込む
            let mut writer = encoder.write_header()?;

            // インデックスを書き込む
            writer.write_image_data(indexes)?;

            // ファイルを閉じる
            writer.finish()?;
        }
        Ok(buffer)
    }
}

// tests ディレクトリ直下は別クレートになって、非公開関数が呼べないので、
// 子モジュールとして読むためのパスを指定する
#[cfg(test)]
#[path = "../../tests/unit/optimize.rs"]
mod tests;
