use crate::{event, file, ui, state, duration_format};
use crate::ui::{main, assets, StatusColor};
use crate::ui::assets::{constants, fonts::text_color, svg};

/// ファイル一覧を表示
/// * `ui` - UI
/// * `files` - ドロップされたファイル群
/// * `status_color` - ステータスアイコンの色
/// * `row_range` - 表示する行の範囲
/// * `list_row` - 表示する行の情報
/// * `return` - いずれかの行がクリックされたかどうか
pub(crate) fn view(
    ui: &mut egui::Ui,
    state: &state::State,
    pending_actions: &mut Vec<event::EventAction>,
    status_color: &StatusColor,
    row_height: f32,
) {
    // 中央パネルを表示
    egui::CentralPanel::default().show(ui, |ui| {
        // 行数を取得
        let total_length = state.files.len();

        // 利用可能な高さを設定
        let viewport_height = ui.available_height();
        // 行のストライド（行の高さ + 行の間隔）を設定
        let row_stride = row_height + ui.spacing().item_spacing.y;

        // スクロールエリアを作成
        let scroll_area = egui::ScrollArea::vertical()
            // アニメーションを無効化
            .animated(false)
            // コンテナが小さい時に縮小させない
            .auto_shrink([false; 2])
            // スクロールビューの高さを指定
            .max_height(viewport_height);

        // ファイル一覧を表示
        // リスト行をクリックしたら選択状態を保持
        let row_clicked = scroll_area.show_rows(
            ui, row_height, total_length,
            |ui, row_range|
        {
            // UIの幅と行間隔
            let width = ui.available_width();
            let row_spacing = ui.spacing().item_spacing.y;
            let start = row_range.start;

            // 選択された行がある場合はスクロールエリアの表示位置を調整
            if state.scroll_token.selected
                && let Some(index) = *state.files.selected_index()
            {
                // スクロール位置への矩形を計算・作成
                let area = ui.max_rect();
                let top = area.top() + (index as f32 - start as f32) * row_stride;
                let rect = egui::Rect::from_min_size(
                    egui::pos2(area.left(), top),
                    egui::vec2(area.width(), row_height),
                );

                // スクロールエリアの表示位置を調整
                // ui.scroll_to_rect(rect, None);
                ui.scroll_to_rect_animation(rect, None, egui::style::ScrollAnimation::none());
            }

            // 行がクリックされたかどうか
            let mut row_clicked = false;

            // 表示するファイルを取得
            let visible: Vec<_> = state.files.images()
                .get(row_range)
                .unwrap_or(&[])
                .to_vec();

            // ファイルが空の場合は表示
            if state.files.is_empty() {
                ui.horizontal(|ui| {
                    ui.add(main::icon_widget(svg::UPLOAD_FILE, constants::TOP_MENU_UPLOAD_FILE_ICON_SIZE, assets::list_empty_color(&state.config)));
                    ui.label(text_color("Files or Folders to Optimize Drag & Drop.", assets::list_empty_color(&state.config), None));
                });
            }

            // リストを表示
            for (offset, image_file) in visible.iter().enumerate() {
                let index = start + offset;

                // 最適化時間をフォーマット
                let duration = duration_format(*image_file.duration());

                // 行をクリックできるようにする
                let (row_rect, response) = ui.allocate_exact_size(
                    egui::vec2(width, row_height),
                    egui::Sense::click(),
                );

                // 交互に背景色
                if index.is_multiple_of(2) {
                    ui.painter().rect_filled(row_rect, ui::LIST_CORNER_RADIUS, main::alternate_background_color(&state.config));
                }

                // 選択されている場合は背景を表示
                if *state.files.selected_index() == Some(index) {
                    ui.painter().rect_filled(row_rect, ui::LIST_CORNER_RADIUS, main::selected_background_color(&state.config));
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
                        file::Status::Standby => {
                            main::list_row_content(ui, image_file, svg::CIRCLE, constants::CIRCLE_ICON_SIZE, *status_color.standby());
                        }
                        file::Status::Optimizing => {
                            main::list_row_content(ui, image_file, svg::AUTORENEW, constants::AUTORENEW_ICON_SIZE, *status_color.optimizing());
                        }
                        file::Status::Optimized => {
                            main::list_row_content(ui, image_file, svg::CHECK, constants::CHECK_ICON_SIZE, *status_color.optimized());

                            ui.separator();
                            ui.label(format!("{:+.2}%", image_file.saved_rate()));
                            ui.separator();
                            ui.label(duration.to_string());
                        }
                        file::Status::Unchanged => {
                            main::list_row_content(ui, image_file, svg::CHECK, constants::CHECK_ICON_SIZE, *status_color.unchanged());

                            ui.separator();
                            ui.label(text_color("No savings", *status_color.unchanged(), Some(ui::LIST_NOTE_SIZE)));
                        }
                        file::Status::Skipped => {
                            main::list_row_content(ui, image_file, svg::CHECK, constants::CHECK_ICON_SIZE, *status_color.skipped());

                            ui.separator();
                            ui.label(text_color("Skipped", *status_color.skipped(), Some(ui::LIST_NOTE_SIZE)));
                        }
                        file::Status::Canceled => {
                            main::list_row_content(ui, image_file, svg::CANCEL, constants::CANCEL_ICON_SIZE, *status_color.canceled());

                            ui.separator();
                            ui.label(text_color("Canceled", *status_color.canceled(), Some(ui::LIST_NOTE_SIZE)));
                        }
                        file::Status::Error(e) => {
                            main::list_row_content(ui, image_file, svg::ERROR, constants::ERROR_ICON_SIZE, *status_color.error());

                            ui.separator();
                            ui.label(text_color(e, *status_color.error(), Some(ui::LIST_NOTE_SIZE)));
                        }
                    }
                });

                // 最終行以外は行下端に区切り線
                if index + 1 < total_length {
                    ui.painter().hline(
                        row_rect.x_range(),
                        row_rect.bottom() + row_spacing * 0.5,
                        ui.visuals().widgets.noninteractive.bg_stroke,
                    );
                }

                // クリックアクションを処理予約
                if response.clicked() {
                    pending_actions.push(event::EventAction::Click { index: Some(index) });
                    row_clicked = true;
                }

                // ダブルクリックアクションを処理予約
                if response.double_clicked() {
                    pending_actions.push(event::EventAction::DoubleClick(image_file.reveal_path().clone()));
                }
            }

            row_clicked
        }).inner;

        // リスト行以外をクリックしたら選択解除
        if ui.input(|i| i.pointer.primary_clicked()) && !row_clicked {
            pending_actions.push(event::EventAction::Click { index: None });
        }
    });
}
