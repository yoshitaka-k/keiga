use crate::duration_format;
use crate::file;
use crate::event::button;
use crate::optimize::OptimizeJob;
use crate::rendar::ErrorToken;
use crate::rendar::assets;
use crate::rendar::assets::{constants, fonts::text_color, svg};
use crate::rendar::main;

/// 左右に余白を付けてステータスアイコンを配置
/// * `ui` - UI
/// * `pad` - 余白
/// * `widget` - ステータスアイコン
/// * `return` - ステータスアイコンのレスポンス
fn add_padded_icon(ui: &mut egui::Ui, pad: f32, widget: impl egui::Widget) -> egui::Response {
    let spacing = ui.spacing().item_spacing.x;

    ui.spacing_mut().item_spacing.x = 2.0;
    let response = ui.scope(|ui| {
        ui.add_space(pad);
        ui.add(widget);
        ui.add_space(pad);
    }).response;
    ui.spacing_mut().item_spacing.x = spacing;

    response
}

/// 下部ボタンを表示
/// * `ui` - UI
/// * `files` - ドロップされたファイル
pub(crate) fn view(
    ui: &mut egui::Ui,
    files: &mut file::OpenFiles,
    optimize_job: &mut OptimizeJob,
    error_token: &mut ErrorToken,
) {
    // 未処理、最適化中、最適化済み、エラーのファイル数
    files.update_file_length();

    let standby_len = files.standby_len();
    let optimizing_len = files.optimizing_len();
    let optimized_len = files.optimized_len();
    let unchanged_len = files.unchanged_len();
    let error_len = files.error_len();

    // デフォルトのスペースの幅を避けておく
    let spacing = ui.spacing().item_spacing.x;

    // 最適化中アイコンの色
    let optimizing_color = assets::optimizing_color(ui);
    // 最適化済みアイコンの色
    let optimized_color = assets::optimized_color(ui);
    // エラーアイコンの色
    let error_color = assets::error_color(ui);
    // 最適化不要アイコンの色
    let unchanged_color = assets::unchanged_color(ui);

    // 完了アイコンの色
    // 最適化済みがあれば最適化済みの色
    // 最適化済みがなくて最適化不要があれば最適化不要の色
    // それ以外は最適化済みの色
    let completed_color = if optimized_len > 0 {
         optimized_color
    } else if unchanged_len > 0 {
        unchanged_color
    } else {
        optimized_color
    };

    // 丸アイコンの色
    let circle_color = assets::circle_color(ui);

    ui.horizontal(|ui| {
        // 処理中アイコンを配置
        // ホバーテキストを設定
        let hover_text = format!(
            "Total optimization duration: {}",
            duration_format!(files.total_duration()),
        );

        // 優先度: 最適化中 > エラー > 最適化済み・最適化不要 > 待機
        let response = if optimizing_len > 0 {
            add_padded_icon(ui, 3.0, egui::Spinner::new().size(main::SPINNER_SIZE).color(optimizing_color))
        } else if error_len > 0 {
            add_padded_icon(ui, 1.0, egui::Image::new(svg::ERROR).max_height(constants::ERROR_ICON_SIZE).tint(error_color))
        } else if (optimized_len + unchanged_len) > 0 {
            add_padded_icon(ui, 0.0, egui::Image::new(svg::CHECK).max_height(constants::CHECK_ICON_SIZE).tint(completed_color))
        } else {
            add_padded_icon(ui, 1.0, egui::Image::new(svg::CIRCLE).max_height(constants::CIRCLE_ICON_SIZE).tint(circle_color))
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
            ui.label(text_color(&format!("{}", optimized_len + unchanged_len), completed_color, None));
            ui.spacing_mut().item_spacing.x = spacing;
            ui.label(" completed,");
        }).response.on_hover_ui(|ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label(text_color(&format!("{}", optimized_len), optimized_color, None));
                ui.spacing_mut().item_spacing.x = spacing;
                ui.label(" optimized.");
            });
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label(text_color(&format!("{}", unchanged_len), unchanged_color, None));
                ui.spacing_mut().item_spacing.x = spacing;
                ui.label(" no savings.");
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
