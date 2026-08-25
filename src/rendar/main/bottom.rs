use crate::duration_format;
use crate::file;
use crate::event::button;
use crate::optimize::OptimizeJob;
use crate::rendar::{ErrorToken, StatusColor};
use crate::rendar::assets::{self, constants, fonts::text_color, svg};
use crate::rendar::main;

/// 下部パネルを表示
/// * `ui` - UI
/// * `files` - ファイル群
/// * `optimize_job` - 最適化ジョブ
/// * `error_token` - エラーモーダルを表示するためのトークン
pub(crate) fn view(
    ui: &mut egui::Ui,
    status_color: &StatusColor,
    files: &mut file::OpenFiles,
    optimize_job: &mut OptimizeJob,
    error_token: &mut ErrorToken
) {
    // 未処理、最適化中、最適化済み、エラーのファイル数
    files.update_file_length();

    let standby_len = files.standby_len();
    let optimizing_len = files.optimizing_len();
    let optimized_len = files.optimized_len();
    let unchanged_len = files.unchanged_len();
    let skipped_len = files.skipped_len();
    let error_len = files.error_len();

    // デフォルトのスペースの幅を避けておく
    let spacing = ui.spacing().item_spacing.x;

    // アイコンの色
    let circle_color = *status_color.standby();
    let optimizing_color = *status_color.optimizing();
    let optimized_color = *status_color.optimized();
    let unchanged_color = *status_color.unchanged();
    let skipped_color = *status_color.skipped();
    let error_color = *status_color.error();

    // 完了アイコンの色
    // 最適化済みがあれば最適化済みの色
    // 最適化済みがなくて最適化不要があれば最適化不要の色
    // それ以外は最適化済みの色
    let completed_color = if optimized_len > 0 {
         optimized_color
    } else if unchanged_len > 0 {
        unchanged_color
    } else if skipped_len > 0 {
        skipped_color
    } else {
        optimized_color
    };

    ui.horizontal(|ui| {
        // 処理中アイコンを配置
        // ホバーテキストを設定
        let hover_text = format!(
            "Total optimization duration: {}",
            duration_format!(files.total_duration()),
        );

        // 優先度: 最適化中 > エラー > 最適化済み・最適化不要 > 待機
        let response = if optimizing_len > 0 {
            main::add_padded_icon(ui,
                main::spinner_widget(main::SPINNER_SIZE, optimizing_color),
            3.0)
        } else if error_len > 0 {
            main::add_padded_icon(ui,
                main::icon_widget(svg::ERROR, constants::ERROR_ICON_SIZE, error_color),
            1.0)
        } else if (optimized_len + unchanged_len + skipped_len) > 0 {
            main::add_padded_icon(ui,
                main::icon_widget(svg::CHECK, constants::CHECK_ICON_SIZE, completed_color),
            0.0)
        } else {
            main::add_padded_icon(ui,
                main::icon_widget(svg::CIRCLE, constants::CIRCLE_ICON_SIZE, circle_color),
            1.0)
        };

        response.on_hover_text(hover_text);

        ui.separator();

        // 未処理
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label(text_color(&format!("{}", standby_len), circle_color, None));
        ui.spacing_mut().item_spacing.x = spacing;
        ui.label(" standby,");

        // 最適化中
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label(text_color(&format!("{}", optimizing_len), optimizing_color, None));
        ui.spacing_mut().item_spacing.x = spacing;
        ui.label(" optimizing,");

        // 完了
        ui.scope(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label(text_color(&format!("{}", optimized_len + unchanged_len + skipped_len), completed_color, None));
            ui.spacing_mut().item_spacing.x = spacing;
            ui.label(" completed,");
        }).response.on_hover_ui(|ui| {
            // 最適化済み
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label(text_color(&format!("{}", optimized_len), optimized_color, None));
                ui.spacing_mut().item_spacing.x = spacing;
                ui.label(" optimized.");
            });
            // 最適化不要
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label(text_color(&format!("{}", unchanged_len), unchanged_color, None));
                ui.spacing_mut().item_spacing.x = spacing;
                ui.label(" no savings.");
            });
            // スキップ
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label(text_color(&format!("{}", skipped_len), skipped_color, None));
                ui.spacing_mut().item_spacing.x = spacing;
                ui.label(" skipped.");
            });
        });

        // エラー
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label(text_color(&format!("{}", error_len), error_color, None));
        ui.spacing_mut().item_spacing.x = spacing;
        ui.label(" error");

        ui.separator();

        // 平均保存率
        ui.label("Avg saved rate:");
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label(text_color(&format!("{:+.2}%", files.total_saved_rate()), optimized_color, None));
        ui.spacing_mut().item_spacing.x = spacing;

        // 右寄せ
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // ボタンの色を設定
            let button_color = assets::button_icon_color(ui);

            // クリアボタン
            let clear_button = egui::Image::new(svg::CLEAR_ALL).max_height(constants::BUTTON_CLEAR_ICON_SIZE).tint(button_color);
            if ui.button(clear_button).on_hover_text("Cancel and Clear").clicked() {
                if let Err(e) = button::cancel_and_clear(files, optimize_job) {
                    eprintln!("Error canceling and clearing: {}", e);
                    error_token.open = true;
                    error_token.value = Some(e);
                }
            }
        });
    });
}
