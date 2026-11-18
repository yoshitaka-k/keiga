use crate::{ui, state, lang, event, duration_format};
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
    // デフォルトのスペースの幅を避けておく
    let spacing = ui.spacing().item_spacing.x;

    // 完了アイコンの色
    let completed_color = completed_color(state, status_color);

    // パネルのスタイルを設定
    let bottom_panel_style = ui::panel_style(&state.config, ui::BOTTOM_PANEL_INNER_MARGIN);

    // パネルを表示
    egui::Panel::bottom("bottom_taskbar").frame(bottom_panel_style).show(ui, |ui| {
        ui.horizontal(|ui| {
            // 処理中アイコンを配置
            // 優先度: 最適化中 > エラー > 最適化済み・最適化不要 > 待機
            let response = if state.files.optimizing_len() > 0 {
                main::add_padded_icon(ui,
                    main::spinner_widget(ui::SPINNER_SIZE, *status_color.optimizing()),
                3.0)
            } else if state.files.error_len() > 0 {
                main::add_padded_icon(ui,
                    main::icon_widget(svg::ERROR, size::ERROR_ICON_SIZE, *status_color.error()),
                1.0)
            } else if (state.files.optimized_len() + state.files.unchanged_len() + state.files.skipped_len()) > 0 {
                main::add_padded_icon(ui,
                    main::icon_widget(svg::CHECK, size::CHECK_ICON_SIZE, completed_color),
                0.0)
            } else {
                main::add_padded_icon(ui,
                    main::icon_widget(svg::CIRCLE, size::CIRCLE_ICON_SIZE, *status_color.standby()),
                1.0)
            };

            // ホバーテキストを設定
            response.on_hover_ui(|ui| {
                ui.label(format!(
                    "{}: {}",
                    lang::current().main_bottom.average_duration,
                    duration_format(state.files.average_duration()),
                ));

                // JPEG と PNG の平均最適化時間を表示
                if state.files.has_jpeg() && state.files.has_png() {
                    ui.separator();

                    if state.files.has_jpeg() {
                        ui.label(format!(
                            "{}: {}",
                            lang::current().main_bottom.jpeg_average_duration,
                            duration_format(state.files.jpeg_average_duration()),
                        ));
                    }
                    if state.files.has_png() {
                        ui.label(format!(
                            "{}: {}",
                            lang::current().main_bottom.png_average_duration,
                            duration_format(state.files.png_average_duration()),
                        ));
                    }
                }
            });

            ui.separator();

            // 未処理
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label(format!("{}: ", lang::current().optimize_status.standby));
            ui.label(assets::text_color(&format!("{}", state.files.standby_len()), *status_color.standby(), None));
            ui.spacing_mut().item_spacing.x = spacing;
            ui.label(",");

            // 最適化中
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label(format!("{}: ", lang::current().optimize_status.optimizing));
            ui.label(assets::text_color(&format!("{}", state.files.optimizing_len()), *status_color.optimizing(), None));
            ui.spacing_mut().item_spacing.x = spacing;
            ui.label(",");

            // 完了
            ui.scope(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label(format!("{}: ", lang::current().optimize_status.completed));
                ui.label(assets::text_color(&format!("{}", state.files.optimized_len() + state.files.unchanged_len() + state.files.skipped_len()), completed_color, None));
                ui.spacing_mut().item_spacing.x = spacing;
                ui.label(",");
            }).response.on_hover_ui(|ui| {
                // 最適化済み
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label(format!("{}: ", lang::current().optimize_status.optimized));
                    ui.spacing_mut().item_spacing.x = spacing;
                    ui.label(assets::text_color(&format!("{}", state.files.optimized_len()), *status_color.optimized(), None));
                });
                // 最適化不要
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label(format!("{}: ", lang::current().optimize_status.unchanged));
                    ui.spacing_mut().item_spacing.x = spacing;
                    ui.label(assets::text_color(&format!("{}", state.files.unchanged_len()), *status_color.unchanged(), None));
                });
                // スキップ
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label(format!("{}: ", lang::current().optimize_status.skipped));
                    ui.spacing_mut().item_spacing.x = spacing;
                    ui.label(assets::text_color(&format!("{}", state.files.skipped_len()), *status_color.skipped(), None));
                });
            });

            // エラー
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label(format!("{}: ", lang::current().optimize_status.error));
            ui.spacing_mut().item_spacing.x = spacing;
            ui.label(assets::text_color(&format!("{}", state.files.error_len()), *status_color.error(), None));

            ui.separator();

            // 平均保存率
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label(format!("{}: ", lang::current().main_bottom.average_saved_rate));
            ui.label(assets::text_color(&format!("{:+.2}%", state.files.total_saved_rate()), *status_color.optimized(), None));
            ui.spacing_mut().item_spacing.x = spacing;

            // 右寄せ
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // ボタンの色を設定
                let button_color = assets::button_icon_color(&state.config);

                // クリアボタン
                let clear_button = egui::Image::new(svg::CLEAR_ALL).max_height(size::BUTTON_CLEAR_ICON_SIZE).tint(button_color);
                if ui.button(clear_button).on_hover_text(lang::current().main_bottom.cancel_and_clear).clicked() {
                    pending_actions.push(event::EventAction::Clear);
                }
            });
        });
    });
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
