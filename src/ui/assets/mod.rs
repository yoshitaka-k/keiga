pub(crate) mod color;
pub(crate) mod size;
pub(crate) mod font;
pub(crate) mod svg;

use crate::{config, file, ui};

/// アプリアイコン
pub(crate) const APP_ICON: egui::ImageSource<'static> = svg::bytes_source(
    "bytes://assets/icon.png",
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icon.png")),
);

/// アイコンの色
/// * `config` - 設定
/// * `status` - ステータス
/// * `return` - アイコンの色
pub(crate) fn status_icon_color(config: &config::Config, status: file::Status) -> egui::Color32 {
    if *config.theme() == ui::Theme::Dark {
        match status {
            file::Status::Standby => color::DARK_MODE_CIRCLE_COLOR,
            file::Status::Optimizing => color::DARK_MODE_OPTIMIZING_COLOR,
            file::Status::Optimized => color::DARK_MODE_OPTIMIZED_COLOR,
            file::Status::Unchanged => color::DARK_MODE_UNCHANGED_COLOR,
            file::Status::Skipped => color::DARK_MODE_SKIPPED_COLOR,
            file::Status::Canceled => color::DARK_MODE_CANCELED_COLOR,
            file::Status::Error(_) => color::DARK_MODE_ERROR_COLOR,
        }
    } else {
        match status {
            file::Status::Standby => color::LIGHT_MODE_CIRCLE_COLOR,
            file::Status::Optimizing => color::LIGHT_MODE_OPTIMIZING_COLOR,
            file::Status::Optimized => color::LIGHT_MODE_OPTIMIZED_COLOR,
            file::Status::Unchanged => color::LIGHT_MODE_UNCHANGED_COLOR,
            file::Status::Skipped => color::LIGHT_MODE_SKIPPED_COLOR,
            file::Status::Canceled => color::LIGHT_MODE_CANCELED_COLOR,
            file::Status::Error(_) => color::LIGHT_MODE_ERROR_COLOR,
        }
    }
}

/// リストなし時のテキスト色
/// * `ui` - UI
/// * `return` - リストなし時のテキストの色
pub(crate) fn list_empty_color(config: &config::Config) -> egui::Color32 {
    if *config.theme() == ui::Theme::Dark {
        color::DARK_MODE_LIST_EMPTY_COLOR
    } else {
        color::LIGHT_MODE_LIST_EMPTY_COLOR
    }
}

/// タブの選択時の背景色を取得
/// * `ui` - UI
/// * `return` - タブの選択時の背景色
pub(crate) fn tab_selected_color(config: &config::Config) -> egui::Color32 {
    if *config.theme() == ui::Theme::Dark {
        ui::DARK_TAB_SELECTED_COLOR
    } else {
        ui::LIGHT_TAB_SELECTED_COLOR
    }
}

/// 警告アイコンの色
/// * `ui` - UI
/// * `return` - 警告アイコンの色
pub(crate) fn warning_color(config: &config::Config) -> egui::Color32 {
    if *config.theme() == ui::Theme::Dark {
        color::DARK_MODE_WARNING_COLOR
    } else {
        color::LIGHT_MODE_WARNING_COLOR
    }
}

/// ボタンアイコンの色
/// * `ui` - UI
/// * `return` - ボタンアイコンの色
pub(crate) fn button_icon_color(config: &config::Config) -> egui::Color32 {
    if *config.theme() == ui::Theme::Dark {
        color::DARK_MODE_BUTTON_ICON_COLOR
    } else {
        color::LIGHT_MODE_BUTTON_ICON_COLOR
    }
}
