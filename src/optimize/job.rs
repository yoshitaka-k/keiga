use std::sync::{mpsc, Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::collections::HashSet;
use crate::{config, error, file, optimize};
use crate::optimize::Optimizer as _;

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

/// public methods
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
                    Self::optimize(&mut file, &config, Arc::clone(&running), Arc::clone(&canceled))
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
                tx.send(file).unwrap_or_else(|e| {
                    log::warn!("Failed to send optimize result: {}", e);
                });

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
            self.remove_canceled_id(id).unwrap_or_else(|e| {
                log::error!("Failed to remove canceled ID: {}", e);
            });
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

/// private methods
impl OptimizeJob {
    /// 画像を最適化
    /// * `config` - 設定
    /// * `return` - 最適化の結果
    pub fn optimize(
        image: &mut file::Image,
        config: &config::Config,
        running: Arc<AtomicBool>,
        canceled: Arc<Mutex<HashSet<u64>>>
    ) -> error::Result<file::Status> {
        // 待機中・最適化中以外は再実行しない
        if !image.is_standby_or_optimizing() {
            return Ok(image.status().clone());
        }

        // 最適化開始時間を取得
        let start_time = std::time::Instant::now();

        // 最適化トークンを作成
        let token = optimize::OptimToken {
            id: *image.id(),
            running: Arc::clone(&running),
            canceled: Arc::clone(&canceled),
        };

        // 最適化中にする
        image.set_status(file::Status::Optimizing);

        // 出力ファイルのパスを作成
        let output_path = image.make_output_path(config);
        image.set_output_path(Some(output_path.clone()));

        // 最適化を実行
        let result = match image.extension() {
            // jpeg ファイルの最適化
            file::Extension::Jpeg => {
                let options = optimize::JpegOptions {
                    lossy: *config.jpeg_lossy(),
                    quality: *config.jpeg_quality(),
                };
                optimize::Jpeg::execute(&image.path(), &output_path, options, token)
            }

            // png ファイルの最適化
            file::Extension::Png => {
                let options = optimize::PngOptions {
                    lossy: *config.png_lossy(),
                    dithering: *config.png_dithering(),
                    options: config.png_options(),
                };
                optimize::Png::execute(&image.path(), &output_path, options, token)
            }

            // サポートしていないファイル形式
            _ => Err(error::KeigaError::unsupported_extension(image.path().clone())),
        };

        // 最適化結果を処理
        match result {
            Ok(status) => {
                match status {
                    file::Status::Optimized => {
                        // 最適化終了時間を取得
                        let end_time = std::time::Instant::now();
                        // 最適化時間を計算
                        let duration = end_time.duration_since(start_time).as_millis();
                        image.set_duration(duration as u64);

                        // 最適化後のファイルサイズと節約率を更新
                        let metadata = output_path.metadata().map_err(|e| {
                            error::KeigaError::file_error(e.to_string(), output_path.clone())
                        })?;
                        image.set_new_size(metadata.len());
                        image.set_saved_rate(file::calc_saved_rate(*image.size(), *image.new_size()));

                        // 最適化済みに設定
                        Ok(status)
                    }
                    file::Status::Error(e) => {
                        // 最適化エラーに設定
                        Ok(file::Status::Error(e.clone()))
                    }
                    _ => Ok(status)
                }
            }
            Err(e) => {
                // 最適化エラーに設定
                Ok(file::Status::Error(e.to_string()))
            }
        }
    }
}
