mod error;
mod update;

pub(crate) use error::error;
pub(crate) use update::updated;

/// 見出し横のアイコン
/// * `icon` - アイコン
/// * `size` - アイコンのサイズ
/// * `tint` - アイコンの色
/// * `return` - アイコンウィジェット
fn heading_icon(icon: egui::ImageSource<'static>, size: f32, tint: egui::Color32) -> egui::Image<'static> {
    // horizontal の行高 (interact_size.y = 18) に丸められるため、実サイズ (size) を指定する
    egui::Image::new(icon).fit_to_exact_size(egui::vec2(size, size)).tint(tint)
}
