// リリースビルド時に Windows でコンソールウィンドウを隠す
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(clippy::all, rust_2018_idioms)]

use std::env;
use keiga::Rendar;
use keiga::App;

/// アプリケーション名
const APP_NAME: &str = "Keiga";
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// ウィンドウのサイズ
const WINDOW_WIDTH: f32 = 580.0;
const WINDOW_HEIGHT: f32 = 240.0;
const MAX_WINDOW_WIDTH: f32 = 800.0;
const MAX_WINDOW_HEIGHT: f32 = 1024.0;

fn main() -> eframe::Result {
    env_logger::init();

    // アプリケーションのインスタンスを作成
    let app = App::new();

    // アプリケーションアイコンを読み込む
    let icon = eframe::icon_data::from_png_bytes(include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/icon.png"
    )))
    .expect("failed to load app icon");

    // ウィンドウのオプション
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(format!("{} v{}", APP_NAME, VERSION))
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_min_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_max_inner_size([MAX_WINDOW_WIDTH, MAX_WINDOW_HEIGHT])
            .with_maximize_button(false)
            .with_resizable(true)
            .with_icon(icon),
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc| Ok(Box::new(Rendar::new(cc, app))))
    )
}
