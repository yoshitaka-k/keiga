use std::fs;
use std::path::{Path, PathBuf};
use getset::{Getters, Setters};

use crate::{app, error, file};
use crate::optimize::OptimizeStatus;

/// 最適化済みのファイル数を管理する構造体
#[derive(Clone)]
struct OptimizedLen {
    jpeg: u32,
    png: u32,
}

impl OptimizedLen {
    pub fn new() -> Self {
        Self {
            jpeg: 0,
            png: 0,
        }
    }
    pub fn total(&self) -> u32 {
        self.jpeg + self.png
    }
}

/// 最適化時間を管理する構造体
#[derive(Clone)]
struct DurationInfo {
    jpeg: u64,
    png: u64,
}

impl DurationInfo {
    pub fn new() -> Self {
        Self {
            jpeg: 0,
            png: 0,
        }
    }
    pub fn total(&self) -> u64 {
        self.jpeg + self.png
    }
}

/// ファイル情報を管理する構造体
#[derive(Clone)]
struct FileInfo {
    standby_len: u32,
    optimizing_len: u32,
    optimized_len: OptimizedLen,
    unchanged_len: u32,
    skipped_len: u32,
    canceled_len: u32,
    error_len: u32,
    total_size: u64,
    total_new_size: u64,
    total_duration: DurationInfo,
    total_saved_rate: f32,
    average_duration: u64,
    jpeg_average_duration: u64,
    png_average_duration: u64,
}

impl FileInfo {
    pub fn new() -> Self {
        Self {
            standby_len: 0,
            optimizing_len: 0,
            optimized_len: OptimizedLen::new(),
            unchanged_len: 0,
            skipped_len: 0,
            canceled_len: 0,
            error_len: 0,
            total_size: 0,
            total_new_size: 0,
            total_duration: DurationInfo::new(),
            total_saved_rate: 0.00,
            average_duration: 0,
            jpeg_average_duration: 0,
            png_average_duration: 0,
        }
    }
}

/// ドロップされたファイルを管理する構造体
#[derive(Clone, Getters, Setters)]
pub struct OpenFiles {
    /// ファイル一覧
    #[getset(get = "pub")]
    paths: Vec<file::ImageFile>,

    /// 許可されたファイル拡張子
    #[getset(set = "pub")]
    extensions: Vec<String>,

    /// 選択されたファイルの ID
    #[getset(get = "pub", set = "pub")]
    selected_id: Option<u64>,

    /// ファイル情報
    #[getset(get = "pub")]
    file_info: FileInfo,
}

impl OpenFiles {
    /// 新しい OpenFiles を作成
    /// * `return` - OpenFiles のインスタンス
    pub fn new() -> Self {
        Self {
            paths: vec![],
            extensions: vec![],
            selected_id: None,
            file_info: FileInfo::new(),
        }
    }

    /// 待機中のファイルを1件取得
    /// * `return` - 待機中のファイル1件
    pub fn get_standby_file(&mut self, allow_png: bool) -> Option<&mut file::ImageFile> {
        self.paths.iter_mut().find(|f| {
            f.is_standby() && (allow_png || !f.is_png())
        })
    }

    /// 選択されたファイルのパスを取得
    /// * `return` - 選択されたファイルのパス
    pub fn selected_image_file(&self) -> Option<file::ImageFile> {
        if let Some(id) = self.selected_id() {
            self.paths.iter().find(|p| *p.id() == *id).map(|p| p.clone())
        } else {
            None
        }
    }

    /// キャンセルされたファイルのステータスを更新
    /// * `id` - キャンセルされたファイルの ID
    pub fn set_status_canceled(&mut self, id: u64) {
        if let Some(file) = self.paths.iter_mut().find(|f| *f.id() == id) {
            // 最適化済み・最適化不要・スキップ・エラーはキャンセルできない
            if !file.is_optimized() && !file.is_unchanged() && !file.is_skipped() && !file.is_error() {
                file.set_status(OptimizeStatus::Canceled);
            }
        }
    }

