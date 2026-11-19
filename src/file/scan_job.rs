use std::fs;
use std::sync::{mpsc, Arc};
use std::sync::atomic::{AtomicU64, Ordering};
use std::path::{Path, PathBuf};
use crate::{config, file};

// ジョブの世代の初期値
const GENERATION_INIT: u64 = 0;
// ジョブの世代の加算数
const GENERATION_STEP: u64 = 1;

/// ファイルスキャンジョブ
pub(crate) struct ScanJob {
    /// コンテキスト
    ctx: egui::Context,

    /// ジョブ結果を送信するチャネル
    result_tx: mpsc::Sender<(u64, file::ScanResult)>,
    /// ジョブ結果を受信するチャネル
    result_rx: mpsc::Receiver<(u64, file::ScanResult)>,

    /// ジョブ実行世代
    generation: Arc<AtomicU64>,

    /// ジョブ実行中のカウント
    running_count: Arc<AtomicU64>,
}

impl ScanJob {
    /// 新しいファイルスキャンジョブを作成
    /// * `ctx` - コンテキスト
    /// * `return` - ファイルスキャンジョブ
    pub fn new(ctx: egui::Context) -> Self {
        let (result_tx, result_rx) = mpsc::channel();
        Self {
            ctx,
            result_tx,
            result_rx,
            generation: Arc::new(AtomicU64::new(GENERATION_INIT)),
            running_count: Arc::new(AtomicU64::new(0)),
        }
    }

    /// ジョブを実行
    /// * `paths` - ファイルパス
    pub fn run(&self, paths: Vec<PathBuf>) {
        if paths.is_empty() { return; }

        // スキャントークンを作成
        let token = file::ScanToken {
            ctx: self.ctx.clone(),
            result_tx: self.result_tx.clone(),
            generation: self.generation.load(Ordering::Relaxed),
            current: Arc::clone(&self.generation),
        };

        // ジョブ実行中のカウントを増やす
        self.running_count.fetch_add(1, Ordering::Relaxed);
        let running_count = Arc::clone(&self.running_count);

        // スキャンを実行
        std::thread::spawn(move || {
            // ファイルを検索
            for path in paths {
                let base_dir = path.parent().unwrap_or(&path);
                find_file(&token, &path, base_dir)
            }

            // ジョブ実行中のカウントを減らす
            running_count.fetch_sub(1, Ordering::Relaxed);

            // 再描画を要求
            token.ctx.request_repaint();
        });
    }

    /// ジョブ結果を受信
    pub fn result(&self, config: &config::Config, files: &mut file::OpenFiles) {
        // ジョブ結果を受信
        while let Ok((generation, result)) = self.result_rx.try_recv() {
            if generation != self.generation.load(Ordering::Relaxed) { continue; }

            // ジョブ結果を反映
            files.scan_result(config, result);
        }
    }

    /// ジョブをキャンセル
    pub fn stop_running(&self) {
        // ジョブ実行世代を更新
        self.generation.fetch_add(GENERATION_STEP, Ordering::Relaxed);
    }

    /// ジョブ実行中かどうかを取得
    /// * `return` - ジョブ実行中のカウント
    pub fn is_running(&self) -> bool {
        self.running_count.load(Ordering::Relaxed) > 0
    }
}

/// エラーを追加
/// * `token` - スキャントークン
/// * `path` - ファイルのパス
/// * `base_dir` - ファイルの親（相対パスの基準）
/// * `error` - エラー
fn error_file(
    token: &file::ScanToken,
    path: &Path,
    base_dir: &Path,
    error: String
) {
    let result = file::ScanResult::Failed {
        path: path.to_path_buf(),
        base_dir: base_dir.to_path_buf(),
        error,
    };

    let _ = token.result_tx.send((token.generation, result));

    // 再描画を要求
    token.ctx.request_repaint();
}

/// ファイルを検索
/// * `token` - スキャントークン
/// * `path` - ファイルのパス
/// * `base_dir` - ファイルの親（相対パスの基準）
fn find_file(
    token: &file::ScanToken,
    path: &Path,
    base_dir: &Path
) {
    // キャンセルされた場合は処理を中断
    if token.is_canceled() { return; }

    // ファイルの存在を確認
    // シンボリックリンクは無視する
    let metadata = match path.symlink_metadata() {
        Ok(metadata) => metadata,
        Err(e) => return error_file(token, path, base_dir, e.to_string()),
    };

    if metadata.is_file() {
        if file::is_allowed_extension(path) {
            let result = file::ScanResult::Found {
                path: path.to_path_buf(),
                base_dir: base_dir.to_path_buf()
            };

            let _ = token.result_tx.send((token.generation, result));

            // 再描画を要求
            token.ctx.request_repaint();
        }
    } else if metadata.is_dir() {
        let entries = match fs::read_dir(path) {
            Ok(entries) => entries,
            Err(e) => return error_file(token, path, base_dir, e.to_string()),
        };

        // ディレクトリ内のファイルを検索
        for entry in entries {
            // キャンセルされた場合は処理を中断
            if token.is_canceled() { return; }

            match entry {
                Ok(entry) => find_file(token, &entry.path(), base_dir),
                Err(e) => error_file(token, path, base_dir, e.to_string()),
            }
        }
    }
}

