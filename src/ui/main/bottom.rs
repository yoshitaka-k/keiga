use crate::{ui, state, lang, event, duration_format, filesize_format};
use crate::ui::{main, assets, StatusColor};
use crate::ui::assets::{size, svg};

/// 下部パネルを表示
/// * `ui` - UI
/// * `state` - 状態
/// * `pending_actions` - 未処理のアクション
/// * `status_color` - ステータスアイコンの色
pub(crate) fn view(
    ui: &mut egui::Ui,
    state: &state::State,
    pending_actions: &mut Vec<event::EventAction>,
    status_color: &StatusColor,
) {
    // パネルのスタイルを設定
    let style = ui::panel_style(&state.config, ui::BOTTOM_PANEL_INNER_MARGIN);
    // ボタンの色を設定
    let button_color = assets::button_icon_color(&state.config);

    // パネルを表示
    egui::Panel::bottom("bottom_taskbar").frame(style).show(ui, |ui| {
        egui::Sides::new().shrink_left().truncate().show(ui,
            |ui| {
                // デフォルトのスペースの幅を避けておく
                let spacing = ui.spacing().item_spacing.x;

                // ジョブの状態を取得
                let job_status = state.files.job_status();

                // 完了アイコンの色
                let completed_color = completed_color(state, status_color);

                // 処理中アイコンを配置
                // 優先度: 最適化中 > エラー > 最適化済み・最適化不要 > 待機
                main::status_icon(ui, &job_status, status_color, completed_color, true)
                    .on_hover_ui(|ui|
                {
                    // 平均最適化時間を表示
                    labeled_value(
                        ui,
                        spacing,
                        Some(&format!("{}: ", lang::current().main_bottom.average_duration)),
                        &duration_format(state.files.total_average_duration()),
                        optimizing_color(state, status_color),
                    );

                    // JPEG と PNG の平均最適化時間を表示
                    if state.files.has_jpeg() && state.files.has_png() {
                        ui.separator();

                        if state.files.has_jpeg() {
                            labeled_value(
                                ui,
                                spacing,
                                Some(&format!("{}: ", lang::current().main_bottom.jpeg_average_duration)),
                                &duration_format(state.files.jpeg_average_duration()),
                                optimizing_color(state, status_color),
                            );
                        }
                        if state.files.has_png() {
                            labeled_value(
                                ui,
                                spacing,
                                Some(&format!("{}: ", lang::current().main_bottom.png_average_duration)),
                                &duration_format(state.files.png_average_duration()),
                                optimizing_color(state, status_color),
                            );
                        }
                    }
                });

                ui.separator();

                // 未処理
                labeled_value(
                    ui,
                    spacing,
                    Some(&format!("{}: ", lang::current().optimize_status.standby)),
                    &state.files.standby_len().to_string(),
                    *status_color.standby(),
                );
                ui.label(",");

                // 最適化中
                let optimizing_response = ui.scope(|ui| {
                    labeled_value(
                        ui,
                        spacing,
                        Some(&format!("{}: ", lang::current().optimize_status.optimizing)),
                        &state.files.optimizing_len().to_string(),
                        *status_color.optimizing(),
                    );
                    ui.label(",");
                }).response;

                // 最適化中の JPEG と PNG の数を表示
                if state.files.has_jpeg() && state.files.has_png() {
                    optimizing_response.on_hover_ui(|ui| {
                        if state.files.has_jpeg() {
                            labeled_value(
                                ui,
                                spacing,
                                Some(&format!("{}: ", lang::current().extensions.jpeg)),
                                &state.files.jpeg_optimizing_len().to_string(),
                                *status_color.optimizing(),
                            );
                        }
                        if state.files.has_png() {
                            labeled_value(
                                ui,
                                spacing,
                                Some(&format!("{}: ", lang::current().extensions.png)),
                                &state.files.png_optimizing_len().to_string(),
                                *status_color.optimizing(),
                            );
                        }
                    });
                }

                // 完了
                ui.scope(|ui| {
                    labeled_value(
                        ui,
                        spacing,
                        Some(&format!("{}: ", lang::current().optimize_status.completed)),
                        &state.files.completed_len().to_string(),
                        completed_color,
                    );
                    ui.label(",");
                }).response.on_hover_ui(|ui| {
                    // 最適化済み
                    labeled_value(
                        ui,
                        spacing,
                        Some(&format!("{}: ", lang::current().optimize_status.optimized)),
                        &state.files.optimized_len().to_string(),
                        *status_color.optimized(),
                    );
                    // 最適化不要
                    labeled_value(
                        ui,
                        spacing,
                        Some(&format!("{}: ", lang::current().optimize_status.unchanged)),
                        &state.files.unchanged_len().to_string(),
                        *status_color.unchanged(),
                    );
                    // スキップ
                    labeled_value(
                        ui,
                        spacing,
                        Some(&format!("{}: ", lang::current().optimize_status.skipped)),
                        &state.files.skipped_len().to_string(),
                        *status_color.skipped(),
                    );
                });

                // エラー
                labeled_value(
                    ui,
                    spacing,
                    Some(&format!("{}: ", lang::current().optimize_status.error)),
                    &state.files.error_len().to_string(),
                    *status_color.error(),
                );

                // 次のセパレータとの間を空ける
                ui.label("");

                ui.separator();

                // 平均保存率
                let rate_response = ui.scope(|ui| {
                    labeled_value(
                        ui,
                        spacing,
                        Some(&format!("{}: ", lang::current().main_bottom.average)),
                        &format!("{:+.2}%", state.files.total_saved_rate()),
                        *status_color.optimized(),
                    );
                }).response;

                // JPEG と PNG の平均節約率を表示
                if state.files.has_jpeg() && state.files.has_png() {
                    rate_response.on_hover_ui(|ui| {
                        if state.files.has_jpeg() {
                            labeled_value(
                                ui,
                                spacing,
                                Some(&format!("{}: ", lang::current().main_bottom.jpeg_average_saved_rate)),
                                &format!("{:+.2}%", state.files.jpeg_average_saved_rate()),
                                *status_color.optimized(),
                            );
                        }
                        if state.files.has_png() {
                            labeled_value(
                                ui,
                                spacing,
                                Some(&format!("{}: ", lang::current().main_bottom.png_average_saved_rate)),
                                &format!("{:+.2}%", state.files.png_average_saved_rate()),
                                *status_color.optimized(),
                            );
                        }
                    });
                }

                ui.label("/");

                // 総平均新サイズを表示
                let new_size_response = ui.scope(|ui| {
                    labeled_value(
                        ui,
                        spacing,
                        None,
                        &filesize_format(state.files.total_average_saved()),
                        *status_color.optimized(),
                    );
                }).response;

                // JPEG と PNG の平均新サイズを表示
                if state.files.has_jpeg() && state.files.has_png() {
                    new_size_response.on_hover_ui(|ui| {
                        if state.files.has_jpeg() {
                            labeled_value(
                                ui,
                                spacing,
                                Some(&format!("{}: ", lang::current().main_bottom.jpeg_average_saved)),
                                &filesize_format(state.files.jpeg_average_saved()),
                                *status_color.optimized(),
                            );
                        }
                        if state.files.has_png() {
                            labeled_value(
                                ui,
                                spacing,
                                Some(&format!("{}: ", lang::current().main_bottom.png_average_saved)),
                                &filesize_format(state.files.png_average_saved()),
                                *status_color.optimized(),
                            );
                        }
                    });
                }
            },
            |ui| {
                // クリアボタン
                let clear_button = egui::Image::new(svg::CLEAR_ALL).max_height(size::BUTTON_CLEAR_ICON_SIZE).tint(button_color);
                if ui.button(clear_button).on_hover_text(lang::current().main_bottom.cancel_and_clear).clicked() {
                    pending_actions.push(event::EventAction::Clear);
                }
            }
        );
    });
}

