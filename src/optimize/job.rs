use std::sync::Arc;
use std::sync::{mpsc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::collections::HashSet;

use crate::{app, error};
use crate::file::{open_files, image_file};
use super::OptimizeStatus;

const RUNNING_COUNT_NUM: usize = 1;

/// 最適化ジョブを管理する構造体
pub struct OptimizeJob {
    ctx: egui::Context,

    /// 最適化結果を送信するチャネル
    result_tx: mpsc::Sender<image_file::ImageFile>,
    /// 最適化結果を受信するチャネル
    result_rx: mpsc::Receiver<image_file::ImageFile>,

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
    /// * `app` - アプリケーション
    /// * `files` - ファイルリスト
    pub fn run(&self, app: &app::App, files: &mut open_files::OpenFiles) {
        // 待機中のファイルがある場合は最適化実行中フラグを立てる
        if files.has_standby() {
            self.start_running();
        }

        // 最適化実行の数が最大値に達していない場合はループを続ける
        while self.is_running_count(app) {
            // PNG 最適化実行可能かどうか
            let allow_png = self.is_png_running_count(app);

            // 待機中のファイルを1件取得、なければループを終了
            let Some(file) = files.get_standby_file(allow_png) else { break };

            // ステータスを最適化中にする
            file.set_status(OptimizeStatus::Optimizing);
            let mut file = file.clone();

            // クローンしておく
            let app = app.clone();
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
                // 最適化を実行
                if let Err(e) = file.optimize(&app, Arc::clone(&running), Arc::clone(&canceled)) {
                    file.set_status(OptimizeStatus::Error(e.to_string()));
                }

                // 最適化結果を送信
                let _ = tx.send(file.clone());

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
    pub fn result(&self, files: &mut open_files::OpenFiles) {
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
    /// * `app` - アプリケーション
    /// * `return` - 最適化実行可能かどうか
    pub fn is_running_count(&self, app: &app::App) -> bool {
        self.running_count.load(Ordering::Relaxed) < *app.optimization_num() as usize
    }

    /// PNG 最適化実行可能かどうか
    /// * `app` - アプリケーション
    /// * `return` - PNG 最適化実行可能かどうか
    pub fn is_png_running_count(&self, app: &app::App) -> bool {
        self.png_running_count.load(Ordering::Relaxed) < *app.png_optimization_num() as usize
    }

    /// 最適化をキャンセル
    /// * `id` - キャンセルするファイルの ID
    pub fn add_canceled_id(&self, id: u64) -> error::Result<()> {
        self.canceled.lock().map_err(|_| error::KeigaError::LockPoisoned)?.insert(id);
        Ok(())
    }

    /// キャンセルに登録されているファイル ID を全てクリア
    /// * `return` - 成功かどうか
    pub fn clear_canceled(&self) -> error::Result<()> {
        self.canceled.lock().map_err(|_| error::KeigaError::LockPoisoned)?.clear();
        Ok(())
    }

    /// キャンセルに登録されているファイル ID を削除
    /// * `id` - 削除するファイル ID
    /// * `return` - 成功かどうか
    pub fn remove_canceled_id(&self, id: u64) -> error::Result<()> {
        self.canceled.lock().map_err(|_| error::KeigaError::LockPoisoned)?.remove(&id);
        Ok(())
    }
}
