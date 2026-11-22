use crate::{event, file, ui, state, lang, filesize_format, duration_format};
use crate::ui::{main, assets, StatusColor};
use crate::ui::assets::{size, svg};

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
            // ファイルが空の場合は表示
            if state.files.is_empty() {
                ui.horizontal(|ui| {
                    ui.add(main::icon_widget(svg::UPLOAD_FILE, size::EMPTY_ICON_SIZE, assets::list_empty_color(&state.config)));
                    ui.label(ui::rich_text(lang::current().main_middle.upload_note, assets::list_empty_color(&state.config)));
                });
            }

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

            // リストを表示
            for (offset, image) in visible.iter().enumerate() {
                let index = start + offset;

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

                // ステータスに応じてノートを表示
                let note = match image.status() {
                    file::Status::Standby | file::Status::Optimizing => None,
                    file::Status::Optimized => Some(egui::RichText::new(duration_format(*image.duration()))),
                    file::Status::Unchanged => Some(ui::rich_text(lang::current().optimize_status.unchanged, *status_color.unchanged())),
                    file::Status::Skipped => Some(ui::rich_text(lang::current().optimize_status.skipped, *status_color.skipped())),
                    file::Status::Canceled => Some(ui::rich_text(lang::current().optimize_status.canceled, *status_color.canceled())),
                    file::Status::Error(e) => Some(ui::rich_text(e, *status_color.error())),
                };

                // コンテンツを表示
                ui.scope_builder(
                    egui::UiBuilder::new()
                        .max_rect(row_rect)
                        .layout(egui::Layout::left_to_right(egui::Align::Center)),
                |ui| {
                    // 折り返さないようにして、溢れた分は省略
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);

                    let spacing = ui.spacing().item_spacing.x;

                    // ステータスアイコンの表示
                    ui.add_space(ui::LIST_ICON_INDENT);
                    ui.add_space(ui::LIST_ICON_INDENT);
                    ui.spacing_mut().item_spacing.x = ui::LIST_ICON_INDENT;
                    main::status_icon(ui, image.status(), status_color, *status_color.optimized(), false)
                        .on_hover_text(image.status().to_string());
                    ui.spacing_mut().item_spacing.x = spacing;

                    ui.separator();

                    // ディレクトリか、ファイルかを表示
                    main::opentype_icon(ui, image);

                    // ファイル名とステータスノートを表示
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // ステータスノートを表示
                        if let Some(note) = note {
                            ui.add_space(ui::LIST_RIGHT_INDENT);
                            ui.label(note);
                            ui.separator();
                        }

                        // 節約率と新しいサイズを表示
                        if matches!(image.status(), file::Status::Optimized) {
                            ui.label(format!("{:+.2}% ({})",
                                *image.saved_rate(),
                                &filesize_format(*image.new_size()),
                            ));
                            ui.separator();
                        }

                        // ファイル名と元の画像のサイズを表示
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            // ファイル名の表示最小幅を設定
                            ui.set_min_width(ui.available_width() * ui::LIST_NAME_MIN_RATIO);

                            // ファイル名と元の画像のサイズを取得
                            let label = format!("{} ({})",
                                image.file_name(),
                                &filesize_format(*image.size())
                            );

                            ui.add(egui::Label::new(label).truncate());
                        });
                    });
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
                    pending_actions.push(event::EventAction::DoubleClick(image.reveal_path().clone()));
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
