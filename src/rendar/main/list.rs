use egui::Sense;

use crate::{file, duration_format};
use crate::optimize::OptimizeStatus;
use crate::rendar::{ListRowToken, StatusColor};
use crate::rendar::assets::{constants, fonts::text_color, svg};
use crate::rendar::main;

/// ファイル一覧を表示
/// * `ui` - UI
/// * `files` - ドロップされたファイル群
/// * `status_color` - ステータスアイコンの色
/// * `row_range` - 表示する行の範囲
/// * `list_row` - 表示する行の情報
/// * `return` - いずれかの行がクリックされたかどうか
pub(crate) fn view(
    ui: &mut egui::Ui,
    status_color: &StatusColor,
    files: &file::OpenFiles,
    list_row: ListRowToken,
    pending_actions: &mut Vec<main::EventAction>,
) -> bool {
    // UIの幅と行間隔
    let width = ui.available_width();
    let row_spacing = ui.spacing().item_spacing.y;

    // アクションを処理するためのベクタ
    let mut row_clicked = false;

    // 削除キーが押されたら処理予約
    if ui.input(|input| input.key_released(egui::Key::Backspace)) {
        if files.selected_id().is_some() {
            pending_actions.push(main::EventAction::Backspace);
        }
    }

    // スペースキーが押されたら処理予約
    if ui.input(|input| input.key_released(egui::Key::Space)) {
        if let Some(image_file) = files.selected_image_file() {
            pending_actions.push(main::EventAction::Space {
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
    for (offset, image_file) in visible.iter().enumerate() {
        let index = start + offset;

        // 最適化時間をフォーマット
        let duration = duration_format(*image_file.duration());

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
        if selected_id == Some(*image_file.id()) {
            ui.painter().rect_filled(row_rect, main::LIST_CORNER_RADIUS, main::selected_background_color(ui));
        }

        // コンテンツを表示
        ui.scope_builder(
            egui::UiBuilder::new()
                .max_rect(row_rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        |ui| {
            // 折り返さないようにして、溢れた分は省略
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);

            // 最適化ステータスに応じて表示
            match image_file.status() {
                OptimizeStatus::Standby => {
                    main::list_row_content(ui, image_file, svg::CIRCLE, constants::CIRCLE_ICON_SIZE, *status_color.standby());
                }
                OptimizeStatus::Optimizing => {
                    main::list_row_content(ui, image_file, svg::AUTORENEW, constants::AUTORENEW_ICON_SIZE, *status_color.optimizing());
                }
                OptimizeStatus::Optimized => {
                    main::list_row_content(ui, image_file, svg::CHECK, constants::CHECK_ICON_SIZE, *status_color.optimized());

                    ui.separator();
                    ui.label(format!("{:+.2}%", image_file.saved_rate()));
                    ui.separator();
                    ui.label(format!("{}", duration));
                }
                OptimizeStatus::Unchanged => {
                    main::list_row_content(ui, image_file, svg::CHECK, constants::CHECK_ICON_SIZE, *status_color.unchanged());

                    ui.separator();
                    ui.label(text_color("No savings", *status_color.unchanged(), Some(main::LIST_NOTE_SIZE)));
                }
                OptimizeStatus::Skipped => {
                    main::list_row_content(ui, image_file, svg::CHECK, constants::CHECK_ICON_SIZE, *status_color.skipped());

                    ui.separator();
                    ui.label(text_color("Skipped", *status_color.skipped(), Some(main::LIST_NOTE_SIZE)));
                }
                OptimizeStatus::Canceled => {
                    main::list_row_content(ui, image_file, svg::CANCEL, constants::CANCEL_ICON_SIZE, *status_color.canceled());

                    ui.separator();
                    ui.label(text_color("Canceled", *status_color.canceled(), Some(main::LIST_NOTE_SIZE)));
                }
                OptimizeStatus::Error(e) => {
                    main::list_row_content(ui, image_file, svg::ERROR, constants::ERROR_ICON_SIZE, *status_color.error());

                    ui.separator();
                    ui.label(text_color(e, *status_color.error(), Some(main::LIST_NOTE_SIZE)));
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

        // クリックアクションを処理予約
        if response.clicked() {
            pending_actions.push(main::EventAction::Click { id: *image_file.id() });
            row_clicked = true;
        }

        // ダブルクリックアクションを処理予約
        if response.double_clicked() {
            pending_actions.push(main::EventAction::DoubleClick {
                path: image_file.reveal_path().clone(),
            });
        }
    }

    row_clicked
}
