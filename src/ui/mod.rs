mod main;
mod assets;
mod setting;
mod modal;

/// メイン描画関数を公開
pub(crate) use main::view as draw;

use serde::{Deserialize, Serialize};
use getset::Getters;
use crate::{config, file, lang};
use assets::{font, svg};

/// モーダルウィンドウの幅
const MODAL_WINDOW_WIDTH: f32 = 320.0;
/// モーダルウィンドウのスペーシング
const MODAL_WINDOW_SPACING: f32 = 8.0;

/// ダークモードパネルの背景色
const DARK_MODE_PANEL_COLOR: egui::Color32 = egui::Color32::from_rgb(35, 35, 35);
/// ライトモードパネルの背景色
const LIGHT_MODE_PANEL_COLOR: egui::Color32 = egui::Color32::from_rgb(225, 225, 225);

// メインウィンドウ

/// セパレータの高さ
const SEPARATOR_HEIGHT: f32 = 4.0;
/// スピナーのサイズ
const SPINNER_SIZE: f32 = 8.0;
/// アイコンセルの左パディング
const LIST_ICON_CELL_LEFT_PADDING: f32 = 2.0;
/// スタンバイアイコンセルの左パディング
const LIST_ICON_STANDBY_LEFT_PADDING: f32 = 1.0;
/// ボトムアイコンセルの左パディング
const BOTTOM_ICON_CELL_LEFT_PADDING: f32 = 4.0;
/// リストのノートのサイズ
const LIST_NOTE_SIZE: f32 = 11.0;
/// リストの角丸
const LIST_CORNER_RADIUS: f32 = 1.0;
/// リストの最大幅の割り算
const LIST_MAX_WIDTH_DIVIDER: f32 = 2.25;

// 設定ウィンドウ

/// ウィンドウのID
const SETTING_WINDOW_ID: &str = "setting_window";
/// ウィンドウの幅
const WINDOW_WIDTH: f32 = 480.0;
/// ウィンドウの高さ
const WINDOW_HEIGHT: f32 = 260.0;
/// ヘッダーのスペースの幅
const HEADER_ICON_SPACING: f32 = 4.0;
/// ヘッダーの下のスペースの幅
const HEADER_BOTTOM_SPACING: f32 = 2.0;
/// 警告アイコンのスペースの幅
const WARNING_ICON_SPACING: f32 = 3.0;
/// ラベルの幅
const GENERAL_LABEL_WIDTH: f32 = 90.0;
/// 品質のラベルの幅
const QUALITY_LABEL_WIDTH: f32 = 78.0;
/// 出力のラベルの幅
const OUTPUT_LABEL_WIDTH: f32 = 34.0;
/// 並行処理のラベルの幅
const CONCURRENT_LABEL_WIDTH: f32 = 50.0;
/// 追加のスペースの幅
const SETTING_ADD_SPACING: f32 = 4.0;
/// 設定のインデントのスペースの幅
const SETTING_INDENT_SPACING: f32 = 4.0;
/// ダークモードタブの選択時の背景色
const DARK_TAB_SELECTED_COLOR: egui::Color32 = egui::Color32::from_rgb(20, 120, 130);
/// ライトモードタブの選択時の背景色
const LIGHT_TAB_SELECTED_COLOR: egui::Color32 = egui::Color32::from_rgb(130, 220, 210);

// モーダル

/// モーダルのラベルの幅
const MODAL_LABEL_WIDTH: f32 = 130.0;

// パネル

/// パネルの内側のマージン
const PANEL_INNER_MARGIN: egui::Margin = egui::Margin {
    left: 12,
    right: 12,
    top: 0,
    bottom: 0,
};

/// パネルの内側のマージン
const TOP_PANEL_INNER_MARGIN: egui::Margin = egui::Margin {
    left: 10,
    right: 10,
    top: 4,
    bottom: 4,
};

/// ボトムパネルの内側のマージン
const BOTTOM_PANEL_INNER_MARGIN: egui::Margin = egui::Margin {
    left: 10,
    right: 10,
    top: 4,
    bottom: 6,
};

/// テーマ
#[derive(Clone, Copy, PartialEq, Deserialize, Serialize)]
pub(crate) enum Theme {
    Dark,
    Light,
}
impl Theme {
    /// テーマの視覚スタイルを取得
    /// * `return` - テーマの視覚スタイル
    pub(crate) fn visuals(&self) -> egui::Visuals {
        match self {
            Theme::Dark => egui::Visuals::dark(),
            Theme::Light => egui::Visuals::light(),
        }
    }
}
impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Theme::Dark => lang::current().setting_general.dark.to_string(),
            Theme::Light => lang::current().setting_general.light.to_string(),
        })
    }
}

/// フォントと SVG ローダーを追加
/// * `ctx` - コンテキスト
pub(crate) fn install(ctx: &egui::Context) {
    // フォントと SVG ローダーを追加
    font::install(ctx);
    svg::install(ctx);
}

/// ステータスアイコンの色
#[derive(Getters)]
#[getset(get = "pub")]
pub(crate) struct StatusColor {
    standby: egui::Color32,
    optimizing: egui::Color32,
    optimized: egui::Color32,
    unchanged: egui::Color32,
    skipped: egui::Color32,
    canceled: egui::Color32,
    error: egui::Color32,
}

impl StatusColor {
    /// 新しいステータスアイコンの色を作成
    /// * `config` - 設定
    /// * `return` - ステータスアイコンの色
    pub fn new(config: &config::Config) -> Self {
        Self {
            standby: assets::status_icon_color(&config, file::Status::Standby),
            optimizing: assets::status_icon_color(&config, file::Status::Optimizing),
            optimized: assets::status_icon_color(&config, file::Status::Optimized),
            unchanged: assets::status_icon_color(&config, file::Status::Unchanged),
            skipped: assets::status_icon_color(&config, file::Status::Skipped),
            canceled: assets::status_icon_color(&config, file::Status::Canceled),
            error: assets::status_icon_color(&config, file::Status::Error(String::new())),
        }
    }
}

/// パネルの背景色
/// * `config` - 設定
/// * `return` - パネルの背景色
pub(crate) fn panel_fill_color(config: &config::Config) -> egui::Color32 {
    if *config.theme() == Theme::Dark {
        DARK_MODE_PANEL_COLOR
    } else {
        LIGHT_MODE_PANEL_COLOR
    }
}

/// パネルのスタイルを設定
/// * `config` - 設定
/// * `inner_margin` - パネルの内側のマージン
/// * `return` - パネルのスタイル
pub(crate) fn panel_style(config: &config::Config, inner_margin: egui::Margin) -> egui::Frame {
    let panel_fill_color = panel_fill_color(config);
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
