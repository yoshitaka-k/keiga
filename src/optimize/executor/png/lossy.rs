use std::path::Path;
use image::GenericImageView;
use crate::{error, optimize};
use crate::optimize::executor;

/// PNG Lossy 最適化情報
struct LossyInfo {
    width: u32,
    height: u32,
    exif: Option<Vec<u8>>,
    icc: Option<Vec<u8>>,
}

/// PNG Lossy 最適化
/// quantizr crate で最適化
/// * `path` - 最適化する PNG のパス
/// * `options` - 最適化オプション
/// * `return` - 元のファイルサイズとエンコードされたデータ
pub(crate) fn optimize(
    path: &Path,
    options: optimize::PngOptions
) -> error::Result<(usize, Vec<u8>)> {
    // 画像を読み込む
    let executor::OpenImageResult { image, exif, icc } = executor::open_image(path)?;

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
    let buffer = write(palette, &indexes, lossy_info).map_err(|e| {
        error::KeigaError::optimized_error(e.to_string(), path.to_path_buf())
    })?;

    // 元ファイルのファイルサイズを取得
    let size = path.metadata().map_err(|e| {
        error::KeigaError::optimized_error(e.to_string(), path.to_path_buf())
    })?.len() as usize;

    Ok((size, buffer))
}
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
