use std::fs;
use std::path::{Path, PathBuf};
use getset::{Getters, Setters};

use crate::{config, error, file};

/// ドロップされたファイルを管理する構造体
#[derive(Clone, Getters, Setters)]
pub(crate) struct OpenFiles {
    /// ファイル一覧
    #[getset(get = "pub")]
    images: Vec<file::Image>,

    /// 許可されたファイル拡張子
    #[getset(set = "pub")]
    extensions: Vec<String>,

    /// 選択されたファイルの ID
    #[getset(get = "pub", set = "pub")]
    selected_index: Option<usize>,

    /// ファイル情報
    #[getset(get = "pub")]
    file_info: file::FileInfo,
}

impl OpenFiles {
    /// 新しい OpenFiles を作成
    /// * `return` - OpenFiles のインスタンス
    pub fn new() -> Self {
        Self {
            images: vec![],
            extensions: vec![],
            selected_index: None,
            file_info: file::FileInfo::new(),
        }
    }

    /// パスをクリア
    pub fn clear(&mut self) {
        self.images.clear();
        self.selected_index = None;
        self.file_info = file::FileInfo::new();
    }

    /// 待機中のファイルを1件取得
    /// * `return` - 待機中のファイル1件
    pub fn get_standby_file(&mut self, allow_png: bool) -> Option<&mut file::Image> {
        self.images.iter_mut().find(|f| {
            f.is_standby() && (allow_png || !f.is_png())
        })
    }

    /// 選択されたファイルを取得
    /// * `return` - 選択されたファイル
    pub fn selected_image(&self) -> Option<&file::Image> {
        self.selected_index.and_then(|index| self.images.get(index))
    }

    /// キャンセルされたファイルのステータスを更新
    /// * `id` - キャンセルされたファイルの ID
    pub fn set_status_canceled(&mut self, id: u64) {
        if let Some(file) = self.images.iter_mut().find(|f| *f.id() == id) {
            // 最適化済み・最適化不要・スキップ・エラーはキャンセルできない
            if !file.is_optimized() && !file.is_unchanged() && !file.is_skipped() && !file.is_error() {
                file.set_status(file::Status::Canceled);
            }
        }
    }

    /// エラーのファイルのステータスを更新
    /// * `id` - エラーのファイルの ID
    /// * `error` - エラー内容
    pub fn set_status_error(&mut self, id: u64, error: &str) {
        if let Some(file) = self.images.iter_mut().find(|f| *f.id() == id) {
            file.set_status(file::Status::Error(error.to_string()));
        }
    }

