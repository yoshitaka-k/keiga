use crate::{file, error, update, state};
use crate::optimize::OptimizeJob;

/// 設定ダイアログを開く
/// * `setting_token` - 設定ダイアログを開くためのトークン
/// * `pos` - 設定ウィンドウの表示位置
pub(crate) fn setting_open(
    setting_token: &mut state::SettingToken,
    pos: Option<egui::Pos2>,
) {
    // タブを初期化
    setting_token.tab = state::SettingTab::General;

    // 設定ウィンドウの表示位置を設定
    setting_token.pos = pos;

    // 設定ウィンドウを開く
    setting_token.open = true;
}

/// 最適化を停止（キャンセル）してファイル一覧をクリアする
/// * `files` - ファイル一覧
/// * `optimize_job` - 最適化ジョブ
pub(crate) fn cancel_and_clear(
    files: &mut file::OpenFiles,
    optimize_job: &mut OptimizeJob
) -> error::Result<()> {
    // 最適化を停止（全体キャンセル）
    optimize_job.stop_running();

    // キャンセル ID の集合をクリア
    optimize_job.clear_canceled()?;

    // エラーを初期化
    let mut error = None;

    // ファイルを1件ずつキャンセル
    for file in files.images() {
        // 待機中か最適化中でない場合はスキップ
        if !file.is_standby_or_optimizing() {
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
pub(crate) fn check_for_update(update_job: &mut update::UpdateJob) {
    update_job.run();
}
