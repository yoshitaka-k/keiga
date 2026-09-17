use std::path::PathBuf;
use crate::{error, optimize};
use image::GenericImageView;
use quantizr;

/// PNG 最適化を行う構造体
pub struct Png;

impl optimize::Optimizer for Png {
    type Options = optimize::PngOptions;

    /// PNG ファイルを最適化
    /// * `path` - 最適化する PNG のパス
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

    /// PNG Lossy 最適化
    /// quantizr crate で最適化
    /// * `path` - 最適化する PNG のパス
    /// * `options` - 最適化オプション
    /// * `return` - 元のファイルサイズとエンコードされたデータ
    fn lossy(
        path: &PathBuf,
        options: Self::Options
    ) -> error::Result<(usize, Vec<u8>)> {
        // 画像を読み込んで画像サイズを取得
        let img = image::open(path).map_err(|e| {
            error::KeigaError::OptimizedError(e.to_string(), path.clone())
        })?;
        let (width, height) = img.dimensions();
        let width = width as usize;
        let height = height as usize;

        // quantizr インスタンス
        let bytes = img.to_rgba8();
        let image = quantizr::Image::new(&bytes, width, height).map_err(|e| {
            error::KeigaError::OptimizedError(e.to_string(), path.clone())
        })?;

        // quantizr オプション
        let mut opts = quantizr::Options::default();
        opts.set_max_colors(256).map_err(|e| {
            error::KeigaError::OptimizedError(e.to_string(), path.clone())
        })?;

        // 最適化
        let mut result = quantizr::QuantizeResult::quantize(&image, &opts);
        let dithering_level = options.dithering as f32 / 100.0;
        result.set_dithering_level(dithering_level).map_err(|e| {
            error::KeigaError::OptimizedError(e.to_string(), path.clone())
        })?;

        // インデックスを取得
        let mut indexes = vec![0u8; width * height];
        result.remap_image(&image, indexes.as_mut_slice()).map_err(|e| {
            error::KeigaError::OptimizedError(e.to_string(), path.clone())
        })?;

        // パレットを取得
        let palette = result.get_palette();

        // 画像を保存
        let output = Self::save_image(&palette, &indexes, width, height).map_err(|e| {
            error::KeigaError::OptimizedError(e.to_string(), path.clone())
        })?;

        // 元ファイルのファイルサイズを取得
        let size = path.metadata().map_err(|e| {
            error::KeigaError::OptimizedError(e.to_string(), path.clone())
        })?.len() as usize;

        Ok((size, output))
    }

    /// PNG Lossless 最適化
    /// oxipng crate で最適化
    /// * `path` - 最適化する PNG のパス
    /// * `options` - 最適化オプション
    /// * `return` - 元のファイルサイズとエンコードされたデータ
    fn lossless(
        path: &PathBuf,
        options: Self::Options
    ) -> error::Result<(usize, Vec<u8>)> {
        // 先にファイルを読み込んでおく
        let input = std::fs::read(path).map_err(|e| {
            error::KeigaError::OptimizedError(e.to_string(), path.clone())
        })?;

        // oxipng でロスレス最適化（パレット維持・ビット深度削減・再圧縮）
        let output = oxipng::optimize_from_memory(&input, &options.options).map_err(|e| {
            error::KeigaError::OptimizedError(e.to_string(), path.clone())
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
    /// * `width` - 幅
    /// * `height` - 高さ
    /// * `return` - 保存結果
    fn save_image(
        palette: &quantizr::Palette,
        indexes: &Vec<u8>,
        width: usize,
        height: usize
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {

        let mut rgb_palette = Vec::with_capacity((palette.count * 3) as usize);
        let mut trans = Vec::with_capacity(palette.count as usize);

        for e in palette.entries.iter().take(palette.count as usize) {
            rgb_palette.push(e.r);
            rgb_palette.push(e.g);
            rgb_palette.push(e.b);
            trans.push(e.a);
        }

        let mut output = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut output, width as u32, height as u32);
            encoder.set_color(png::ColorType::Indexed);
            encoder.set_depth(png::BitDepth::Eight);
            encoder.set_palette(rgb_palette);
            encoder.set_trns(trans);
            let mut writer = encoder.write_header()?;

            writer.write_image_data(&indexes)?;
            writer.finish()?;
        }
        Ok(output)
    }
}
