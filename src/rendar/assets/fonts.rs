use std::sync::Arc;
use egui::{FontData, FontDefinitions, FontFamily};

mod generated {
    include!(concat!(env!("OUT_DIR"), "/fonts_generated.rs"));
}

/// egui に assets/fonts のフォントを登録する
/// * `ctx` - コンテキスト
pub fn install(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    for &(name, bytes) in generated::FONTS {
        fonts.font_data
             .insert(name.to_owned(), Arc::new(FontData::from_static(bytes)));

        // デフォルトフォントに無いフォントはフォールバック
        // プロポーショナルフォント
        if let Some(family) = fonts.families.get_mut(&FontFamily::Proportional) {
            family.push(name.to_owned());
        }
        // 等幅フォント
        if let Some(family) = fonts.families.get_mut(&FontFamily::Monospace) {
            family.push(name.to_owned());
        }
    }

    ctx.set_fonts(fonts);
}

/// フォントカラーを設定する
/// * `text` - テキスト
/// * `color` - カラー
/// * `size` - 文字サイズ
/// * `return` - テキスト
pub fn text_color(text: &str, color: egui::Color32, size: Option<f32>) -> egui::RichText {
    let mut rich_text = egui::RichText::new(text).color(color);
    if let Some(size) = size {
        rich_text = rich_text.size(size);
    }
    rich_text
}
