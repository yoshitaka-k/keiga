use std::sync::{Arc, mpsc};
use std::sync::atomic::{AtomicU64, Ordering};
use crate::file;

/// スキャントークン
/// * `ctx` - コンテキスト
/// * `result_tx` - 結果送信チャネル
/// * `generation` - ジョブ実行時の世代
/// * `current` - 今のジョブ実行中の世代（ScanJob と共通）
#[derive(Clone)]
pub(crate) struct ScanToken {
    pub ctx: egui::Context,
    pub result_tx: mpsc::Sender<(u64, file::ScanResult)>,
    pub generation: u64,
    pub current: Arc<AtomicU64>,
}

impl ScanToken {
    /// キャンセルされたかどうか
    /// * `return` - キャンセルされたかどうか
    pub fn is_canceled(&self) -> bool {
        self.current.load(Ordering::Relaxed) != self.generation
    }
}
