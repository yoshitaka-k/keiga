pub(crate) mod constants;
pub(crate) mod fonts;
pub(crate) mod svg;
pub(crate) mod sounds;

pub(crate) use sounds::SoundPlayer;

use crate::optimize::OptimizeStatus;

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
pub(crate) fn status_icon_color(ctx: &egui::Context, status: OptimizeStatus) -> egui::Color32 {
    if ctx.global_style().visuals.dark_mode {
        match status {
            OptimizeStatus::Standby => constants::DARK_MODE_CIRCLE_COLOR,
            OptimizeStatus::Optimizing => constants::DARK_MODE_OPTIMIZING_COLOR,
            OptimizeStatus::Optimized => constants::DARK_MODE_OPTIMIZED_COLOR,
            OptimizeStatus::Unchanged => constants::DARK_MODE_UNCHANGED_COLOR,
            OptimizeStatus::Skipped => constants::DARK_MODE_SKIPPED_COLOR,
            OptimizeStatus::Canceled => constants::DARK_MODE_CANCELED_COLOR,
            OptimizeStatus::Error(_) => constants::DARK_MODE_ERROR_COLOR,
        }
    } else {
        match status {
            OptimizeStatus::Standby => constants::LIGHT_MODE_CIRCLE_COLOR,
            OptimizeStatus::Optimizing => constants::LIGHT_MODE_OPTIMIZING_COLOR,
            OptimizeStatus::Optimized => constants::LIGHT_MODE_OPTIMIZED_COLOR,
            OptimizeStatus::Unchanged => constants::LIGHT_MODE_UNCHANGED_COLOR,
            OptimizeStatus::Skipped => constants::LIGHT_MODE_SKIPPED_COLOR,
            OptimizeStatus::Canceled => constants::LIGHT_MODE_CANCELED_COLOR,
            OptimizeStatus::Error(_) => constants::LIGHT_MODE_ERROR_COLOR,
        }
    }
}