    /// ファイル情報を更新
    pub fn update_file_info(&mut self) {
        let mut standby_len = 0;
        let mut optimizing_len = 0;
        let mut optimized_len = OptimizedLen::new();
        let mut unchanged_len = 0;
        let mut canceled_len = 0;
        let mut skipped_len = 0;
        let mut error_len = 0;

        let mut total_size = 0;
        let mut total_new_size = 0;
        let mut total_duration = DurationInfo::new();

        // 各ファイルの情報を集計
        for file in &self.paths {
            match file.status() {
                OptimizeStatus::Standby => standby_len += 1,
                OptimizeStatus::Optimizing => optimizing_len += 1,
                OptimizeStatus::Optimized => {
                    if file.is_jpeg() {
                        optimized_len.jpeg += 1;
                        total_duration.jpeg += file.duration();
                    } else if file.is_png() {
                        optimized_len.png += 1;
                        total_duration.png += file.duration();
                    }

                    total_size += file.size();
                    total_new_size += file.new_size();
                }
                OptimizeStatus::Unchanged => unchanged_len += 1,
                OptimizeStatus::Skipped => skipped_len += 1,
                OptimizeStatus::Canceled => canceled_len += 1,
                OptimizeStatus::Error(_) => error_len += 1,
            }
        }

        // 節約率を計算
        let total_saved_rate = file::calc_saved_rate(total_size, total_new_size);

        // 平均最適化時間を計算
        let average_duration = if optimized_len.total() > 0 {
            total_duration.total() / optimized_len.total() as u64
        } else {
            0
        };

        let jpeg_average_duration = if optimized_len.jpeg > 0 {
            total_duration.jpeg / optimized_len.jpeg as u64
        } else {
            0
        };

        let png_average_duration = if optimized_len.png > 0 {
            total_duration.png / optimized_len.png as u64
        } else {
            0
        };

        // ファイル情報を更新
        self.file_info.standby_len = standby_len;
        self.file_info.optimizing_len = optimizing_len;
        self.file_info.optimized_len = optimized_len;
        self.file_info.unchanged_len = unchanged_len;
        self.file_info.skipped_len = skipped_len;
        self.file_info.canceled_len = canceled_len;
        self.file_info.error_len = error_len;

        self.file_info.total_size = total_size;
        self.file_info.total_new_size = total_new_size;
        self.file_info.total_duration = total_duration;
        self.file_info.total_saved_rate = total_saved_rate;
        self.file_info.average_duration = average_duration;
        self.file_info.jpeg_average_duration = jpeg_average_duration;
        self.file_info.png_average_duration = png_average_duration;
    }

    /// ファイルの数を取得
    /// * `return` - ファイルの数
    pub fn len(&self) -> usize {
        self.paths.len()
    }

    /// 未処理のファイルの数を取得
    /// * `return` - 未処理のファイルの数
    pub fn standby_len(&self) -> u32 {
        self.file_info.standby_len
    }

    /// 最適化中のファイルの数を取得
    /// * `return` - 最適化中のファイルの数
    pub fn optimizing_len(&self) -> u32 {
        self.file_info.optimizing_len
    }

    /// 最適化済みのファイルの数を取得
    /// * `return` - 最適化済みのファイルの数
    pub fn optimized_len(&self) -> u32 {
        self.file_info.optimized_len.total()
    }

    /// 最適化不要のファイルの数を取得
    /// * `return` - 最適化不要のファイルの数
    pub fn unchanged_len(&self) -> u32 {
        self.file_info.unchanged_len
    }

    /// スキップされたファイルの数を取得
    /// * `return` - スキップされたファイルの数
    pub fn skipped_len(&self) -> u32 {
        self.file_info.skipped_len
    }

    /// キャンセルされたファイルの数を取得
    /// * `return` - キャンセルされたファイルの数
    pub fn canceled_len(&self) -> u32 {
        self.file_info.canceled_len
    }

