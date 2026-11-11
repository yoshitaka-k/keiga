use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use getset::{Getters, Setters};

use crate::{config, error, file};

/// Image の一意な ID を発行するカウンタ
static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// 画像ファイルを管理する構造体
#[derive(Clone, Getters, Setters)]
pub(crate) struct Image {
    /// ファイルの一意な ID
    #[getset(get = "pub")]
    id: u64,

    /// ファイルのパス
    #[getset(get= "pub")]
    path: PathBuf,

    /// ドロップ基準からの相対パス
    #[getset(get = "pub")]
    relative_path: String,

    /// 出力したファイルのパス
    #[getset(get = "pub", set = "pub")]
    output_path: Option<PathBuf>,

    /// 相対パスかどうか
    #[getset(get = "pub")]
    is_relative_path: bool,

    /// ファイルの名前
    #[getset(get = "pub")]
    file_name: String,

    /// ファイルの拡張子
    #[getset(get = "pub")]
    extension: file::Extension,

    /// ファイルの最適化ステータス
    #[getset(get = "pub", set = "pub")]
    status: file::Status,

    /// ファイルのサイズ
    #[getset(get = "pub")]
    size: u64,

    /// ファイルの最適化後のサイズ
    #[getset(get = "pub", set = "pub")]
    new_size: u64,

    /// ファイルの最適化での節約率
    #[getset(get = "pub", set = "pub")]
    saved_rate: f32,

    /// 最適化時間（ミリ秒）
    #[getset(get = "pub", set = "pub")]
    duration: u64,
}

impl Image {
    /// 新しい Image を作成
    /// * `path` - ファイルのパス
    /// * `relative_path` - ドロップ基準からの相対パス
    /// * `return` - Image のインスタンス
    pub fn new(path: PathBuf, relative_path: String) -> Result<Self, error::KeigaError> {
        // ファイルの一意な ID を発行
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);

        // ファイル名を取得
        let file_name = if let Some(name) = path.file_name() {
            name.to_string_lossy().to_string()
        } else {
            return Err(error::KeigaError::file_error("File name not found".to_string(), path.clone()));
        };

        // 画像を読み込む
        let reader = image::ImageReader::open(&path)
            .and_then(|r| r.with_guessed_format())
            .map_err(|e| error::KeigaError::file_error(e.to_string(), path.to_path_buf()))?;

        // 画像のフォーマットを取得
        let format = reader.format().ok_or_else(|| {
            error::KeigaError::file_error("Unknown image format".to_string(), path.to_path_buf())
        })?;

        // 読み取ったフォーマットを拡張子に対応づける
        let extension = match format {
            image::ImageFormat::Jpeg => file::Extension::Jpeg,
            image::ImageFormat::Png => file::Extension::Png,
            _ => file::Extension::None,
        };

        // ファイルサイズを取得
        let size = path.metadata()
            .map_err(|e| error::KeigaError::file_error(e.to_string(), path.clone()))?
            .len();

        // 相対パスかどうかを判断
        let is_relative_path = relative_path != file_name;

        Ok(Self {
            id,
            path,
            relative_path,
            output_path: None,
            is_relative_path,
            file_name,
            extension,
            status: file::Status::Standby,
            size,
            new_size: 0,
            saved_rate: 0.00f32,
            duration: 0,
        })
    }

    /// ファイルが JPEG かどうか
    /// * `return` - ファイルが JPEG かどうか
    pub fn is_jpeg(&self) -> bool {
        matches!(self.extension, file::Extension::Jpeg)
    }

    /// ファイルが PNG かどうか
    /// * `return` - ファイルが PNG かどうか
    pub fn is_png(&self) -> bool {
        matches!(self.extension, file::Extension::Png)
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

    /// 出力ファイルのパスを作成
    /// * `config` - 設定
    /// * `return` - 出力ファイルのパス
    pub fn make_output_path(&self, config: &config::Config) -> PathBuf {
        if config.output_path().is_empty() {
            self.path.clone()
        } else {
            PathBuf::from(config.output_path()).join(self.relative_path.clone())
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
