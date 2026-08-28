mod main;
mod assets;
mod setting;
mod modal;

pub use main::view::Rendar;

// モーダルウィンドウの幅
pub(crate) const MODAL_WINDOW_WIDTH: f32 = 320.0;
pub(crate) const MODAL_WINDOW_SPACING: f32 = 8.0;

// パネルの背景色
pub(crate) const DARK_MODE_PANEL_COLOR: egui::Color32 = egui::Color32::from_rgb(35, 35, 35);
pub(crate) const LIGHT_MODE_PANEL_COLOR: egui::Color32 = egui::Color32::from_rgb(225, 225, 225);

// パネルの内側のマージン
pub(crate) const PANEL_INNER_MARGIN: egui::Margin = egui::Margin {
    left: 8,
    right: 8,
    top: 0,
    bottom: 0,
};

// パネルの内側のマージン
pub(crate) const TOP_PANEL_INNER_MARGIN: egui::Margin = egui::Margin {
    left: 10,
    right: 10,
    top: 4,
    bottom: 4,
};

pub(crate) const BOTTOM_PANEL_INNER_MARGIN: egui::Margin = egui::Margin {
    left: 10,
    right: 10,
    top: 4,
    bottom: 6,
};

use getset::Getters;
use crate::error;
use crate::optimize::OptimizeStatus;

/// ステータスアイコンの色
#[derive(Getters)]
pub(crate) struct StatusColor {
    #[getset(get = "pub")]
    standby: egui::Color32,
    #[getset(get = "pub")]
    optimizing: egui::Color32,
    #[getset(get = "pub")]
    optimized: egui::Color32,
    #[getset(get = "pub")]
    unchanged: egui::Color32,
    #[getset(get = "pub")]
    skipped: egui::Color32,
    #[getset(get = "pub")]
    canceled: egui::Color32,
    #[getset(get = "pub")]
    error: egui::Color32,
}

impl StatusColor {
    /// 新しいステータスアイコンの色を作成
    /// * `ctx` - コンテキスト
    /// * `return` - ステータスアイコンの色
    pub fn new(ctx: &egui::Context) -> Self {
        Self {
            standby: assets::status_icon_color(&ctx, OptimizeStatus::Standby),
            optimizing: assets::status_icon_color(&ctx, OptimizeStatus::Optimizing),
            optimized: assets::status_icon_color(&ctx, OptimizeStatus::Optimized),
            unchanged: assets::status_icon_color(&ctx, OptimizeStatus::Unchanged),
            skipped: assets::status_icon_color(&ctx, OptimizeStatus::Skipped),
            canceled: assets::status_icon_color(&ctx, OptimizeStatus::Canceled),
            error: assets::status_icon_color(&ctx, OptimizeStatus::Error(String::new())),
        }
    }
}

/// 設定タブ
#[derive(PartialEq)]
pub enum SettingTab {
    General,
    Concurrent,
    Quality,
    About,
}

impl SettingTab {
    fn to_string(&self) -> &str {
        match self {
            SettingTab::General => "General",
            SettingTab::Concurrent => "Concurrent",
            SettingTab::Quality => "Quality",
            SettingTab::About => "About",
        }
    }
}

/// ファイル一覧を表示するためのトークン
pub struct ListRowToken {
    pub range: std::ops::Range<usize>,
    pub height: f32,
}

/// ファイルダイアログを表示するためのトークン
pub struct OpenDialogToken {
    pub file_dialog: bool,
    pub folder_dialog: bool,
}

/// 設定ウィンドウを表示するためのトークン
pub struct SettingToken {
    pub open: bool,
    pub pos: Option<egui::Pos2>,
    pub tab: SettingTab,
}

/// エラーモーダルを表示するためのトークン
pub struct ErrorToken {
    pub open: bool,
    pub value: Option<error::KeigaError>,
}

/// パネルの背景色
/// * `ui` - UI
/// * `return` - パネルの背景色
pub(crate) fn panel_fill_color(ui: &egui::Ui) -> egui::Color32 {
    if ui.ctx().global_style().visuals.dark_mode {
        DARK_MODE_PANEL_COLOR
    } else {
        LIGHT_MODE_PANEL_COLOR
    }
}

/// パネルのスタイルを設定
/// * `ui` - UI
/// * `inner_margin` - パネルの内側のマージン
/// * `return` - パネルのスタイル
pub(crate) fn panel_style(ui: &mut egui::Ui, inner_margin: egui::Margin) -> egui::Frame {
    let panel_fill_color = panel_fill_color(ui);
    egui::Frame::default()
        .fill(panel_fill_color)
        .inner_margin(inner_margin)
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
