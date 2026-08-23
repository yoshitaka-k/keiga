use crate::app::UpdateJob;
use crate::file;
use crate::optimize::{OptimizeJob, OptimizeStatus};
use crate::rendar::{SettingToken};

/// ファイルダイアログを開く
/// * `ui` - UI
/// * `open_dialog` - ファイルダイアログを開くフラグ
pub(crate) fn files_open(ui: &mut egui::Ui, open_dialog: &mut bool) {
    // ファイルダイアログを開くタイミングをずらす
    *open_dialog = true;

    // 再描画を要求
    ui.ctx().request_repaint();
}

/// 設定ダイアログを開く
/// * `ui` - UI
/// * `setting_token` - 設定ダイアログを開くためのトークン
pub(crate) fn setting_open(ui: &mut egui::Ui, setting_token: &mut SettingToken) {
    // 設定ダイアログを開く
    setting_token.open = true;

    // 設定ダイアログの表示位置を設定
    setting_token.pos = ui.ctx().input(|input| {
        input.viewport().outer_rect.map(|rect| rect.min)
    });

    // 再描画を要求
    ui.ctx().request_repaint();
}

/// 最適化を停止（キャンセル）してファイル一覧をクリアする
/// * `files` - ファイル一覧
/// * `optimize_job` - 最適化ジョブ
pub(crate) fn cancel_and_clear(files: &mut file::OpenFiles, optimize_job: &mut OptimizeJob) -> Result<(), Box<dyn std::error::Error>> {
    // 最適化を停止（全体キャンセル）
    optimize_job.stop_running();

    // キャンセル ID の集合をクリア
    optimize_job.clear_canceled()?;

    // エラーを初期化
    let mut error = None;

    // ファイルを1件ずつキャンセル
    for file in files.paths() {
        // 待機中か最適化中でない場合はスキップ
        if !matches!(file.status(), OptimizeStatus::Standby | OptimizeStatus::Optimizing) {
            continue;
        }
        // キャンセル ID を追加してキャンセル状態にする
        if let Err(e) = optimize_job.add_canceled_id(*file.id()) {
            error = Some(e);
            break;
        }
    }

    // ファイル一覧をクリア
    files.clear();

    // エラーがあれば返却
    if let Some(error) = error {
        return Err(error);
    }

    Ok(())
}

/// アップデートを確認する
/// * `update_job` - 更新ジョブ
/// * `updated_token` - 更新モーダルを表示するためのトークン
pub(crate) fn check_for_update(update_job: &mut UpdateJob) {
    update_job.run();
}