    /// エラーのファイルの数を取得
    /// * `return` - エラーのファイルの数
    pub fn error_len(&self) -> u32 {
        self.file_info.error_len
    }

    /// ファイルの総サイズを取得
    /// * `return` - ファイルの総サイズ
    pub fn total_size(&self) -> u64 {
        self.file_info.total_size
    }

    /// ファイルの総新サイズを取得
    /// * `return` - ファイルの総新サイズ
    pub fn total_new_size(&self) -> u64 {
        self.file_info.total_new_size
    }

    /// ファイルの総節約率を取得
    /// * `return` - ファイルの総節約率
    pub fn total_saved_rate(&self) -> f32 {
        self.file_info.total_saved_rate
    }

    /// ファイルの平均最適化時間を取得
    /// * `return` - ファイルの平均最適化時間
    pub fn average_duration(&self) -> u64 {
        self.file_info.average_duration
    }

    /// ファイルの JPEG 平均最適化時間を取得
    /// * `return` - ファイルの JPEG 平均最適化時間
    pub fn jpeg_average_duration(&self) -> u64 {
        self.file_info.jpeg_average_duration
    }

    /// ファイルの PNG 平均最適化時間を取得
    /// * `return` - ファイルの PNG 平均最適化時間
    pub fn png_average_duration(&self) -> u64 {
        self.file_info.png_average_duration
    }

    /// パスをクリア
    pub fn clear(&mut self) {
        self.paths.clear();
        self.selected_id = None;
        self.file_info = FileInfo::new();
    }

    /// JPEG ファイルがあるかどうか
    /// * `return` - JPEG ファイルがあるかどうか
    pub fn has_jpeg(&self) -> bool {
        self.paths.iter().any(|f| f.is_jpeg())
    }

    /// PNG ファイルがあるかどうか
    /// * `return` - PNG ファイルがあるかどうか
    pub fn has_png(&self) -> bool {
        self.paths.iter().any(|f| f.is_png())
    }

    /// 未処理ファイルがあるかどうか
    /// * `return` - 未処理ファイルがあるかどうか
    pub fn has_standby(&self) -> bool {
        self.paths.iter().any(|f| f.is_standby())
    }

    /// 最適化中のファイルがあるかどうか
    /// * `return` - 最適化中のファイルがあるかどうか
    pub fn has_optimizing(&self) -> bool {
        self.paths.iter().any(|f| f.is_optimizing())
    }

    /// エラーのファイルがあるかどうか
    /// * `return` - エラーのファイルがあるかどうか
    pub fn has_error(&self) -> bool {
        self.paths.iter().any(|f| f.is_error())
    }

    /// 最適化結果を既存の一覧へ反映
    /// * `results` - 最適化済みのファイル
    pub fn apply_result(&mut self, result: file::ImageFile) {
        // 既存のファイル一覧から ID が一致するファイルを検索
        if let Some(file) = self.paths.iter_mut().find(|f| f.id() == result.id()) {
            // 元のファイルが最適化済みであればスキップ
            if file.is_optimized() {
                return;
            }

            // 元のファイルがキャンセルされていて、最適化済みであれば反映
            if file.is_canceled() && result.is_optimized() {
                *file = result;
                return;
            }

            // それ以外は反映
            *file = result;
        }
    }

    /// ファイルの拡張子が許可されているかどうかを確認
    /// * `path` - ファイルのパス
    /// * `return` - 許可されているかどうか
    fn is_allowed_extension(&self, path: &PathBuf) -> bool {
        if let Some(ext) = path.extension() {
            if let Some(ext) = ext.to_str() {
                if self.extensions.iter().any(|e| e.eq_ignore_ascii_case(ext)) {
                    return true;
                }
            }
        }
        false
    }

    /// ファイルが待機中か最適化中かどうかを確認
    /// * `path` - ファイルのパス
    /// * `return` - 待機中か最適化中かどうか
    fn is_standby_or_optimizing(&self, path: &PathBuf) -> bool {
        self.paths.iter().any(|f| f.path() == path && f.is_standby_or_optimizing())
    }

