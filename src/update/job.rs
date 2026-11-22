use std::time::{Duration, Instant};
use std::sync::{mpsc, Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use crate::{APP_NAME, REQUEST_URL, update};

/// 更新チェック間隔（1時間）
const UPDATE_CHECK_INTERVAL: Duration = Duration::from_secs(60 * 60);

/// 更新ジョブを管理する構造体
pub(crate) struct UpdateJob {
    ctx: egui::Context,

    /// 更新結果を送信するチャネル（更新モーダルを表示するためのトークンとETag）
    result_tx: mpsc::Sender<(update::UpdateToken, Option<String>)>,
    /// 更新結果を受信するチャネル（更新モーダルを表示するためのトークンとETag）
    result_rx: mpsc::Receiver<(update::UpdateToken, Option<String>)>,

    /// ジョブ実行中のカウント
    running_bool: Arc<AtomicBool>,

    /// 前回の更新チェック時間
    last_check_at: Option<Instant>,

    /// 前回の更新結果
    last_result: update::UpdateToken,

    /// 前回のETag
    last_etag: Option<String>,
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
            last_check_at: None,
            last_result: update::UpdateToken::new(),
            last_etag: None,
        }
    }

    /// 更新を実行する
    /// * `update_token` - 更新モーダルを表示するためのトークン
    pub fn run(&mut self, update_token: &mut update::UpdateToken) {
        // ジョブが実行可能でない場合は処理を終了
        if self.is_running() { return; }

        // 前回の更新結果が存在する場合
        if self.can_use_cache() {
            Self::tx_send(&self.ctx, &self.result_tx, self.last_result.clone(), self.last_etag.clone());
            return;
        }

        // クローンしておく
        let ctx = self.ctx.clone();
        let tx = self.result_tx.clone();
        let last_token = self.last_result.clone();

        // ジョブ実行中フラグを立てる
        self.running_bool.store(true, Ordering::Relaxed);
        let running_bool = Arc::clone(&self.running_bool);

        // UI 用にジョブ実行中フラグを立てておく
        update_token.running = true;
        let mut token = update_token.clone();

        // 前回の更新チェック時間を更新
        self.last_check_at = Some(Instant::now());

        // リクエストURLを作成
        let request_url = REQUEST_URL.replace("{repository}", env!("CARGO_PKG_REPOSITORY"));
        let mut request = ehttp::Request::get(request_url);

        // リクエストヘッダを設定
        let user_agent = format!("{}/{}", APP_NAME, env!("CARGO_PKG_VERSION"));
        request.headers.insert("Accept", "application/vnd.github+json");
        request.headers.insert("Accept-Encoding", "identity");
        request.headers.insert("User-Agent", user_agent);

        // 前回のETagが存在する場合はリクエストヘッダに追加
        if let Some(ref etag) = self.last_etag {
            request.headers.insert("If-None-Match", etag.clone());
        }
        let mut etag = self.last_etag.clone();

        // リクエストを実行
        ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
            match result {
                // リクエストが成功した場合
                Ok(response) => {
                    // ETagを取得
                    etag = response.headers.get("ETag").map(|s| s.to_string());
                    // 更新モーダルを表示するためのトークンを更新
                    token.open = true;
                    // レスポンスを解釈
                    token.check = Some(Self::interpret_response(response, last_token.check));
                }
                // リクエストが失敗した場合
                Err(error) => {
                    log::error!("Failed to fetch update info: {:?}", error);
                    token.open = true;
                    token.check = Some(update::UpdateCheck::Failed);
                }
            }

            // ジョブ実行中フラグを降ろす
            running_bool.store(false, Ordering::Relaxed);
            token.running = false;

            // 更新モーダルを表示するためのトークンを送信
            Self::tx_send(&ctx, &tx, token, etag);
        });
    }

    /// 更新結果を受信する
    /// * `update_token` - 更新モーダルを表示するためのトークン
    pub fn result(&mut self, update_token: &mut update::UpdateToken) {
        while let Ok(result) = self.result_rx.try_recv() {
            // 前回の更新結果を更新
            self.last_result = result.0.clone();
            self.last_etag = result.1;

            // 更新モーダルを表示するためのトークンを更新
            *update_token = result.0;
        }
    }
}

/// private methods
impl UpdateJob {
    /// ジョブ実行中かどうか
    /// * `return` - ジョブ実行中かどうか
    fn is_running(&self) -> bool {
        self.running_bool.load(Ordering::Relaxed)
    }

    /// キャッシュを使用できるかどうか
    /// * `return` - キャッシュを使用できるかどうか
    fn can_use_cache(&self) -> bool {
        // 前回の更新結果が存在する場合
        let Some(ref check) = self.last_result.check else { return false; };
        // 前回の更新結果が失敗していた場合はキャッシュを使用できない
        if matches!(*check, update::UpdateCheck::Failed) { return false; }
        // 前回の更新チェック時間が存在する場合
        let Some(last_check_at) = self.last_check_at else { return false; };
        // 前回の更新チェック時間から1時間以内にチェックしていた場合は処理を終了
        if last_check_at.elapsed() >= UPDATE_CHECK_INTERVAL { return false; }
        return true;
    }

    /// 更新モーダルを表示するためのトークンを送信
    /// * `ctx` - コンテキスト
    /// * `tx` - 更新モーダルを表示するためのトークンを送信するチャネル
    /// * `token` - 更新モーダルを表示するためのトークン
    fn tx_send(
        ctx: &egui::Context,
        tx: &mpsc::Sender<(update::UpdateToken, Option<String>)>,
        token: update::UpdateToken,
        etag: Option<String>,
    ) {
        tx.send((token.clone(), etag.clone())).unwrap_or_else(|e| {
            log::warn!("Failed to send update token: {}", e);
        });

        // 再描画を要求
        ctx.request_repaint();
    }

    /// レスポンスを解釈する
    /// * `response` - レスポンス
    /// * `last_check` - 前回の更新結果
    /// * `return` - 更新結果
    fn interpret_response(
        response: ehttp::Response,
        last_check: Option<update::UpdateCheck>,
    ) -> update::UpdateCheck {
        // レスポンスステータスによって処理を分岐
        if response.status == 304 {
            let Some(last_check) = last_check else {
                log::error!("Last check is not found");
                return update::UpdateCheck::Failed;
            };
            return last_check;
        }

        // アップデート情報をJSONからパース
        let update_info: Result<update::UpdateInfo, _> = response.json();
        match update_info {
            // アップデート情報が取得できた場合
            Ok(update_info) => {
                match update::version_compare(update_info.tag_name(), env!("CARGO_PKG_VERSION")) {
                    Ok(true) => {
                        return update::UpdateCheck::Available {
                            version: update_info.tag_name().clone(),
                            url: update_info.html_url.clone(),
                        };
                    }
                    Ok(false) => {
                        return update::UpdateCheck::Latest;
                    }
                    Err(error) => {
                        log::error!("Failed to compare version: {:?}", error);
                        return update::UpdateCheck::Failed;
                    }
                }
            }
            // アップデート情報が取得できなかった場合
            Err(error) => {
                log::error!("Failed to parse update info: {:?}", error);
                return update::UpdateCheck::Failed;
            }
        }
    }
}
