use std::sync::Arc;
use std::sync::{mpsc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::collections::HashSet;

use crate::{config, error, file};

const RUNNING_COUNT_NUM: usize = 1;

/// 最適化ジョブを管理する構造体
pub struct OptimizeJob {
    ctx: egui::Context,

    /// 最適化結果を送信するチャネル
    result_tx: mpsc::Sender<file::Image>,
    /// 最適化結果を受信するチャネル
    result_rx: mpsc::Receiver<file::Image>,

    /// 最適化実行フラグ
    running: Arc<AtomicBool>,

    /// 最適化実行中のカウント
    running_count: Arc<AtomicUsize>,

    /// PNG 最適化実行中のカウント
    png_running_count: Arc<AtomicUsize>,

    /// キャンセルフラグ
    canceled: Arc<Mutex<HashSet<u64>>>,
}

impl OptimizeJob {
    /// 新しい最適化ジョブを作成
    /// * `ctx` - UI コンテキスト
    /// * `return` - 最適化ジョブ
    pub fn new(ctx: egui::Context) -> Self {
        let (result_tx, result_rx) = mpsc::channel();
        Self {
            ctx,
            result_tx,
            result_rx,
            running: Arc::new(AtomicBool::new(true)),
            running_count: Arc::new(AtomicUsize::new(0)),
            png_running_count: Arc::new(AtomicUsize::new(0)),
            canceled: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    /// 最適化を実行
    /// * `config` - 設定
    /// * `files` - ファイルリスト
    pub fn run(&self, config: &config::Config, files: &mut file::OpenFiles) {
        // 待機中のファイルがある場合は最適化実行中フラグを立てる
        if files.has_standby() {
            self.start_running();
        }

        // 最適化実行の数が最大値に達していない場合はループを続ける
        while self.is_running_count(config) {
            // PNG 最適化実行可能かどうか
            let allow_png = self.is_png_running_count(config);

            // 待機中のファイルを1件取得、なければループを終了
            let Some(file) = files.get_standby_file(allow_png) else { break };

            // ステータスを最適化中にする
            file.set_status(file::Status::Optimizing);

            // クローンして借用を解除しておく
            // files の可変借用を終わらせて、下の衝突チェックで files を読めるようにするため
            let mut file = file.clone();

            // 入力が違くて、出力先パスが同じファイルが処理中、処理済みの状態である場合はエラーを返す
            if files.has_output_conflict(file.path(), file.output_path().as_deref()) {
                files.set_status_error(*file.id(), "Output path conflicts");
                continue;
            }

            // クローンしておく
            let config = config.clone();
            let tx = self.result_tx.clone();
            let ctx = self.ctx.clone();
            let running = Arc::clone(&self.running);
            let canceled = Arc::clone(&self.canceled);

            // 最適化カウントを増やす
            self.running_count.fetch_add(RUNNING_COUNT_NUM, Ordering::Relaxed);
            let is_png = file.is_png();
            if is_png {
                self.png_running_count.fetch_add(RUNNING_COUNT_NUM, Ordering::Relaxed);
            }

            // カウントをクローンしておく
            let running_count = Arc::clone(&self.running_count);
            let png_running_count = Arc::clone(&self.png_running_count);

            // 最適化を実行するスレッドを作成
            std::thread::spawn(move || {
                let result = catch_unwind(AssertUnwindSafe(|| {
                    // 最適化を実行
                    file.optimize(&config, Arc::clone(&running), Arc::clone(&canceled))
                }));

                let status = match result {
                    Ok(Ok(status)) => status,
                    Ok(Err(e)) => file::Status::Error(e.to_string()),
                    Err(payload) => {
                        let message = payload.downcast_ref::<&str>().map(|s| s.to_string())
                            .or_else(|| payload.downcast_ref::<String>().cloned())
                            .unwrap_or_else(|| "Unknown error".to_string());
                        file::Status::Error(message)
                    }
                };

                file.set_status(status);

                // 最適化結果を送信
                let _ = tx.send(file);

                // この最適化のスレッドが終わってからカウントを減らす
                running_count.fetch_sub(RUNNING_COUNT_NUM, Ordering::Relaxed);

                // PNG 最適化の場合は PNG 最適化カウントを減らす
                if is_png {
                    png_running_count.fetch_sub(RUNNING_COUNT_NUM, Ordering::Relaxed);
                }

                // 再描画を要求
                ctx.request_repaint();
            });
        }
    }

    /// 最適化結果を反映
    /// * `files` - ファイルリスト
    pub fn result(&self, files: &mut file::OpenFiles) {
        // 最適化結果を受信
        while let Ok(result) = self.result_rx.try_recv() {
            // ファイル ID を控えておく
            let id = *result.id();

            // 最適化結果を反映
            files.apply_result(result);

            // ファイル ID を削除
            let _ = self.remove_canceled_id(id);
        }
    }

    /// 最適化を開始
    pub fn start_running(&self) {
        self.running.store(true, Ordering::Relaxed);
    }

    /// 最適化をキャンセル
    pub fn stop_running(&self) {
        self.running.store(false, Ordering::Relaxed);
    }

    /// 最適化実行可能かどうか
    /// * `config` - 設定
    /// * `return` - 最適化実行可能かどうか
    pub fn is_running_count(&self, config: &config::Config) -> bool {
        self.running_count.load(Ordering::Relaxed) < *config.optimization_num() as usize
    }

    /// PNG 最適化実行可能かどうか
    /// * `config` - 設定
    /// * `return` - PNG 最適化実行可能かどうか
    pub fn is_png_running_count(&self, config: &config::Config) -> bool {
        self.png_running_count.load(Ordering::Relaxed) < *config.png_optimization_num() as usize
    }

    /// 最適化をキャンセル
    /// * `id` - キャンセルするファイルの ID
    pub fn add_canceled_id(&self, id: u64) -> error::Result<()> {
        self.canceled.lock().map_err(|_| error::KeigaError::lock_poisoned())?.insert(id);
        Ok(())
    }

    /// キャンセルに登録されているファイル ID を全てクリア
    /// * `return` - 成功かどうか
    pub fn clear_canceled(&self) -> error::Result<()> {
        self.canceled.lock().map_err(|_| error::KeigaError::lock_poisoned())?.clear();
        Ok(())
    }

    /// キャンセルに登録されているファイル ID を削除
    /// * `id` - 削除するファイル ID
    /// * `return` - 成功かどうか
    pub fn remove_canceled_id(&self, id: u64) -> error::Result<()> {
        self.canceled.lock().map_err(|_| error::KeigaError::lock_poisoned())?.remove(&id);
        Ok(())
    }
}
