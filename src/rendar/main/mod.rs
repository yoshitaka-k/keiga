pub(crate) mod view;
pub(crate) mod top;
pub(crate) mod list;
pub(crate) mod bottom;

// セパレータの高さ
pub(crate) const SEPARATOR_HEIGHT: f32 = 4.0;

// スピナーのサイズ
pub(crate) const SPINNER_SIZE: f32 = 8.0;

// アイコンセルの左パディング
pub(crate) const LIST_ICON_CELL_LEFT_PADDING: f32 = 2.0;
pub(crate) const LIST_ICON_STANDBY_LEFT_PADDING: f32 = 1.0;
pub(crate) const BOTTOM_ICON_CELL_LEFT_PADDING: f32 = 4.0;

// リストのノートのサイズ
pub(crate) const LIST_NOTE_SIZE: f32 = 11.0;
// リストの角丸
pub(crate) const LIST_CORNER_RADIUS: f32 = 1.0;

/// 交互に表示する背景色を取得
/// * `ui` - UI
/// * `return` - 交互に表示する背景色
pub(crate) fn alternate_background_color(ui: &egui::Ui) -> egui::Color32 {
    if ui.ctx().global_style().visuals.dark_mode {
        egui::Color32::from_rgba_unmultiplied(255, 255, 255, 5)
    } else {
        egui::Color32::from_rgba_unmultiplied(0, 0, 0, 10)
    }
}

/// 選択されている背景色を取得
/// * `ui` - UI
/// * `return` - 選択されている背景色
pub(crate) fn selected_background_color(ui: &egui::Ui) -> egui::Color32 {
    if ui.ctx().global_style().visuals.dark_mode {
        egui::Color32::from_rgba_unmultiplied(20, 120, 130, 50)
    } else {
        egui::Color32::from_rgba_unmultiplied(20, 120, 130, 50)
    }
}
