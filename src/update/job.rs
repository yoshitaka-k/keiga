use std::sync::{mpsc, Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use crate::{REQUEST_URL, version_compare, update};

/// 更新ジョブを管理する構造体
pub(crate) struct UpdateJob {
    ctx: egui::Context,

    /// 更新結果を送信するチャネル
    result_tx: mpsc::Sender<update::UpdatedToken>,
    /// 更新結果を受信するチャネル
    result_rx: mpsc::Receiver<update::UpdatedToken>,

    /// ジョブ実行中のカウント
    running_bool: Arc<AtomicBool>,
}

impl UpdateJob {
    /// 新しい更新ジョブを作成する
    /// * `ctx` - コンテキスト
    /// * `return` - 更新ジョブ
    pub fn new(ctx: egui::Context) -> Self {
        let (result_tx, result_rx) = mpsc::channel();
        Self {
            ctx,
            result_tx,
            result_rx,
            running_bool: Arc::new(AtomicBool::new(false)),
        }
    }

    /// 更新を実行する
    /// * `updated_token` - 更新モーダルを表示するためのトークン
    pub fn run(&mut self, updated_token: &mut update::UpdatedToken) {
        // クローンしておく
        let tx = self.result_tx.clone();
        let ctx = self.ctx.clone();

        // リクエストURLを作成
        let request_url = REQUEST_URL.replace("{repository}", env!("CARGO_PKG_REPOSITORY"));
        let request = ehttp::Request::get(request_url);

        // ジョブが実行可能でない場合は処理を終了
        if self.is_running() { return; }

        // ジョブ実行中フラグを立てる
        self.running_bool.store(true, Ordering::Relaxed);
        let running_bool = Arc::clone(&self.running_bool);

        // UI 用にジョブ実行中フラグを立てておく
        updated_token.running = true;
        let mut token = updated_token.clone();

        ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
            match result {
                // リクエストが成功した場合
                Ok(response) => {
                    // アップデート情報をJSONからパース
                    let update_info: Result<update::UpdateInfo, _> = response.json();
                    match update_info {
                        // アップデート情報が取得できた場合
                        Ok(update_info) => {
                            match version_compare(update_info.tag_name(), env!("CARGO_PKG_VERSION")) {
                                Ok(true) => {
                                    token.open = true;
                                    token.check = Some(update::UpdateCheck::Available {
                                        version: update_info.tag_name().clone(),
                                        url: update_info.html_url.clone(),
                                    });
                                }
                                Ok(false) => {
                                    token.open = true;
                                    token.check = Some(update::UpdateCheck::Latest);
                                }
                                Err(error) => {
                                    println!("error: {:?}", error);
                                    token.open = true;
                                    token.check = Some(update::UpdateCheck::Failed);
                                }
                            }
                        }
                        // アップデート情報が取得できなかった場合
                        Err(error) => {
                            println!("error: {:?}", error);
                            token.open = true;
                            token.check = Some(update::UpdateCheck::Failed);
                        }
                    }
                }
                // リクエストが失敗した場合
                Err(error) => {
                    println!("error: {:?}", error);
                    token.open = true;
                    token.check = Some(update::UpdateCheck::Failed);
                }
            }

            // ジョブ実行中フラグを降ろす
            running_bool.store(false, Ordering::Relaxed);
            token.running = false;

            // 更新モーダルを表示するためのトークンを送信
            tx.send(token.clone()).unwrap_or_else(|e| {
                log::warn!("Failed to send update token: {}", e);
            });

            // 再描画を要求
            ctx.request_repaint();
        });
    }

    /// ジョブ実行中かどうか
    /// * `return` - ジョブ実行中かどうか
    pub fn is_running(&self) -> bool {
        self.running_bool.load(Ordering::Relaxed)
    }

    /// 更新結果を受信する
    /// * `updated_token` - 更新モーダルを表示するためのトークン
    pub fn result(&self, updated_token: &mut update::UpdatedToken) {
        while let Ok(result) = self.result_rx.try_recv() {
            *updated_token = result;
        }
    }
}
