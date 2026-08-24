pub(crate) mod options;
mod job;
mod jpeg;
mod png;
pub use jpeg::Jpeg;
pub use png::Png;
pub(crate) use job::OptimizeJob;

/// 一時ファイルの拡張子
pub const TEMP_EXTENSION: &str = "keiga.temp";

use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::path::PathBuf;

/// 最適化ステータス
#[derive(Clone, PartialEq)]
pub enum OptimizeStatus {
    /// 最適化未実行
    Standby,
    /// 最適化中
    Optimizing,
    /// 最適化完了
    Optimized,
    /// 最適化不要
    Unchanged,
    /// 最適化スキップ
    Skipped,
    /// 最適化キャンセル
    Canceled,
    /// 最適化エラー（メッセージ）
    Error(String),
}

impl OptimizeStatus {
    /// 最適化ステータスを文字列に変換
    /// * `return` - 最適化ステータスを文字列に変換
    pub fn to_string(&self) -> String {
        match self {
            OptimizeStatus::Standby => "Standby".to_string(),
            OptimizeStatus::Optimizing => "Optimizing".to_string(),
            OptimizeStatus::Optimized => "Optimized".to_string(),
            OptimizeStatus::Unchanged => "Unchanged".to_string(),
            OptimizeStatus::Skipped => "Skipped".to_string(),
            OptimizeStatus::Canceled => "Canceled".to_string(),
            OptimizeStatus::Error(message) => format!("Error: {}", message),
        }
    }
}

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
    pub fn is_canceled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(!self.running.load(Ordering::Relaxed) || self.canceled.lock().map_err(|e| format!("{}", e))?.contains(&self.id))
    }
}

/// 出力ファイルのパスを作成
/// * `output_path` - 出力パス
/// * `return` - 出力ファイルのパスが作成できたかどうか
pub(crate) fn create_output_path(output_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = output_path.parent() {
        if !parent.exists() {
            match std::fs::create_dir_all(parent) {
                Ok(_) => (),
                Err(e) => {
                    return Err(e.to_string().into());
                }
            }
        }
    } else {
        return Err("Output path parent not found".to_string().into());
    }

    Ok(())
}

/// 一時ファイルを元のファイルに上書き
/// * `from` - 一時ファイルのパス
/// * `to` - 元のファイルのパス
/// * `return` - 一時ファイルを元のファイルに上書きしたかどうか
pub(crate) fn replace_file(from: &PathBuf, to: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // Windows 以外の環境ではファイルを直接上書き
    #[cfg(not(target_os = "windows"))]
    {
        std::fs::rename(from, to)?;
    }

    // Windows 環境では MoveFileExW を使用してファイルを上書き
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::ffi::OsStrExt;
        use winapi::um::winbase::{MoveFileExW, MOVEFILE_WRITE_THROUGH, MOVEFILE_REPLACE_EXISTING};

        // Windows の API に渡すためにパスを UTF-16 に変換
        let from_win: Vec<u16> = from.as_os_str().encode_wide().chain(Some(0)).collect();
        let to_win: Vec<u16> = to.as_os_str().encode_wide().chain(Some(0)).collect();

        // MoveFileExW を呼び出してファイルを上書き
        let result = unsafe {
            MoveFileExW(from_win.as_ptr(), to_win.as_ptr(), MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH)
        };

        // エラーが発生した場合はエラーを返す
        if result == 0 {
            return Err(std::io::Error::last_os_error().into());
        }
    }
    Ok(())
}