    /// ファイル情報を更新
    pub fn update_file_info(&mut self) {
        let mut standby_len = 0;
        let mut optimizing_len = 0;
        let mut optimized_len = file::OptimizedLen::new();
        let mut unchanged_len = 0;
        let mut canceled_len = 0;
        let mut skipped_len = 0;
        let mut error_len = 0;

        let mut total_size = 0;
        let mut total_new_size = 0;
        let mut total_duration = file::DurationInfo::new();

        // 各ファイルの情報を集計
        for file in &self.images {
            match file.status() {
                file::Status::Standby => standby_len += 1,
                file::Status::Optimizing => optimizing_len += 1,
                file::Status::Optimized => {
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
                file::Status::Unchanged => unchanged_len += 1,
                file::Status::Skipped => skipped_len += 1,
                file::Status::Canceled => canceled_len += 1,
                file::Status::Error(_) => error_len += 1,
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
        self.images.len()
    }

    /// ファイルが空かどうかを取得
    /// * `return` - ファイルが空かどうか
    pub fn is_empty(&self) -> bool {
        self.images.is_empty()
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

    /// エラーのファイルの数を取得
    /// * `return` - エラーのファイルの数
    pub fn error_len(&self) -> u32 {
        self.file_info.error_len
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

    /// JPEG ファイルがあるかどうか
    /// * `return` - JPEG ファイルがあるかどうか
    pub fn has_jpeg(&self) -> bool {
        self.images.iter().any(|f| f.is_jpeg())
    }

    /// PNG ファイルがあるかどうか
    /// * `return` - PNG ファイルがあるかどうか
    pub fn has_png(&self) -> bool {
        self.images.iter().any(|f| f.is_png())
    }

    /// 未処理ファイルがあるかどうか
    /// * `return` - 未処理ファイルがあるかどうか
    pub fn has_standby(&self) -> bool {
        self.images.iter().any(|f| f.is_standby())
    }

    /// 最適化中のファイルがあるかどうか
    /// * `return` - 最適化中のファイルがあるかどうか
    pub fn has_optimizing(&self) -> bool {
        self.images.iter().any(|f| f.is_optimizing())
    }

    /// エラーのファイルがあるかどうか
    /// * `return` - エラーのファイルがあるかどうか
    pub fn has_error(&self) -> bool {
        self.images.iter().any(|f| f.is_error())
    }

    /// 入力が違くて、出力パスが一致するファイルが処理中、処理済みの状態であるかどうか
    /// * `path` - 入力パス
    /// * `output_path` - 出力パス
    /// * `return` - 入力が違くて、出力パスが一致するファイルが処理中、処理済みの状態であるかどうか
    pub fn has_output_conflict(&self,
        path: &Path,
        output_path: Option<&Path>,
    ) -> bool {
        if let Some(output_path) = output_path {
            // 出力パスが一致するファイルでリサイズ中、リサイズ済み、変更なしの状態
            self.images.iter().any(|f| {
                f.path() != path
                    && f.output_path().as_deref() == Some(output_path)
                    && (
                        f.is_optimizing()
                        || f.is_optimized()
                        || f.is_unchanged()
                    )
            })
        } else {
            false
        }
    }

    /// 最適化結果を既存の一覧へ反映
    /// * `results` - 最適化済みのファイル
    pub fn apply_result(&mut self, result: file::Image) {
        // 既存のファイル一覧から ID が一致するファイルを検索
        if let Some(file) = self.images.iter_mut().find(|f| f.id() == result.id()) {
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
    fn is_allowed_extension(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension()
            && let Some(ext) = ext.to_str()
                && self.extensions.iter().any(|e| e.eq_ignore_ascii_case(ext)) {
                    return true;
                }
        false
    }

    /// ファイルが待機中か最適化中かどうかを確認
    /// * `path` - ファイルのパス
    /// * `return` - 待機中か最適化中かどうか
    fn is_standby_or_optimizing(&self, path: &Path) -> bool {
        self.images.iter().any(|f| f.path() == path && f.is_standby_or_optimizing())
    }

    /// 入力・出力ファイルが一致しているかどうかを確認
    /// * `path` - ファイルのパス
    /// * `output_path` - 出力ファイルのパス
    /// * `return` - 入力・出力ファイルが一致しているかどうか
    fn is_input_output_path(&self, path: &Path, output_path: &Path) -> bool {
        self.images.iter().any(|f| {
            f.path() == path
                && f.output_path().as_deref() == Some(output_path)
                && (f.is_optimized() || f.is_unchanged())
        })
    }

    /// 選択されたファイルのインデックスを1増やす
    pub fn index_add(&mut self) {
        let selected = self.selected_index.map(|i| {
            let last = self.images.len().saturating_sub(1);
            (i + 1).min(last)
        });

        self.selected_index = selected;
    }

    /// 選択されたファイルのインデックスを1減らす
    pub fn index_sub(&mut self) {
        let selected = self.selected_index.map(|i| {
            i.saturating_sub(1)
        });

        self.selected_index = selected;
    }

    /// パスを追加
    /// * `config` - 設定
    /// * `path` - ドロップされたファイルのパス
    /// * `return` - 結果
    pub fn add_path(&mut self, config: &config::Config, path: PathBuf) -> error::Result<()> {
        // parent() は path を借りるので、先に PathBuf にして借用を終わらせる
        let base_dir = path.parent().unwrap_or(&path).to_path_buf();

        self.find_file(config, path, &base_dir)?;

        Ok(())
    }

    /// ファイルをエラーで追加
    /// * `path` - ファイルのパス
    /// * `base_dir` - ドロップされたパスの親（相対パスの基準）
    /// * `error` - エラー内容
    fn error_file(&mut self,
        path: &Path,
        base_dir: &Path,
        error: String,
    ) -> error::Result<()> {
        let relative_path = path.strip_prefix(base_dir);
        let relative_path = relative_path.unwrap_or(path).to_path_buf().to_string_lossy().into_owned();

        let mut image = match file::Image::new(path.to_path_buf(), relative_path) {
            Ok(image) => image,
            Err(e) => return Err(error::KeigaError::file_error(e.to_string(), path.to_path_buf())),
        };

        image.set_status(file::Status::Error(error));
        self.images.push(image);

        Ok(())
    }

    /// ファイルを検索
    /// * `config` - 設定
    /// * `path` - ドロップされたファイルのパス
    /// * `base_dir` - ドロップされたパスの親（相対パスの基準）
    /// * `return` - ファイルのパス
    fn find_file(&mut self,
        config: &config::Config,
        path: PathBuf,
        base_dir: &Path
    ) -> error::Result<()> {
        // ファイルの存在を確認
        // シンボリックリンクは無視する
        let metadata = match path.symlink_metadata() {
            Ok(metadata) => metadata,
            Err(e) => return Err(error::KeigaError::file_error(e.to_string(), path.clone())),
        };

        if metadata.is_file() {
            if self.is_allowed_extension(&path) {
                // strip_prefix は path を借りるので、
                // 先に String にして into_owned()で所有権を移す
                let relative_path = path.strip_prefix(base_dir)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .into_owned();

                // ファイルを作成
                let image = file::Image::new(path, relative_path)?;

                // 同じパスが待機中か最適化中なら、新規行をエラーで追加
                if self.is_standby_or_optimizing(image.path()) {
                    self.error_file(image.path(), base_dir, "Already in progress".to_string())?;
                }

                // 同じ入力を同じ出力先へ書き済みなら、新規行をスキップで追加
                if *config.skip_same_path()
                    && self.is_input_output_path(image.path(), &image.make_output_path(config))
                {
                    self.error_file(image.path(), base_dir, "Already processed".to_string())?;
                }

                self.images.push(image);
            }
        } else if metadata.is_dir() {
            for entry in fs::read_dir(&path).map_err(|e| error::KeigaError::file_error(e.to_string(), path.clone()))? {
                let entry = entry.map_err(|e| error::KeigaError::file_error(e.to_string(), path.clone()))?;
                self.find_file(config, entry.path(), base_dir)?;
            }
        }

        Ok(())
    }
}
