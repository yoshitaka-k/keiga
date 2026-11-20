use std::sync::Arc;
use std::sync::mpsc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::panic::{catch_unwind, AssertUnwindSafe};
use crate::{error, file};

const RUNNING_COUNT_NUM: usize = 1;
const CONCURRENT_NUM: u8 = 10;

/// メタデータジョブ
pub(crate) struct MetaJob {
    /// コンテキスト
    ctx: egui::Context,

    /// ジョブ結果を送信するチャネル
    result_tx: mpsc::Sender<file::Image>,
    /// ジョブ結果を受信するチャネル
    result_rx: mpsc::Receiver<file::Image>,

    /// ジョブ実行中のカウント
    running_count: Arc<AtomicUsize>,
}

/// public methods
impl MetaJob {
    /// 新しいメタデータジョブを作成
    /// * `ctx` - コンテキスト
    /// * `return` - メタデータジョブ
    pub fn new(ctx: egui::Context) -> Self {
        let (result_tx, result_rx) = mpsc::channel();
        Self {
            ctx,
            result_tx,
            result_rx,
            running_count: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// ジョブを実行
    /// * `files` - 開いているファイル
    pub fn run(&self, files: &mut file::OpenFiles) {
        // ジョブ実行可能な間、ジョブを実行
        while self.is_runnable() {
            let Some(image) = files.get_pending_file() else { break };

            // 読み込み中に更新
            image.set_metadata(file::Metadata::Loading);

            // 必要なものをクローン
            let mut image = image.clone();
            let ctx = self.ctx.clone();
            let result_tx = self.result_tx.clone();

            // ジョブ実行中のカウントを増加
            self.running_count.fetch_add(RUNNING_COUNT_NUM, Ordering::Relaxed);
            let running_count = Arc::clone(&self.running_count);

            // ジョブ実行スレッドを作成
            std::thread::spawn(move || {
                // ジョブ中のパニックをキャッチ
                let result = catch_unwind(AssertUnwindSafe(|| {
                    // 画像の拡張子を読み込む
                    Self::read_extension(&image)
                }));

                // ジョブ結果からメタデータを設定
                let metadata = match result {
                    Ok(Ok(extension)) => file::Metadata::Ready(extension),
                    Ok(Err(_)) => file::Metadata::Unavailable,
                    Err(_) => file::Metadata::Unavailable,
                };

                // メタデータを設定
                image.set_metadata(metadata);

                // ジョブ結果を送信
                let _ = result_tx.send(image);

                // ジョブ実行中のカウントを減少
                running_count.fetch_sub(RUNNING_COUNT_NUM, Ordering::Relaxed);

                // 再描画を要求
                ctx.request_repaint();
            });
        }
    }

    /// ジョブ結果を受信
    /// * `files` - 開いているファイル
    pub fn result(&self, files: &mut file::OpenFiles) {
        // ジョブ結果を受信
        while let Ok(result) = self.result_rx.try_recv() {
            // ジョブ結果を反映
            files.apply_metadata_result(result);
        }
    }
}

/// private methods
impl MetaJob {
    /// ジョブ実行可能かどうか
    /// * `return` - ジョブ実行可能かどうか
    fn is_runnable(&self) -> bool {
        self.running_count.load(Ordering::Relaxed) < CONCURRENT_NUM as usize
    }

    /// 画像の拡張子を読み込む
    /// * `image` - 画像
    /// * `return` - 画像の拡張子
    fn read_extension(image: &file::Image) -> error::Result<file::Extension> {
        // 画像を読み込む
        let reader = image::ImageReader::open(image.path())
            .and_then(|r| r.with_guessed_format())
            .map_err(|e| error::KeigaError::file_error(e.to_string(), image.path().to_path_buf()))?;

        // 画像のフォーマットを取得
        let format = reader.format().ok_or_else(|| {
            error::KeigaError::file_error("Unknown image format".to_string(), image.path().to_path_buf())
        })?;

        // 読み取ったフォーマットを拡張子に対応づける
        let extension = match format {
            image::ImageFormat::Jpeg => file::Extension::Jpeg,
            image::ImageFormat::Png => file::Extension::Png,
            _ => file::Extension::None,
        };

        Ok(extension)
    }
}
