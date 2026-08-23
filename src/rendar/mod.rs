mod main;
mod assets;
mod setting;
mod modal;

pub use main::view::Rendar;

// パネルの背景色
pub(crate) const DARK_MODE_PANEL_COLOR: egui::Color32 = egui::Color32::from_rgb(35, 35, 35);
pub(crate) const LIGHT_MODE_PANEL_COLOR: egui::Color32 = egui::Color32::from_rgb(225, 225, 225);

/// 設定タブ
#[derive(PartialEq)]
pub enum SettingTab {
    Concurrent,
    Quality,
    About,
}

impl SettingTab {
    fn to_string(&self) -> &str {
        match self {
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

/// 設定ウィンドウを表示するためのトークン
pub struct SettingToken {
    pub open: bool,
    pub pos: Option<egui::Pos2>,
    pub tab: SettingTab,
}

/// エラーモーダルを表示するためのトークン
pub struct ErrorToken {
    pub open: bool,
    pub value: Option<Box<dyn std::error::Error>>,
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
/// * `return` - パネルのスタイル
pub(crate) fn panel_style(ui: &mut egui::Ui) -> egui::Frame {
    let panel_fill_color = panel_fill_color(ui);
    egui::Frame::default()
        .fill(panel_fill_color)
        .inner_margin(egui::Margin {
            left: 10,
            right: 10,
            top: 2,
            bottom: 3,
        })
}
