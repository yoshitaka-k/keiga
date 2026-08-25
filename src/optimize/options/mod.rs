mod status;
mod png_preset;
pub(crate) use status::OptimizeStatus;
pub(crate) use png_preset::PngPreset;

/// JPEG オプション
#[derive(Clone)]
pub struct JpegOptions {
    pub quality: u8,
}

/// PNG オプション
#[derive(Clone)]
pub struct PngOptions {
    pub options: oxipng::Options,
}
