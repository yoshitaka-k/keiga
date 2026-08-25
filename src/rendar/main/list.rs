use std::path::PathBuf;
use egui::Sense;

use crate::{file, filesize_format, duration_format};
use crate::optimize::OptimizeStatus;
use crate::event::{click, key_up};
use crate::optimize::OptimizeJob;
use crate::rendar::{self, ListRowToken, ErrorToken};
use crate::rendar::assets::{self, constants, fonts::text_color, svg, icon_color};
use crate::rendar::main;

/// ファイル一覧のアクション
enum FileListAction {
    Hover { id: u64 },
    Click { id: u64 },
    DoubleClick { path: PathBuf },
    Backspace { id: u64 },
    Space { path: PathBuf },
}

/// show_rows 用の高さ
/// * `ui` - UI
/// * `return` - 行高
pub(crate) fn row_height(ui: &egui::Ui) -> f32 {
    ui.text_style_height(&egui::TextStyle::Body).max(constants::CHECK_ICON_SIZE) + main::SEPARATOR_HEIGHT
}

/// アイコンセルを表示
/// * `ui` - UI
/// * `pad` - パディング
/// * `widget` - アイコン
/// * `return` - アイコンセルのレスポンス
fn add_icon_cell(ui: &mut egui::Ui, widget: impl egui::Widget, is_standby: bool) -> egui::Response {
    // 間隔を保存
    let spacing = ui.spacing().item_spacing.x;
    let pad = if is_standby { main::LIST_ICON_STANDBY_LEFT_PADDING } else { 0.0 };

    ui.scope(|ui| {
        ui.add_space(main::LIST_ICON_CELL_LEFT_PADDING + pad);
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
fn add_opentype_icon(ui: &mut egui::Ui, path: &file::ImageFile, color: egui::Color32) -> egui::Response {
    if *path.is_relative_path() {
        ui.add(rendar::icon_widget(svg::FOLDER, constants::FOLDER_ICON_SIZE, color)).on_hover_text(path.relative_path().to_string())
    } else {
        ui.add(rendar::icon_widget(svg::PHOTO, constants::PHOTO_ICON_SIZE, color))
    }
}

/// ファイル一覧を表示
/// * `ui` - UI
/// * `files` - ドロップされたファイル群
/// * `row_range` - 表示する行の範囲
/// * `list_row` - 表示する行の情報
/// * `return` - いずれかの行がクリックされたかどうか
pub(crate) fn view(
    ui: &mut egui::Ui,
    files: &mut file::OpenFiles,
    optimize_job: &mut OptimizeJob,
    list_row: ListRowToken,
    error_token: &mut ErrorToken,
) -> bool {
    // UIの幅と行間隔
    let width = ui.available_width();
    let row_spacing = ui.spacing().item_spacing.y;
    // let col_spacing = ui.spacing().item_spacing.x;

    // アクションを処理するためのベクタ
    let mut pending_action: Vec<FileListAction> = Vec::new();
    let mut row_clicked = false;

    // アイコンの色
    let icon_color = icon_color(ui);
    let circle_color = assets::circle_color(ui);
    let optimizing_color = assets::optimizing_color(ui);
    let optimized_color = assets::optimized_color(ui);
    let unchanged_color = assets::unchanged_color(ui);
    let skipped_color = assets::skipped_color(ui);
    let canceled_color = assets::canceled_color(ui);
    let error_color = assets::error_color(ui);

    // 削除キーが押されたら処理予約
    if ui.input(|input| input.key_released(egui::Key::Backspace)) {
        if let Some(image_file) = files.selected_image_file() {
            pending_action.push(FileListAction::Backspace {
                id: *image_file.id(),
            });
        }
    }

    // スペースキーが押されたら処理予約
    if ui.input(|input| input.key_released(egui::Key::Space)) {
        if let Some(image_file) = files.selected_image_file() {
            pending_action.push(FileListAction::Space {
                path: image_file.reveal_path().clone(),
            });
        }
    }

    // リスト表示の準備
    let total = files.paths().len();
    let selected_id = *files.selected_id();
    let start = list_row.range.start;

    // 表示するファイルを取得
    let visible: Vec<_> = files.paths()
        .get(list_row.range)
        .unwrap_or(&[])
        .to_vec();

    // リストを表示
    for (offset, path) in visible.iter().enumerate() {
        let index = start + offset;

        // 表示するファイルサイズを計算
        let size = filesize_format!(*path.size());
        let new_size = filesize_format!(*path.new_size());

        // 最適化時間をフォーマット
        let duration = duration_format!(*path.duration());

        // 高さがズレると赤くチラつくので予めサイズ確保
        // 行のクリックイベントを受け取るために Sense::click() を指定
        let (row_rect, response) = ui.allocate_exact_size(
            egui::vec2(width, list_row.height),
            Sense::click(),
        );

        // 交互に背景色
        if index % 2 == 0 {
            ui.painter().rect_filled(row_rect, main::LIST_CORNER_RADIUS, main::alternate_background_color(ui));
        }

        // 選択されている場合は背景を表示
        if selected_id == Some(*path.id()) {
            ui.painter().rect_filled(row_rect, main::LIST_CORNER_RADIUS, main::selected_background_color(ui));
        }

        // コンテンツを表示
        ui.scope_builder(
            egui::UiBuilder::new()
                .max_rect(row_rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        |ui| {
            // 最適化ステータスに応じて表示
            match path.status() {
                OptimizeStatus::Standby => {
                    add_icon_cell(ui,
                        rendar::icon_widget(svg::CIRCLE, constants::CIRCLE_ICON_SIZE, circle_color)
                    , true).on_hover_text(path.status().to_string());
                    add_opentype_icon(ui, path, icon_color);
                    ui.label(path.file_name());
                    ui.label(format!("({})", size));
                }
                OptimizeStatus::Optimizing => {
                    add_icon_cell(ui,
                        rendar::icon_widget(svg::AUTORENEW, constants::AUTORENEW_ICON_SIZE, optimizing_color)
                    , false).on_hover_text(path.status().to_string());
                    add_opentype_icon(ui, path, icon_color);
                    ui.label(path.file_name());
                    ui.label(format!("({})", size));
                }
                OptimizeStatus::Optimized => {
                    add_icon_cell(ui,
                        rendar::icon_widget(svg::CHECK, constants::CHECK_ICON_SIZE, optimized_color)
                    , false).on_hover_text(path.status().to_string());
                    add_opentype_icon(ui, path, icon_color);
                    ui.label(path.file_name());
                    ui.label(format!("({} -> {})", size, new_size));
                    ui.separator();
                    ui.label(format!("{:+.2}%", path.percent()));
                    ui.separator();
                    ui.label(format!("{}", duration));
                }
                OptimizeStatus::Unchanged => {
                    add_icon_cell(ui,
                        rendar::icon_widget(svg::CHECK, constants::CHECK_ICON_SIZE, unchanged_color)
                    , false).on_hover_text(path.status().to_string());
                    add_opentype_icon(ui, path, icon_color);
                    ui.label(path.file_name());
                    ui.label(format!("({})", size));
                    ui.separator();
                    ui.label(text_color("No savings", unchanged_color, Some(main::LIST_NOTE_SIZE)));
                }
                OptimizeStatus::Skipped => {
                    add_icon_cell(ui,
                        rendar::icon_widget(svg::CHECK, constants::CHECK_ICON_SIZE, skipped_color)
                    , false).on_hover_text(path.status().to_string());
                    add_opentype_icon(ui, path, icon_color);
                    ui.label(path.file_name());
                    ui.label(format!("({})", size));
                    ui.separator();
                    ui.label(text_color("Skipped", skipped_color, Some(main::LIST_NOTE_SIZE)));
                }
                OptimizeStatus::Canceled => {
                    add_icon_cell(ui,
                        rendar::icon_widget(svg::CANCEL, constants::CANCEL_ICON_SIZE, canceled_color)
                    , false).on_hover_text(path.status().to_string());
                    add_opentype_icon(ui, path, icon_color);
                    ui.label(path.file_name());
                    ui.label(format!("({})", size));
                    ui.separator();
                    ui.label(text_color("Canceled", canceled_color, Some(main::LIST_NOTE_SIZE)));
                }
                OptimizeStatus::Error(e) => {
                    add_icon_cell(ui,
                        rendar::icon_widget(svg::ERROR, constants::ERROR_ICON_SIZE, error_color)
                    , false).on_hover_text(path.status().to_string());
                    add_opentype_icon(ui, path, icon_color);
                    ui.label(path.file_name());
                    ui.label(format!("({})", size));
                    ui.separator();
                    ui.label(text_color(e, error_color, Some(main::LIST_NOTE_SIZE)));
                }
            }
        });

        // 最終行以外は行下端に区切り線
        if index + 1 < total {
            ui.painter().hline(
                row_rect.x_range(),
                row_rect.bottom() + row_spacing * 0.5,
                ui.visuals().widgets.noninteractive.bg_stroke,
            );
        }

        // ホバーアクションを処理予約
        if response.hovered() {
            pending_action.push(FileListAction::Hover { id: *path.id() });
        }

        // クリックアクションを処理予約
        if response.clicked() {
            pending_action.push(FileListAction::Click { id: *path.id() });
            row_clicked = true;
        }

        // ダブルクリックアクションを処理予約
        if response.double_clicked() {
            pending_action.push(FileListAction::DoubleClick {
                path: path.reveal_path().clone(),
            });
        }
    }

    // クリックアクションを処理
    for action in pending_action {
        match action {
            FileListAction::Hover { id: _id } => {
                // ホバー
            }
            FileListAction::Click { id } => {
                files.set_selected_id(Some(id));
            }
            FileListAction::DoubleClick { path } => {
                click::double_click(&path);
            }
            FileListAction::Backspace { id: _id } => {
                if let Err(e) = key_up::backspace(files, optimize_job) {
                    eprintln!("Error canceling file: {}", e);
                    error_token.open = true;
                    error_token.value = Some(e);
                }
            }
            FileListAction::Space { path } => {
                if let Err(e) = key_up::space(&path) {
                    eprintln!("Error revealing file: {}", e);
                    error_token.open = true;
                    error_token.value = Some(e);
                }
            }
        }
    }

    row_clicked
}