    /// 入力・出力ファイルが一致しているかどうかを確認
    /// * `path` - ファイルのパス
    /// * `output_path` - 出力ファイルのパス
    /// * `return` - 入力・出力ファイルが一致しているかどうか
    fn is_input_output_path(&self, path: &PathBuf, output_path: &PathBuf) -> bool {
        self.paths.iter().any(|f| {
            f.path() == path
                && f.output_path().as_ref() == Some(output_path)
                && (f.is_optimized() || f.is_unchanged())
        })
    }

    /// 最小の ID を取得
    /// * `return` - 最小の ID
    pub fn get_min_id(&self) -> u64 {
        *self.paths.iter().map(|f| f.id()).min().unwrap_or(&1)
    }

    /// 最大の ID を取得
    /// * `return` - 最大の ID
    pub fn get_max_id(&self) -> u64 {
        *self.paths.iter().map(|f| f.id()).max().unwrap_or(&1)
    }

    /// 選択されたファイルのインデックスを取得
    /// * `return` - 選択されたファイルのインデックス
    pub fn get_selected_index(&self) -> Option<usize> {
        if let Some(id) = self.selected_id() {
            self.paths.iter().position(|p| *p.id() == *id)
        } else {
            None
        }
    }

    /// ID からインデックスを取得
    /// * `id` - ID
    /// * `return` - インデックス
    pub fn get_index_by_id(&self, id: u64) -> Option<usize> {
        self.paths.iter().position(|p| *p.id() == id)
    }

    /// パスを追加
    /// * `app` - アプリケーション
    /// * `path` - ドロップされたファイルのパス
    /// * `return` - 結果
    pub fn add_path(&mut self, app: &app::App, path: PathBuf) -> error::Result<()> {
        // parent() は path を借りるので、先に PathBuf にして借用を終わらせる
        let base_dir = path.parent().unwrap_or(&path).to_path_buf();

        self.find_file(app, path, &base_dir)?;

        Ok(())
    }

    /// ファイルを検索
    /// * `path` - ドロップされたファイルのパス
    /// * `base_dir` - ドロップされたパスの親（相対パスの基準）
    /// * `return` - ファイルのパス
    fn find_file(&mut self,
        app: &app::App,
        path: PathBuf,
        base_dir: &Path
    ) -> error::Result<()> {
        let metadata = path.metadata().map_err(|e| error::KeigaError::FileError(e.to_string(), path.clone()))?;

        if metadata.is_file() {
            if self.is_allowed_extension(&path) {
                // strip_prefix は path を借りるので、
                // 先に String にして into_owned()で所有権を移す
                let relative_path = path.strip_prefix(base_dir)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .into_owned();

                // ファイルを作成
                let mut image_file = file::ImageFile::new(path, relative_path)?;

                // 同じパスが待機中か最適化中なら、新規行をエラーで追加
                if self.is_standby_or_optimizing(&image_file.path()) {
                    image_file.set_status(OptimizeStatus::Error("Already in progress".to_string()));
                    self.paths.push(image_file);
                    return Ok(());
                }

                // 同じ入力を同じ出力先へ書き済みなら、新規行をスキップで追加
                if *app.skip_same_path()
                    && self.is_input_output_path(image_file.path(), &image_file.make_output_path(app))
                {
                    image_file.set_status(OptimizeStatus::Skipped);
                    self.paths.push(image_file);
                    return Ok(());
                }

                self.paths.push(image_file);
            }
        } else if metadata.is_dir() {
            for entry in fs::read_dir(&path).map_err(|e| error::KeigaError::FileError(e.to_string(), path.clone()))? {
                let entry = entry.map_err(|e| error::KeigaError::FileError(e.to_string(), path.clone()))?;
                self.find_file(app, entry.path(), base_dir)?;
            }
        }

        Ok(())
    }
}
