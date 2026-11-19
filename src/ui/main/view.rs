use crate::{event, ui, state};
use crate::ui::{setting, modal};
use crate::ui::assets::size;
use crate::ui::main::{top, middle, bottom};

/// メインウィンドウを表示
/// * `ui` - UI
/// * `state` - 状態
/// * `pending_actions` - 処理予約されたアクション
pub(crate) fn view(
    ui: &mut egui::Ui,
    state: &mut state::State,
    pending_actions: &mut Vec<event::EventAction>,
) {
    // 状態カラーを作成
    let status_color = ui::StatusColor::new(&state.config);

    // リスト行の高さと行数を取得
    let row_height = ui.text_style_height(&egui::TextStyle::Body)
        .max(size::STATUS_ICON_SIZE) + ui::SEPARATOR_HEIGHT;

    // 上部ボタンを表示
    top::view(
        ui,
        &state.config,
        pending_actions,
    );

    // 状態とかボタンを表示するタスクバーを表示
    bottom::view(
        ui,
        state,
        pending_actions,
        &status_color,
    );

    // ファイル一覧を表示
    middle::view(
        ui,
        &state,
        pending_actions,
        &status_color,
        row_height,
    );

    // 設定ウィンドウを表示
    if state.setting_token.open {
        setting::view(
            ui.ctx(),
            state,
            pending_actions,
        );
    }

    // エラーモーダルを表示
    if state.error_token.open {
        modal::error(
            ui.ctx(),
            &mut state.error_token,
        );
    }
}
