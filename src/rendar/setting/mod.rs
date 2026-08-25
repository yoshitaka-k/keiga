pub(crate) mod view;
pub(crate) mod general;
pub(crate) mod concurrent;
pub(crate) mod quality;
pub(crate) mod about;

// ウィンドウのID
pub(crate) const SETTING_WINDOW_ID: &str = "setting_window";

/// ウィンドウのタイトル
pub(crate) const WINDOW_TITLE: &str = "Keiga Settings";

// ウィンドウのサイズ
pub(crate) const WINDOW_WIDTH: f32 = 480.0;
pub(crate) const WINDOW_HEIGHT: f32 = 270.0;

// ラベルの幅
pub(crate) const GENERAL_LABEL_WIDTH: f32 = 90.0;
pub(crate) const CONCURRENT_LABEL_WIDTH: f32 = 124.0;
pub(crate) const QUALITY_LABEL_WIDTH: f32 = 78.0;

// 追加のスペースの幅
pub(crate) const SETTING_ADD_SPACING: f32 = 4.0;

// タブの選択時の背景色
pub(crate) const DARK_TAB_SELECTED_COLOR: egui::Color32 = egui::Color32::from_rgb(20, 120, 130);
pub(crate) const LIGHT_TAB_SELECTED_COLOR: egui::Color32 = egui::Color32::from_rgb(130, 220, 210);

// 出力パスのテキストエディタの幅
pub(crate) const OUTPUT_PATH_TEXT_EDIT_WIDTH: f32 = 380.0;

// 効果音の音量のスライダーの幅
pub(crate) const SOUND_VOLUME_SLIDER_WIDTH: f32 = 318.0;

// 効果音の音量のスライダーの最小値と最大値
pub(crate) const SOUND_VOLUME_MIN: u8 = 1;
pub(crate) const SOUND_VOLUME_MAX: u8 = 10;

/// スライダーの幅
pub(crate) const CONCURRENT_SLIDER_WIDTH: f32 = 284.0;
pub(crate) const QUALITY_SLIDER_WIDTH: f32 = 330.0;

// 並行処理数の最小値と最大値
pub(crate) const OPTIMIZATION_NUM_MIN: u8 = 3;
pub(crate) const OPTIMIZATION_NUM_MAX: u8 = 8;

pub(crate) const PNG_OPTIMIZATION_NUM_MIN: u8 = 1;
pub(crate) const PNG_OPTIMIZATION_NUM_MAX: u8 = 3;

// JPEG の品質の最小値と最大値
pub(crate) const JPEG_QUALITY_MIN: u8 = 50;
pub(crate) const JPEG_QUALITY_MAX: u8 = 99;

/// タブの選択時の背景色を取得
/// * `ui` - UI
/// * `return` - タブの選択時の背景色
pub(crate) fn tab_selected_color(ui: &egui::Ui) -> egui::Color32 {
    if ui.ctx().global_style().visuals.dark_mode {
        DARK_TAB_SELECTED_COLOR
    } else {
        LIGHT_TAB_SELECTED_COLOR
    }
}

/// ラベルの幅を取得
/// * `ui` - UI
/// * `label` - ラベルのテキスト
/// * `width` - ラベルの幅
/// * `return` - ラベルのレスポンス
pub(crate) fn add_label(ui: &mut egui::Ui, label: &str, width: f32) -> egui::Response {
    ui.allocate_ui_with_layout(
        egui::vec2(width, ui.spacing().interact_size.y),
        egui::Layout::right_to_left(egui::Align::Center),
        |ui| {
            ui.set_min_width(width);
            ui.label(label)
        }
    ).inner
}
