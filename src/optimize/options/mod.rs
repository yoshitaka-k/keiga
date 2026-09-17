mod status;
mod png_preset;
pub(crate) use status::OptimizeStatus;
pub(crate) use png_preset::PngPreset;

/// JPEG オプション
/// * `lossy` - ロスレスではない場合は true
/// * `quality` - 品質
#[derive(Clone)]
pub struct JpegOptions {
    pub lossy: bool,
    pub quality: u8,
}

/// PNG オプション
/// * `lossy` - ロスレスではない場合は true
/// * `dithering` - ディザリング
/// * `options` - oxipng オプション
#[derive(Clone)]
pub struct PngOptions {
    pub lossy: bool,
    pub dithering: u8,
    pub options: oxipng::Options,
}
