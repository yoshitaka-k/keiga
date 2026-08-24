use std::fs;
use std::path::PathBuf;
use getset::{Getters, Setters};

use crate::{app, file};
use crate::optimize::OptimizeStatus;

/// ファイル情報を管理する構造体
#[derive(Clone, PartialEq)]
struct FileInfo {
    standby_len: u32,
    optimizing_len: u32,
    optimized_len: u32,
    unchanged_len: u32,
    skipped_len: u32,
    canceled_len: u32,
    error_len: u32,
    total_size: u64,
    total_new_size: u64,
    total_duration: u64,
}

impl FileInfo {
    pub fn new() -> Self {
        Self {
            standby_len: 0,
            optimizing_len: 0,
            optimized_len: 0,
            unchanged_len: 0,
            skipped_len: 0,
            canceled_len: 0,
            error_len: 0,
            total_size: 0,
            total_new_size: 0,
            total_duration: 0,
        }
    }
}

/// ドロップされたファイルを管理する構造体
#[derive(Clone, PartialEq, Getters, Setters)]
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
            matches!(f.status(), OptimizeStatus::Standby) && (allow_png || !f.is_png())
        })
    }

    /// 選択されたファイルのパスを取得
    /// * `return` - 選択されたファイルのパス
    pub fn selected_path(&self) -> Option<PathBuf> {
        if let Some(id) = self.selected_id() {
            self.paths.iter().find(|p| *p.id() == *id).map(|p| p.path().clone())
        } else {
            None
        }
    }

    /// キャンセルされたファイルのステータスを更新
    /// * `id` - キャンセルされたファイルの ID
    pub fn set_status_canceled(&mut self, id: u64) {
        if let Some(file) = self.paths.iter_mut().find(|f| *f.id() == id) {
            // 最適化済み・最適化不要・スキップはキャンセルできない
            if !matches!(file.status(), OptimizeStatus::Optimized | OptimizeStatus::Unchanged | OptimizeStatus::Skipped) {
                file.set_status(OptimizeStatus::Canceled);
            }
        }
    }

    /// ファイル情報を更新
    pub fn update_file_length(&mut self) {
        let mut standby_len = 0;
        let mut optimizing_len = 0;
        let mut optimized_len = 0;
        let mut unchanged_len = 0;
        let mut canceled_len = 0;
        let mut skipped_len = 0;
        let mut error_len = 0;

        for file in &self.paths {
            match file.status() {
                OptimizeStatus::Standby => standby_len += 1,
                OptimizeStatus::Optimizing => optimizing_len += 1,
                OptimizeStatus::Optimized => optimized_len += 1,
                OptimizeStatus::Unchanged => unchanged_len += 1,
                OptimizeStatus::Skipped => skipped_len += 1,
                OptimizeStatus::Canceled => canceled_len += 1,
                OptimizeStatus::Error(_) => error_len += 1,
            }
        }

        self.file_info.standby_len = standby_len;
        self.file_info.optimizing_len = optimizing_len;
        self.file_info.optimized_len = optimized_len;
        self.file_info.unchanged_len = unchanged_len;
        self.file_info.skipped_len = skipped_len;
        self.file_info.canceled_len = canceled_len;
        self.file_info.error_len = error_len;
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
        self.file_info.optimized_len
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

    /// ファイルのサイズを計算
    pub fn calc_total_info(&mut self) {
        let mut total_size = 0;
        let mut total_new_size = 0;
        let mut total_duration = 0;

        for file in &self.paths {
            if matches!(file.status(), OptimizeStatus::Optimized) {
                total_size += file.size();
                total_new_size += file.new_size();
                total_duration += file.duration();
            }
        }

        self.file_info.total_size = total_size;
        self.file_info.total_new_size = total_new_size;
        self.file_info.total_duration = total_duration;
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

    /// ファイルの総最適化時間を取得
    /// * `return` - ファイルの総最適化時間
    pub fn total_duration(&self) -> u64 {
        self.file_info.total_duration
    }

    /// ファイルの総節約率を取得
    /// * `return` - ファイルの総節約率
    pub fn total_saved_rate(&mut self) -> f32 {
        self.calc_total_info();

        if self.total_new_size() == 0 {
            return 0.00;
        }

        // 最適化後のファイズによってパーセントを計算
        if self.total_size() >= self.total_new_size() {
            (self.total_size() - self.total_new_size()) as f32 / self.total_size() as f32 * 100.0 * -1.0
        } else {
            (self.total_new_size() - self.total_size()) as f32 / self.total_size() as f32 * 100.0 * 1.0
        }
    }

    /// パスをクリア
    pub fn clear(&mut self) {
        self.paths.clear();
        self.selected_id = None;
    }

    /// パスを追加
    /// * `app` - アプリケーション
    /// * `path` - ドロップされたファイルのパス
    /// * `return` - 結果
    pub fn add_path(&mut self, app: &app::App, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        self.find_file(app, path)?;
        Ok(())
    }

    /// 未処理ファイルがあるかどうか
    /// * `return` - 未処理ファイルがあるかどうか
    pub fn has_standby(&self) -> bool {
        self.paths.iter().any(|f| matches!(f.status(), OptimizeStatus::Standby))
    }

    /// 最適化中のファイルがあるかどうか
    /// * `return` - 最適化中のファイルがあるかどうか
    pub fn has_optimizing(&self) -> bool {
        self.paths.iter().any(|f| matches!(f.status(), OptimizeStatus::Optimizing))
    }

    /// 最適化結果を既存の一覧へ反映
    /// * `results` - 最適化済みのファイル
    pub fn apply_result(&mut self, result: file::ImageFile) {
        // 既存のファイル一覧から ID が一致するファイルを検索
        if let Some(file) = self.paths.iter_mut().find(|f| f.id() == result.id()) {
            // 元のファイルが最適化済みであればスキップ
            if matches!(file.status(), OptimizeStatus::Optimized) {
                return;
            }

            // 元のファイルがキャンセルされていて、最適化済みであれば反映
            if matches!(file.status(), OptimizeStatus::Canceled) && matches!(result.status(), OptimizeStatus::Optimized) {
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
        self.paths.iter().any(|f| f.path() == path && matches!(f.status(), OptimizeStatus::Standby | OptimizeStatus::Optimizing))
    }

    /// ファイルが最適化済み・最適化不要・スキップされているかどうかを確認
    /// * `path` - ファイルのパス
    /// * `return` - 最適化済みかどうか
    fn is_optimized(&self, path: &PathBuf) -> bool {
        self.paths.iter().any(|f| f.path() == path && matches!(f.status(), OptimizeStatus::Optimized | OptimizeStatus::Unchanged | OptimizeStatus::Skipped))
    }

    /// ファイルを検索
    /// * `path` - ドロップされたファイルのパス
    /// * `return` - ファイルのパス
    fn find_file(&mut self, app: &app::App, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let metadata = path.metadata().map_err(|e| format!("{} \n\n{}", path.display(), e))?;

        if metadata.is_file() {
            if self.is_allowed_extension(&path) {
                // 同じパスが最適化済みなら、新規行をスキップで追加
                if *app.skip_same_path() && self.is_optimized(&path) {
                    let mut image_file = file::ImageFile::new(path)?;
                    image_file.set_status(OptimizeStatus::Skipped);
                    self.paths.push(image_file);
                    return Ok(());
                }

                // 同じパスが待機中か最適化中なら、新規行をエラーで追加
                if self.is_standby_or_optimizing(&path) {
                    let mut image_file = file::ImageFile::new(path)?;
                    image_file.set_status(OptimizeStatus::Error("Already in progress".to_string()));
                    self.paths.push(image_file);
                    return Ok(());
                }

                let image_file = file::ImageFile::new(path)?;
                self.paths.push(image_file);
            }
        } else if metadata.is_dir() {
            for entry in fs::read_dir(&path).map_err(|e| format!("{} \n\n{}", path.display(), e))? {
                let entry = entry.map_err(|e| format!("{} \n\n{}", path.display(), e))?;
                self.find_file(app, entry.path())?;
            }
        }

        Ok(())
    }
}