/// 最適化中アイコンの色
/// * `state` - 状態
/// * `status_color` - ステータスアイコンの色
/// * `return` - 最適化中アイコンの色
fn optimizing_color(
    state: &state::State,
    status_color: &ui::StatusColor,
) -> egui::Color32 {
    if state.files.optimizing_len() > 0 {
        *status_color.optimizing()
    } else if state.files.error_len() > 0 {
        *status_color.error()
    } else {
        *status_color.optimized()
    }
}

/// 完了アイコンの色
///
/// 最適化済みがあれば最適化済みの色
/// 最適化済みがなくて最適化不要があれば最適化不要の色
/// それ以外は最適化済みの色
///
/// * `state` - 状態
/// * `status_color` - ステータスアイコンの色
/// * `return` - 完了アイコンの色
fn completed_color(
    state: &state::State,
    status_color: &ui::StatusColor,
) -> egui::Color32 {
    if state.files.optimized_len() > 0 {
        *status_color.optimized()
    } else if state.files.unchanged_len() > 0 {
        *status_color.unchanged()
    } else if state.files.skipped_len() > 0 {
        *status_color.skipped()
    } else {
        *status_color.optimized()
    }
}

/// ラベル付き値を描画
/// * `ui` - UI
/// * `spacing` - スペース
/// * `label` - ラベル
/// * `value` - 値
/// * `color` - カラー
/// * `return` - ラベル付き値のレスポンス
fn labeled_value(
    ui: &mut egui::Ui,
    spacing: f32,
    label: Option<&str>,  // filesize は None
    value: &str,
    color: egui::Color32,
) -> egui::Response {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        if let Some(label) = label {
            ui.label(label);
        }
        ui::rich_label(ui, value, color);
        ui.spacing_mut().item_spacing.x = spacing;
    }).response
}
