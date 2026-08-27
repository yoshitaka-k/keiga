pub(crate) mod options;
mod job;
mod jpeg;
mod png;
pub use jpeg::Jpeg;
pub use png::Png;
pub(crate) use job::OptimizeJob;
pub(crate) use options::{
    OptimizeStatus,
    JpegOptions,
    PngOptions,
};

/// 一時ファイルの拡張子
pub const TEMP_EXTENSION: &str = "keiga.temp";

use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::path::PathBuf;
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
            error::KeigaError::LockPoisoned
        })?.contains(&self.id))
    }
}

/// 最適化を行うトレイト
pub trait Optimizer {
    type Options;

    /// ファイルをエンコードする
    /// * `path` - エンコードするファイルのパス
    /// * `options` - エンコードオプション
    /// * `return` - エンコードされたファイルのサイズとデータ
    fn encode(
        path: &PathBuf,
        options: Self::Options,
    ) -> error::Result<(usize, Vec<u8>)>;

    /// 最適化を行う
    /// * `path` - 最適化するファイルのパス
    /// * `output_path` - 出力ファイルのパス
    /// * `options` - 最適化オプション
    /// * `token` - 最適化トークン
    /// * `return` - 最適化の結果
    fn optimize(
        path: &PathBuf,
        output_path: &PathBuf,
        options: Self::Options,
        token: OptimToken
    ) -> error::Result<OptimizeStatus> {
        // 最適化中止された場合は処理を中断
        if token.is_canceled()? {
            return Ok(OptimizeStatus::Canceled);
        }

        // ファイルを種類に応じてエンコードする
        let (original_size, byte_data) = Self::encode(path, options)?;

        // 最適化中止された場合は処理を中断
        if token.is_canceled()? {
            return Ok(OptimizeStatus::Canceled);
        }

        // 最適化後のサイズが元のサイズより大きい場合は最適化しない
        let new_size = byte_data.len() as usize;
        if original_size <= new_size {
            // 出力ファイルが存在しない場合は作成
            if path != output_path && !output_path.exists() {
                // 出力ファイルのパスを作成
                create_output_path(&output_path)?;

                // ファイルをコピー
                std::fs::copy(&path, &output_path).map_err(|e| {
                    error::KeigaError::OptimizedError(e.to_string(), output_path.clone())
                })?;
            }
            return Ok(OptimizeStatus::Unchanged);
        }

        // 出力ファイルのパスを作成
        create_output_path(&output_path)?;

        // 一時ファイルを作成して最適化後のデータを保存
        let temp_path = output_path.with_added_extension(TEMP_EXTENSION);
        std::fs::write(&temp_path, &byte_data).map_err(|e| {
            error::KeigaError::OptimizedError(e.to_string(), temp_path.clone())
        })?;

        // 最適化中止された場合は処理を中断
        if token.is_canceled()? {
            // 一時ファイルを削除
            std::fs::remove_file(&temp_path).map_err(|e| {
                error::KeigaError::OptimizedError(e.to_string(), temp_path.clone())
            })?;
            return Ok(OptimizeStatus::Canceled);
        }

        // 一時ファイルを元のファイルに上書き
        replace_file(&temp_path, &output_path)?;

        Ok(OptimizeStatus::Optimized)

    }
}

/// 出力ファイルのパスを作成
/// * `output_path` - 出力パス
/// * `return` - 出力ファイルのパスが作成できたかどうか
pub(crate) fn create_output_path(output_path: &PathBuf) -> error::Result<()> {
    // 出力ファイルのパスの親ディレクトリを取得
    let Some(parent) = output_path.parent() else {
        return Err(error::KeigaError::OptimizedError("Output path parent not found".to_string(), output_path.clone()));
    };

    // 出力ファイルのパスの親ディレクトリを作成
    std::fs::create_dir_all(parent).map_err(|e| {
        error::KeigaError::OptimizedError(e.to_string(), output_path.clone())
    })?;

    Ok(())
}

/// 一時ファイルを元のファイルに上書き
/// * `from` - 一時ファイルのパス
/// * `to` - 元のファイルのパス
/// * `return` - 一時ファイルを元のファイルに上書きしたかどうか
pub(crate) fn replace_file(from: &PathBuf, to: &PathBuf) -> error::Result<()> {
    // Windows 以外の環境ではファイルを直接上書き
    #[cfg(not(target_os = "windows"))]
    {
        std::fs::rename(from, to).map_err(|e| {
            error::KeigaError::OptimizedError(e.to_string(), to.clone())
        })?;
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
            return Err(error::KeigaError::OptimizedError(std::io::Error::last_os_error().to_string(), to.clone()));
        }
    }
    Ok(())
}
