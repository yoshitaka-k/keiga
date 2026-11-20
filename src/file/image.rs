use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use getset::{Getters, Setters};

use crate::{error, file};

/// Image の一意 ID を発行する
const COUNT_UP: u64 = 1;
static NEXT_ID: AtomicU64 = AtomicU64::new(COUNT_UP);

/// 画像ファイルを管理する構造体
#[derive(Clone, Getters, Setters)]
#[getset(get = "pub")]
pub(crate) struct Image {
    /// ファイルの一意な ID
    id: u64,

    /// ファイルのパス
    path: PathBuf,

    /// ファイルの相対パス
    relative_path: PathBuf,

    /// 出力したファイルのパス
    #[getset(set = "pub")]
    output_path: Option<PathBuf>,

    /// ファイルのメタデータ
    #[getset(set = "pub")]
    metadata: file::Metadata,

    /// フォルダ経由で追加されたファイルかどうか
    is_in_folder: bool,

    /// ファイルの最適化ステータス
    #[getset(set = "pub")]
    status: file::Status,

    /// ファイルのサイズ
    size: u64,

    /// ファイルの最適化後のサイズ
    #[getset(set = "pub")]
    new_size: u64,

    /// ファイルの最適化での節約率
    #[getset(set = "pub")]
    saved_rate: f32,

    /// 最適化時間（ミリ秒）
    #[getset(set = "pub")]
    duration: u64,
}

/// public methods
impl Image {
    /// 新しい Image を作成
    /// * `path` - ファイルのパス
    /// * `relative_path` - ドロップ基準からの相対パス
    /// * `return` - Image のインスタンス
    pub fn new(path: PathBuf, relative_path: PathBuf, is_in_folder: bool) -> Result<Self, error::KeigaError> {
        // ファイルサイズを取得
        let size = path.metadata()
            .map_err(|e| error::KeigaError::file_error(e.to_string(), path.clone()))?
            .len();

        Ok(Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            path,
            relative_path,
            output_path: None,
            is_in_folder,
            metadata: file::Metadata::Pending,
            status: file::Status::Standby,
            size,
            new_size: 0,
            saved_rate: 0.00f32,
            duration: 0,
        })
    }

    /// ファイル名を取得
    /// * `return` - ファイル名
    pub fn file_name(&self) -> std::borrow::Cow<'_, str> {
        self.path.file_name().map(|f| f.to_string_lossy()).unwrap_or_default()
    }

    /// ファイルの拡張子を取得
    /// * `return` - ファイルの拡張子
    pub fn extension(&self) -> file::Extension {
        match self.metadata {
            file::Metadata::Loading => file::Extension::None,
            file::Metadata::Pending => file::Extension::None,
            file::Metadata::Ready(extension) => extension,
            file::Metadata::Unavailable => file::Extension::None,
        }
    }

    /// ファイルが JPEG かどうか
    /// * `return` - ファイルが JPEG かどうか
    pub fn is_jpeg(&self) -> bool {
        matches!(self.extension(), file::Extension::Jpeg)
    }

    /// ファイルが PNG かどうか
    /// * `return` - ファイルが PNG かどうか
    pub fn is_png(&self) -> bool {
        matches!(self.extension(), file::Extension::Png)
    }

    /// ファイルがメタデータを読み込んでないかどうか
    /// * `return` - ファイルがメタデータを読み込んでないかどうか
    pub fn is_pending(&self) -> bool {
        matches!(self.metadata, file::Metadata::Pending)
    }

    /// ファイルがメタデータを読み込んでないかリサイズ中かどうか
    /// * `return` - ファイルがメタデータを読み込んでないかリサイズ中かどうか
    pub fn is_pending_or_loading(&self) -> bool {
        matches!(self.metadata, file::Metadata::Pending | file::Metadata::Loading)
    }

    /// ファイルが待機中かどうか
    /// * `return` - ファイルが待機中かどうか
    pub fn is_standby(&self) -> bool {
        matches!(self.status, file::Status::Standby)
    }

    /// ファイルが最適化中かどうか
    /// * `return` - ファイルが最適化中かどうか
    pub fn is_optimizing(&self) -> bool {
        matches!(self.status, file::Status::Optimizing)
    }

    /// ファイルが最適化済みかどうか
    /// * `return` - ファイルが最適化済みかどうか
    pub fn is_optimized(&self) -> bool {
        matches!(self.status, file::Status::Optimized)
    }

    /// ファイルが最適化不要かどうか
    /// * `return` - ファイルが最適化不要かどうか
    pub fn is_unchanged(&self) -> bool {
        matches!(self.status, file::Status::Unchanged)
    }

    /// ファイルがスキップされているかどうか
    /// * `return` - ファイルがスキップされているかどうか
    pub fn is_skipped(&self) -> bool {
        matches!(self.status, file::Status::Skipped)
    }

    /// ファイルがキャンセルされているかどうか
    /// * `return` - ファイルがキャンセルされているかどうか
    pub fn is_canceled(&self) -> bool {
        matches!(self.status, file::Status::Canceled)
    }

    /// ファイルがエラーかどうか
    /// * `return` - ファイルがエラーかどうか
    pub fn is_error(&self) -> bool {
        matches!(self.status, file::Status::Error(_))
    }

    /// ファイルが待機中・最適化中かどうか
    /// * `return` - ファイルが待機中・最適化中かどうか
    pub fn is_standby_or_optimizing(&self) -> bool {
        matches!(self.status, file::Status::Standby | file::Status::Optimizing)
    }

    /// 出力先を作成
    /// * `output_path` - 出力先
    /// * `return` - 出力先
    pub fn make_output_path(&self, output_path: &str) -> PathBuf {
        if output_path.is_empty() {
            self.path.clone()
        } else {
            PathBuf::from(output_path).join(self.relative_path.clone())
        }
    }

    /// 出力ファイルのパスを取得
    /// * `return` - 出力ファイルのパス
    pub fn reveal_path(&self) -> &PathBuf {
        match self.output_path {
            Some(ref path) if path.exists() => path,
            _ => &self.path,
        }
    }
}
