mod main;
mod assets;
mod setting;
mod modal;

pub use main::view::Rendar;

// モーダルウィンドウの幅
pub(crate) const MODAL_WINDOW_WIDTH: f32 = 280.0;
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

/// アイコンウィジェットを作成
/// * `icon` - アイコン
/// * `size` - アイコンのサイズ
/// * `color` - アイコンの色
/// * `return` - アイコンウィジェット
pub(crate) fn icon_widget(icon: egui::ImageSource<'static>, size: f32, color: egui::Color32) -> impl egui::Widget {
    egui::Image::new(icon).max_height(size).tint(color)
}

/// スピナーウィジェットを作成
/// * `size` - スピナーのサイズ
/// * `color` - スピナーの色
/// * `return` - スピナーウィジェット
pub(crate) fn spinner_widget(size: f32, color: egui::Color32) -> impl egui::Widget {
    egui::Spinner::new().size(size).color(color)
}
