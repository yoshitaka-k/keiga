pub(crate) mod executor;
pub(crate) mod options;
mod job;
mod guard;
mod extensions;
pub(crate) use executor::Optimizer;
pub(crate) use job::OptimizeJob;
pub(crate) use options::{JpegOptions, PngOptions};
pub(crate) use extensions::{Jpeg, Png};

/// 一時ファイルの拡張子
const TEMP_EXTENSION: &str = "keiga.temp";

use std::path::Path;
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

/// 画像を読み込む
/// * `path` - 読み込む画像のパス
/// * `return` - 読み込んだ画像
fn open_image(path: &Path) -> error::Result<image::DynamicImage> {
    image::ImageReader::open(path)
        .and_then(|reader| reader.with_guessed_format())
        .map_err(|e| {
            error::KeigaError::optimized_error(e.to_string(), path.to_path_buf())
        })?
        .decode()
        .map_err(|e| {
            error::KeigaError::optimized_error(e.to_string(), path.to_path_buf())
        })
}
