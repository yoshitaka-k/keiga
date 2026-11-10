use crate::{config, ui};
use crate::ui::setting;
use crate::ui::assets::svg;

/// 並行処理数を表示
/// * `ui` - UI
/// * `config` - 設定
pub(crate) fn view(ui: &mut egui::Ui, config: &mut config::Config) {
    // ヘッダーパネルを表示
    setting::header_panel(ui, svg::CYCLE, "Concurrent", None);

    ui.add_space(ui::HEADER_BOTTOM_SPACING);

    ui.separator();

    ui.add_space(ui::SETTING_ADD_SPACING);

    egui::Frame::default().inner_margin(ui::PANEL_INNER_MARGIN).show(ui, |ui| {
        // 全ファイルの最適化数
        ui.horizontal(|ui| {
            ui::add_label(ui, "All files:", ui::CONCURRENT_LABEL_WIDTH);
            ui.scope(|ui| {
                ui.spacing_mut().slider_width = setting::remaining_slider_width(ui);
                ui.add(egui::Slider::new(config.optimization_num_mut(), config::OPTIMIZATION_NUM_MIN..=config::OPTIMIZATION_NUM_MAX));
            });
        });

        ui.add_space(ui::SETTING_ADD_SPACING);

        // PNG の最適化数
        ui.horizontal(|ui| {
            ui::add_label(ui, "PNG files:", ui::CONCURRENT_LABEL_WIDTH);
            ui.scope(|ui| {
                ui.spacing_mut().slider_width = setting::remaining_slider_width(ui);
                ui.add(egui::Slider::new(config.png_optimization_num_mut(), config::PNG_OPTIMIZATION_NUM_MIN..=config::PNG_OPTIMIZATION_NUM_MAX));
            });
        });

        // PNG の最適化数の注意書きを表示
        setting::warning_note(ui, &format!("PNG is included in All. ({} / {})", config.png_optimization_num(), config.optimization_num()));
    });
}
