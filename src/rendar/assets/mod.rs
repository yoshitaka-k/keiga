pub(crate) mod constants;
pub(crate) mod fonts;
pub(crate) mod svg;

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

/// 最適化中アイコンの色
/// * `ui` - UI
/// * `return` - 最適化中アイコンの色
pub(crate) fn optimizing_color(ui: &egui::Ui) -> egui::Color32 {
    if ui.ctx().global_style().visuals.dark_mode {
        constants::DARK_MODE_OPTIMIZING_COLOR
    } else {
        constants::LIGHT_MODE_OPTIMIZING_COLOR
    }
}

/// 最適化済みアイコンの色
/// * `ui` - UI
/// * `return` - 最適化済みアイコンの色
pub(crate) fn optimized_color(ui: &egui::Ui) -> egui::Color32 {
    if ui.ctx().global_style().visuals.dark_mode {
        constants::DARK_MODE_OPTIMIZED_COLOR
    } else {
        constants::LIGHT_MODE_OPTIMIZED_COLOR
    }
}

/// 最適化不要アイコンの色
/// * `ui` - UI
/// * `return` - 最適化不要アイコンの色
pub(crate) fn unchanged_color(ui: &egui::Ui) -> egui::Color32 {
    if ui.ctx().global_style().visuals.dark_mode {
        constants::DARK_MODE_UNCHANGED_COLOR
    } else {
        constants::LIGHT_MODE_UNCHANGED_COLOR
    }
}

/// スキップアイコンの色
/// * `ui` - UI
/// * `return` - スキップアイコンの色
pub(crate) fn skipped_color(ui: &egui::Ui) -> egui::Color32 {
    if ui.ctx().global_style().visuals.dark_mode {
        constants::DARK_MODE_SKIPPED_COLOR
    } else {
        constants::LIGHT_MODE_SKIPPED_COLOR
    }
}

/// キャンセルアイコンの色
/// * `ui` - UI
/// * `return` - キャンセルアイコンの色
pub(crate) fn canceled_color(ui: &egui::Ui) -> egui::Color32 {
    if ui.ctx().global_style().visuals.dark_mode {
        constants::DARK_MODE_CANCELED_COLOR
    } else {
        constants::LIGHT_MODE_CANCELED_COLOR
    }
}

/// エラーアイコンの色
/// * `ui` - UI
/// * `return` - エラーアイコンの色
pub(crate) fn error_color(ui: &egui::Ui) -> egui::Color32 {
    if ui.ctx().global_style().visuals.dark_mode {
        constants::DARK_MODE_ERROR_COLOR
    } else {
        constants::LIGHT_MODE_ERROR_COLOR
    }
}

/// 丸アイコンの色
/// * `ui` - UI
/// * `return` - 丸アイコンの色
pub(crate) fn circle_color(ui: &egui::Ui) -> egui::Color32 {
    if ui.ctx().global_style().visuals.dark_mode {
        constants::DARK_MODE_CIRCLE_COLOR
    } else {
        constants::LIGHT_MODE_CIRCLE_COLOR
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
