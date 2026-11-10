pub(crate) mod constants;
pub(crate) mod fonts;
pub(crate) mod svg;

use crate::file;

/// アプリアイコン
pub(crate) const APP_ICON: egui::ImageSource<'static> = svg::bytes_source(
    "bytes://assets/icon.png",
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icon.png")),
);

/// ラベルと同じ色を返す
/// * `ui` - UI
/// * `return` - アイコンの色
pub(crate) fn icon_color(ui: &egui::Ui) -> egui::Color32 {
    ui.visuals().text_color()
}

/// リストなし時のテキスト色
/// * `ui` - UI
/// * `return` - リストなし時のテキストの色
pub(crate) fn list_empty_color(ui: &egui::Ui) -> egui::Color32 {
    if ui.ctx().global_style().visuals.dark_mode {
        constants::DARK_MODE_LIST_EMPTY_COLOR
    } else {
        constants::LIGHT_MODE_LIST_EMPTY_COLOR
    }
}

/// 警告アイコンの色
/// * `ui` - UI
/// * `return` - 警告アイコンの色
pub(crate) fn warning_color(ui: &egui::Ui) -> egui::Color32 {
    if ui.ctx().global_style().visuals.dark_mode {
        constants::DARK_MODE_WARNING_COLOR
    } else {
        constants::LIGHT_MODE_WARNING_COLOR
    }
}

/// ボタンアイコンの色
/// * `ui` - UI
/// * `return` - ボタンアイコンの色
pub(crate) fn button_icon_color(ui: &egui::Ui) -> egui::Color32 {
    if ui.ctx().global_style().visuals.dark_mode {
        constants::DARK_MODE_BUTTON_ICON_COLOR
    } else {
        constants::LIGHT_MODE_BUTTON_ICON_COLOR
    }
}

/// アイコンの色
/// * `ctx` - コンテキスト
/// * `status` - ステータス
/// * `return` - アイコンの色
pub(crate) fn status_icon_color(ctx: &egui::Context, status: file::Status) -> egui::Color32 {
    if ctx.global_style().visuals.dark_mode {
        match status {
            file::Status::Standby => constants::DARK_MODE_CIRCLE_COLOR,
            file::Status::Optimizing => constants::DARK_MODE_OPTIMIZING_COLOR,
            file::Status::Optimized => constants::DARK_MODE_OPTIMIZED_COLOR,
            file::Status::Unchanged => constants::DARK_MODE_UNCHANGED_COLOR,
            file::Status::Skipped => constants::DARK_MODE_SKIPPED_COLOR,
            file::Status::Canceled => constants::DARK_MODE_CANCELED_COLOR,
            file::Status::Error(_) => constants::DARK_MODE_ERROR_COLOR,
        }
    } else {
        match status {
            file::Status::Standby => constants::LIGHT_MODE_CIRCLE_COLOR,
            file::Status::Optimizing => constants::LIGHT_MODE_OPTIMIZING_COLOR,
            file::Status::Optimized => constants::LIGHT_MODE_OPTIMIZED_COLOR,
            file::Status::Unchanged => constants::LIGHT_MODE_UNCHANGED_COLOR,
            file::Status::Skipped => constants::LIGHT_MODE_SKIPPED_COLOR,
            file::Status::Canceled => constants::LIGHT_MODE_CANCELED_COLOR,
            file::Status::Error(_) => constants::LIGHT_MODE_ERROR_COLOR,
        }
    }
}
