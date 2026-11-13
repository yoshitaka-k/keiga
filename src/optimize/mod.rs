mod job;
mod guard;
pub(crate) mod executor;
pub(crate) mod options;
pub(crate) use job::OptimizeJob;
pub(crate) use executor::{Optimizer, OpenImageResult, Jpeg, Png};
pub(crate) use options::{JpegOptions, PngOptions};

/// 一時ファイルの拡張子
const TEMP_EXTENSION: &str = "keiga.temp";

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::HashSet;
use crate::error;

/// 最適化トークン
#[derive(Clone)]
pub struct OptimToken {
    pub id: u64,
    pub running: Arc<AtomicBool>,
    pub canceled: Arc<Mutex<HashSet<u64>>>,
}

impl OptimToken {
    /// 最適化が中止されたかどうかを返す
    /// * `return` - 最適化が中止されたかどうか
    pub fn is_canceled(&self) -> error::Result<bool> {
        Ok(!self.running.load(Ordering::Relaxed) || self.canceled.lock().map_err(|_| {
            error::KeigaError::lock_poisoned()
        })?.contains(&self.id))
    }
}
