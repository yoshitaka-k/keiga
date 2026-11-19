mod view;
mod top;
mod middle;
mod bottom;

pub(crate) use view::view;

use crate::{config, file, ui};
use crate::ui::assets::{size, svg};

/// アイコンウィジェットを作成
/// * `icon` - アイコン
/// * `size` - アイコンのサイズ
/// * `color` - アイコンの色
/// * `return` - アイコンウィジェット
fn icon_widget(
    icon: egui::ImageSource<'static>,
    size: f32, color: egui::Color32
) -> impl egui::Widget {
    egui::Image::new(icon).max_height(size).tint(color)
}

/// スピナーウィジェットを作成
/// * `size` - スピナーのサイズ
/// * `color` - スピナーの色
/// * `return` - スピナーウィジェット
fn spinner_widget(
    size: f32,
    color: egui::Color32
) -> impl egui::Widget {
    egui::Spinner::new().size(size).color(color)
}

/// 交互に表示する背景色を取得
/// * `ui` - UI
/// * `return` - 交互に表示する背景色
fn alternate_background_color(config: &config::Config) -> egui::Color32 {
    if *config.theme() == ui::Theme::Dark {
        egui::Color32::from_rgba_unmultiplied(255, 255, 255, 5)
    } else {
        egui::Color32::from_rgba_unmultiplied(0, 0, 0, 10)
    }
}

/// 選択されている背景色を取得
/// * `ui` - UI
/// * `return` - 選択されている背景色
fn selected_background_color(_config: &config::Config) -> egui::Color32 {
    egui::Color32::from_rgba_unmultiplied(20, 120, 130, 50)
}

/// ステータスアイコンを描画
/// * `ui` - UI
/// * `status` - ステータス
/// * `status_color` - ステータスカラー
/// * `completed_color` - 完了色
/// * `return` - ステータスアイコン
fn status_icon(
    ui: &mut egui::Ui,
    status: &file::Status,
    status_color: &ui::StatusColor,
    completed_color: egui::Color32,
    is_spinner: bool,
) -> egui::Response {
    match status {
        file::Status::Standby => {
            ui.add(icon_widget(svg::CIRCLE, size::STATUS_ICON_SIZE, *status_color.standby()))
        }
        file::Status::Optimizing => {
            if is_spinner {
                ui.add(spinner_widget(size::STATUS_ICON_SIZE, *status_color.optimizing()))
            } else {
                ui.add(icon_widget(svg::AUTORENEW, size::STATUS_ICON_SIZE, *status_color.optimizing()))
            }
        }
        file::Status::Optimized => {
            ui.add(icon_widget(svg::CHECK, size::STATUS_ICON_SIZE, completed_color))
        }
        file::Status::Unchanged => {
            ui.add(icon_widget(svg::CHECK, size::STATUS_ICON_SIZE, *status_color.unchanged()))
        }
        file::Status::Skipped => {
            ui.add(icon_widget(svg::CHECK, size::STATUS_ICON_SIZE, *status_color.skipped()))
        }
        file::Status::Canceled => {
            ui.add(icon_widget(svg::CANCEL, size::STATUS_ICON_SIZE, *status_color.canceled()))
        }
        file::Status::Error(_) => {
            ui.add(icon_widget(svg::ERROR, size::STATUS_ICON_SIZE, *status_color.error()))
        }
    }
}

/// ディレクトリか、ファイルかを表示
/// * `ui` - UI
/// * `pad` - パディング
/// * `path` - パス
/// * `return` - OpenTypeアイコンのレスポンス
fn opentype_icon(
    ui: &mut egui::Ui,
    path: &file::Image,
) -> egui::Response {
    let icon_color = ui::icon_color(ui);
    let icon = if *path.is_in_folder() { svg::FOLDER } else { svg::PHOTO };
    let hover_text = path.relative_path().display().to_string();

    ui.add(icon_widget(icon, size::LIST_ICON_SIZE, icon_color))
        .on_hover_text(hover_text)
}
