mod generated {
    include!(concat!(env!("OUT_DIR"), "/svg_generated.rs"));
}

pub use generated::*;

/// 埋め込み画像を egui の ImageSource にする
/// * `uri` - 画像の URI
/// * `bytes` - 画像のバイト列
/// * `return` - egui::ImageSource<'static>
pub(crate) const fn bytes_source(uri: &'static str, bytes: &'static [u8]) -> egui::ImageSource<'static> {
    egui::ImageSource::Bytes {
        uri: std::borrow::Cow::Borrowed(uri),
        bytes: egui::load::Bytes::Static(bytes),
    }
}

/// egui に画像ローダーを登録する
/// * `ctx` - egui::Context
pub fn install(ctx: &egui::Context) {
    egui_extras::install_image_loaders(ctx);
}
