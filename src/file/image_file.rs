use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering, AtomicBool};
use std::sync::Mutex;
use std::collections::HashSet;
use getset::{Getters, Setters};

use crate::app::App;
use crate::optimize::{Jpeg, Png, OptimToken, OptimizeStatus};
use crate::file::extension;

/// ImageFile の一意な ID を発行するカウンタ
static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// 画像ファイルを管理する構造体
#[derive(Clone, PartialEq, Getters, Setters)]
pub struct ImageFile {
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
    #[getset(get = "pub")]
    output_path: Option<PathBuf>,

    /// 相対パスかどうか
    #[getset(get = "pub")]
    is_relative_path: bool,

    /// ファイルの名前
    #[getset(get = "pub")]
    file_name: String,

    /// ファイルの拡張子
    #[getset(get = "pub")]
    extension: extension::Extension,

    /// ファイルの最適化ステータス
    #[getset(get = "pub", set = "pub")]
    status: OptimizeStatus,

    /// ファイルのサイズ
    #[getset(get = "pub")]
    size: u64,

    /// ファイルの最適化後のサイズ
    #[getset(get = "pub")]
    new_size: u64,

    /// ファイルの最適化での節約率
    #[getset(get = "pub")]
    percent: f32,

    /// 最適化時間（ミリ秒）
    #[getset(get = "pub", set = "pub")]
    duration: u64,
}

impl ImageFile {
    /// 新しい ImageFile を作成
    /// * `path` - ファイルのパス
    /// * `relative_path` - ドロップ基準からの相対パス
    /// * `return` - ImageFile のインスタンス
    pub fn new(path: PathBuf, relative_path: String) -> Result<Self, Box<dyn std::error::Error>> {
        // ファイルの一意な ID を発行
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);

        // ファイル名を取得
        let file_name = if let Some(name) = path.file_name() {
            name.to_string_lossy().to_string()
        } else {
            return Err(format!("{} \n\nFile name not found", path.display()).into());
        };

        // ファイル拡張子を取得
        let extension = if let Some(ext) = path.extension() {
            extension::Extension::from_str(ext)
        } else {
            return Err(format!("{} \n\nFile extension not found", path.display()).into());
        };

        // ファイルサイズを取得
        let size = path.metadata()
            .map_err(|e| format!("{} \n\n{}", path.display(), e))?
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
            status: OptimizeStatus::Standby,
            size,
            new_size: 0,
            percent: 0.00f32,
            duration: 0,
        })
    }

    /// ファイルが PNG かどうか
    /// * `return` - ファイルが PNG かどうか
    pub fn is_png(&self) -> bool {
        matches!(self.extension, extension::Extension::Png)
    }

    /// 出力ファイルのパスを作成
    /// * `app` - アプリケーションの設定
    /// * `return` - 出力ファイルのパス
    pub fn make_output_path(&self, app: &App) -> PathBuf {
        if app.output_path().is_empty() {
            self.path.clone()
        } else {
            PathBuf::from(app.output_path()).join(self.relative_path.clone())
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

    /// ファイルサイズの更新
    /// * `output_path` - 出力ファイルのパス
    fn update_file_size(&mut self, output_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        // 最適化後のファイル情報
        let metadata = output_path.metadata().map_err(|e| format!("{} \n\n{}", output_path.display(), e))?;
        self.new_size = metadata.len();

        if self.size > 0 {
            // 最適化後のファイズによってパーセントを計算
            if self.size >= self.new_size {
                let percent = (self.size - self.new_size) as f32 / self.size as f32 * 100.0;
                self.percent = percent as f32 * -1.0;
            } else {
                let percent = (self.new_size - self.size) as f32 / self.size as f32 * 100.0;
                self.percent = percent as f32 * 1.0;
            }

            // 小数点第2位までの精度にする
            let res = (self.percent * 100.0).ceil() / 100.0;
            self.percent = res as f32;
        } else {
            self.percent = 0.00f32;
        }

        Ok(())
    }

    /// 画像を最適化
    /// * `app` - アプリケーションの設定
    /// * `return` - 最適化の結果
    pub fn optimize(
        &mut self,
        app: &App,
        running: Arc<AtomicBool>,
        canceled: Arc<Mutex<HashSet<u64>>>
    ) -> Result<OptimizeStatus, Box<dyn std::error::Error>> {
        // 完了済み・最適化不要・キャンセル済み・エラー済みは再実行しない
        if matches!(self.status,
            OptimizeStatus::Optimized | OptimizeStatus::Unchanged | OptimizeStatus::Skipped | OptimizeStatus::Canceled | OptimizeStatus::Error(_)
        ) {
            return Ok(self.status.clone());
        }

        // 最適化開始時間を取得
        let start_time = std::time::Instant::now();

        // 最適化トークンを作成
        let token = OptimToken {
            id: self.id,
            running: Arc::clone(&running),
            canceled: Arc::clone(&canceled),
        };

        // 最適化を中止したかどうかを確認
        if token.is_canceled()? {
            self.status = OptimizeStatus::Canceled;
            return Ok(OptimizeStatus::Canceled);
        }

        // 最適化中にする
        self.status = OptimizeStatus::Optimizing;

        // 出力ファイルのパスを作成
        let output_path = self.make_output_path(app);
        self.output_path = Some(output_path.clone());

        // 最適化を実行
        let result = match self.extension {
            // jpeg ファイルの最適化
            extension::Extension::Jpeg => {
                let quality = *app.jpeg_quality();
                Jpeg::optimize(&self.path, &output_path, quality, token)
            }

            // png ファイルの最適化
            extension::Extension::Png => {
                let options = app.png_options();
                Png::optimize(&self.path, &output_path, options, token)
            }

            // サポートしていないファイル形式
            _ => Err(format!("{} \n\nUnsupported extension", self.path.display()).into()),
        };

        // 最適化結果を処理
        match result {
            Ok(status) => {
                match status {
                    OptimizeStatus::Standby => {
                        Ok(self.status.clone())
                    }
                    OptimizeStatus::Optimizing => {
                        Ok(self.status.clone())
                    }
                    OptimizeStatus::Optimized => {
                        // 最適化終了時間を取得
                        let end_time = std::time::Instant::now();
                        // 最適化時間を計算
                        let duration = end_time.duration_since(start_time).as_millis();
                        self.duration = duration as u64;

                        // ファイルサイズを更新
                        self.update_file_size(&output_path)?;

                        // 最適化済みに設定
                        self.status = OptimizeStatus::Optimized;
                        Ok(OptimizeStatus::Optimized)
                    }
                    OptimizeStatus::Unchanged => {
                        // 最適化不要に設定
                        self.status = OptimizeStatus::Unchanged;
                        Ok(OptimizeStatus::Unchanged)
                    }
                    OptimizeStatus::Canceled => {
                        // 最適化中止に設定
                        self.status = OptimizeStatus::Canceled;
                        Ok(OptimizeStatus::Canceled)
                    }
                    OptimizeStatus::Skipped => {
                        // 最適化スキップに設定
                        self.status = OptimizeStatus::Skipped;
                        Ok(OptimizeStatus::Skipped)
                    }
                    OptimizeStatus::Error(e) => {
                        // 最適化エラーに設定
                        self.status = OptimizeStatus::Error(e.clone());
                        Err(e.to_string().clone().into())
                    }
                }
            }
            Err(e) => {
                // 最適化エラーに設定
                self.status = OptimizeStatus::Error(e.to_string());
                Err(e)
            }
        }
    }
}
