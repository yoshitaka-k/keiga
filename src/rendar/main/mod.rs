pub(crate) mod view;
pub(crate) mod top;
pub(crate) mod list;
pub(crate) mod bottom;

// セパレータの高さ
pub(crate) const SEPARATOR_HEIGHT: f32 = 4.0;

// スピナーのサイズ
pub(crate) const SPINNER_SIZE: f32 = 8.0;

// アイコンセルの左パディング
pub(crate) const LIST_ICON_CELL_LEFT_PADDING: f32 = 2.0;
pub(crate) const LIST_ICON_STANDBY_LEFT_PADDING: f32 = 1.0;
pub(crate) const BOTTOM_ICON_CELL_LEFT_PADDING: f32 = 4.0;

// リストのノートのサイズ
pub(crate) const LIST_NOTE_SIZE: f32 = 11.0;
// リストの角丸
pub(crate) const LIST_CORNER_RADIUS: f32 = 1.0;

use crate::{file, filesize_format};
use crate::rendar::assets::{constants, svg, icon_color};

/// ファイル一覧のアクション
pub(crate) enum EventAction {
    Click { id: u64 },
    DoubleClick { path: std::path::PathBuf },
    Backspace,
    Space { path: std::path::PathBuf },
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

/// 交互に表示する背景色を取得
/// * `ui` - UI
/// * `return` - 交互に表示する背景色
pub(crate) fn alternate_background_color(ui: &egui::Ui) -> egui::Color32 {
    if ui.ctx().global_style().visuals.dark_mode {
        egui::Color32::from_rgba_unmultiplied(255, 255, 255, 5)
    } else {
        egui::Color32::from_rgba_unmultiplied(0, 0, 0, 10)
    }
}

/// 選択されている背景色を取得
/// * `ui` - UI
/// * `return` - 選択されている背景色
pub(crate) fn selected_background_color(ui: &egui::Ui) -> egui::Color32 {
    if ui.ctx().global_style().visuals.dark_mode {
        egui::Color32::from_rgba_unmultiplied(20, 120, 130, 50)
    } else {
        egui::Color32::from_rgba_unmultiplied(20, 120, 130, 50)
    }
}

/// アイコンセルを表示
/// * `ui` - UI
/// * `pad` - パディング
/// * `widget` - アイコン
/// * `return` - アイコンセルのレスポンス
pub(crate) fn add_icon_cell(
    ui: &mut egui::Ui,
    widget: impl egui::Widget,
    is_standby: bool
) -> egui::Response {
    // 間隔を保存
    let spacing = ui.spacing().item_spacing.x;
    let pad = if is_standby { LIST_ICON_STANDBY_LEFT_PADDING } else { 0.0 };

    ui.scope(|ui| {
        ui.add_space(LIST_ICON_CELL_LEFT_PADDING + pad);
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.add(widget);
        ui.spacing_mut().item_spacing.x = spacing;
        ui.add_space(pad);

        ui.separator();
    }).response
}

/// ディレクトリか、ファイルかを表示
/// * `ui` - UI
/// * `pad` - パディング
/// * `path` - パス
/// * `return` - OpenTypeアイコンのレスポンス
pub(crate) fn add_opentype_icon(
    ui: &mut egui::Ui,
    path: &file::ImageFile,
) -> egui::Response {
    let icon_color = icon_color(ui);
    if *path.is_relative_path() {
        ui.add(icon_widget(svg::FOLDER, constants::FOLDER_ICON_SIZE, icon_color)).on_hover_text(path.relative_path().to_string())
    } else {
        ui.add(icon_widget(svg::PHOTO, constants::PHOTO_ICON_SIZE, icon_color))
    }
}

/// リスト行のコンテンツを表示
/// * `ui` - UI
/// * `image_file` - ファイル
/// * `icon` - アイコン
/// * `icon_size` - アイコンのサイズ
/// * `icon_color` - アイコンの色
pub(crate) fn list_row_content(
    ui: &mut egui::Ui,
    image_file: &file::ImageFile,
    icon: egui::ImageSource<'static>,
    icon_size: f32,
    icon_color: egui::Color32
) {
    // 表示するファイルサイズを計算
    let size = filesize_format(*image_file.size());
    let new_size = filesize_format(*image_file.new_size());

    // アイコンセルを表示
    add_icon_cell(ui,
        icon_widget(icon, icon_size, icon_color)
    , true).on_hover_text(image_file.status().to_string());

    // オープンタイプアイコンを表示
    add_opentype_icon(ui, image_file);

    // ファイル名を表示
    ui.label(image_file.file_name());

    // ファイルサイズを表示
    if image_file.is_optimized() {
        ui.label(format!("({} -> {})", size, new_size));
    } else {
        ui.label(format!("({})", size));
    }
}

/// 左右に余白を付けてステータスアイコンを配置
/// * `ui` - UI
/// * `pad` - 余白
/// * `widget` - ステータスアイコン
/// * `return` - ステータスアイコンのレスポンス
pub(crate) fn add_padded_icon(ui: &mut egui::Ui, widget: impl egui::Widget, pad: f32) -> egui::Response {
    let spacing = ui.spacing().item_spacing.x;

    ui.scope(|ui| {
        ui.add_space(BOTTOM_ICON_CELL_LEFT_PADDING + pad);
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.add(widget);
        ui.spacing_mut().item_spacing.x = spacing;
        ui.add_space(pad);
    }).response
}
