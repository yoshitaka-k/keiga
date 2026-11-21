use std::path::Path;
use getset::{Getters, Setters};

use crate::{config, file};

/// ドロップされたファイルを管理する構造体
#[derive(Clone, Getters, Setters)]
pub(crate) struct OpenFiles {
    /// ファイル一覧
    #[getset(get = "pub")]
    images: Vec<file::Image>,

    /// 選択されたファイルの ID
    #[getset(get = "pub", set = "pub")]
    selected_index: Option<usize>,

    /// ファイル情報
    #[getset(get = "pub")]
    file_info: file::FileInfo,
}

/// public methods
impl OpenFiles {
    /// 新しい OpenFiles を作成
    /// * `return` - OpenFiles のインスタンス
    pub fn new() -> Self {
        Self {
            images: vec![],
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

    /// メタデータを読み込んでないファイルを1件取得
    /// * `return` - メタデータを読み込んでないファイル1件
    pub fn get_pending_file(&mut self) -> Option<&mut file::Image> {
        self.images.iter_mut().find(|f| f.is_pending())
    }

    /// 最適化待機中のファイルを1件取得
    ///
    /// メタデータ読み込み中のファイルは取得しない
    /// 最適化待機中のファイルを取得
    /// png ファイルの取得が許可されている場合は、png ファイルも取得する
    ///
    /// * `allow_png` - PNG ファイルを許可するかどうか
    /// * `return` - 最適化待機中のファイル1件
    pub fn get_standby_file(&mut self, allow_png: bool) -> Option<&mut file::Image> {
        self.images.iter_mut().find(|f| {
            !f.is_pending_or_loading()
            && f.is_standby()
            && (allow_png || !f.is_png())
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
        let mut optimizing_len = file::OptimizingLen::new();
        let mut optimized_len = file::OptimizedLen::new();
        let mut unchanged_len = 0;
        let mut canceled_len = 0;
        let mut skipped_len = 0;
        let mut error_len = 0;

        let mut total_size = file::SizeInfo::new();
        let mut total_new_size = file::NewSizeInfo::new();
        let mut total_duration = file::DurationInfo::new();

        // 各ファイルの情報を集計
        for file in &self.images {
            match file.status() {
                file::Status::Standby => standby_len += 1,
                file::Status::Optimizing => {
                    if file.is_jpeg() {
                        optimizing_len.jpeg += 1;
                    } else if file.is_png() {
                        optimizing_len.png += 1;
                    }
                },
                file::Status::Optimized => {
                    if file.is_jpeg() {
                        optimized_len.jpeg += 1;
                        total_duration.jpeg += file.duration();
                        total_size.jpeg += file.size();
                        total_new_size.jpeg += file.new_size();
                    } else if file.is_png() {
                        optimized_len.png += 1;
                        total_duration.png += file.duration();
                        total_size.png += file.size();
                        total_new_size.png += file.new_size();
                    }
                }
                file::Status::Unchanged => unchanged_len += 1,
                file::Status::Skipped => skipped_len += 1,
                file::Status::Canceled => canceled_len += 1,
                file::Status::Error(_) => error_len += 1,
            }
        }

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
        self.file_info.optimizing_len.total()
    }

    /// 最適化中の JPEG ファイルの数を取得
    /// * `return` - 最適化中の JPEG ファイルの数
    pub fn jpeg_optimizing_len(&self) -> u32 {
        self.file_info.optimizing_len.jpeg()
    }

    /// 最適化中の PNG ファイルの数を取得
    /// * `return` - 最適化中の PNG ファイルの数
    pub fn png_optimizing_len(&self) -> u32 {
        self.file_info.optimizing_len.png()
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

    /// 完了したファイルの数を取得
    /// * `return` - 完了したファイルの数
    pub fn completed_len(&self) -> u32 {
        self.file_info.optimized_len.total() + self.file_info.unchanged_len + self.file_info.skipped_len
    }

    /// ファイルの総平均新サイズを取得
    /// * `return` - ファイルの総平均新サイズ
    pub fn total_average_saved(&self) -> u64 {
        file::calc_average_value(self.file_info.total_new_size.total(), self.file_info.optimized_len.total())
    }

    /// ファイルの JPEG 平均新サイズを取得
    /// * `return` - ファイルの JPEG 平均新サイズ
    pub fn jpeg_average_saved(&self) -> u64 {
        file::calc_average_value(self.file_info.total_new_size.jpeg, self.file_info.optimized_len.jpeg)
    }

    /// ファイルの PNG 平均新サイズを取得
    /// * `return` - ファイルの PNG 平均新サイズ
    pub fn png_average_saved(&self) -> u64 {
        file::calc_average_value(self.file_info.total_new_size.png, self.file_info.optimized_len.png)
    }

    /// ファイルの総節約率を取得
    /// * `return` - ファイルの総節約率
    pub fn total_saved_rate(&self) -> f32 {
        file::calc_saved_rate(self.file_info.total_size.total(), self.file_info.total_new_size.total())
    }

    /// ファイルの JPEG 平均節約率を取得
    /// * `return` - ファイルの JPEG 平均節約率
    pub fn jpeg_average_saved_rate(&self) -> f32 {
        file::calc_saved_rate(self.file_info.total_size.jpeg, self.file_info.total_new_size.jpeg)
    }

    /// ファイルの PNG 平均節約率を取得
    /// * `return` - ファイルの PNG 平均節約率
    pub fn png_average_saved_rate(&self) -> f32 {
        file::calc_saved_rate(self.file_info.total_size.png, self.file_info.total_new_size.png)
    }

    /// ファイルの総最適化時間を取得
    /// * `return` - ファイルの総最適化時間
    pub fn total_average_duration(&self) -> u64 {
        file::calc_average_value(self.file_info.total_duration.total(), self.file_info.optimized_len.total())
    }

    /// ファイルの JPEG 平均最適化時間を取得
    /// * `return` - ファイルの JPEG 平均最適化時間
    pub fn jpeg_average_duration(&self) -> u64 {
        file::calc_average_value(self.file_info.total_duration.jpeg, self.file_info.optimized_len.jpeg)
    }

    /// ファイルの PNG 平均最適化時間を取得
    /// * `return` - ファイルの PNG 平均最適化時間
    pub fn png_average_duration(&self) -> u64 {
        file::calc_average_value(self.file_info.total_duration.png, self.file_info.optimized_len.png)
    }

    /// ジョブの状態を取得
    /// 優先度: 最適化中 > エラー > 最適化済み・変更なし・スキップ > 待機
    /// * `return` - ジョブの状態
    pub fn job_status(&self) -> file::Status {
        if self.file_info.optimizing_len.total() > 0 {
            file::Status::Optimizing
        } else if self.file_info.error_len > 0 {
            file::Status::Error(String::new())
        } else if self.completed_len() > 0
        {
            file::Status::Optimized
        } else {
            file::Status::Standby
        }
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

    /// 最適化待機中のファイルがあるかどうか
    /// * `return` - 最適化待機中のファイルがあるかどうか
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
            // 出力パスが一致するファイルで最適化中、最適化済み、変更なしの状態
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

    /// メタデータ読み込み結果を反映
    /// * `result` - メタデータ読み込み結果
    pub fn apply_metadata_result(&mut self, result: file::Image) {
        // 既存のファイル一覧から ID が一致するファイルを検索
        if let Some(file) = self.images.iter_mut().find(|f| f.id() == result.id()) {
            file.set_metadata(*result.metadata());
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

    /// ファイルスキャン結果を反映
    /// * `config` - 設定
    /// * `result` - ファイルスキャン結果
    pub fn scan_result(&mut self,
        config: &config::Config,
        result: file::ScanResult
    ) {
        match result {
            file::ScanResult::Found { path, base_dir } => {
                // base_dir からの相対パスを求める
                let relative_path = path.strip_prefix(base_dir);

                // フォルダ経由で追加されたファイルかどうか
                let is_in_folder = relative_path.as_ref().is_ok_and(|p| {
                    p.components().count() > 1
                });

                // 所有権のある PathBuf にする
                let relative_path = relative_path.unwrap_or(&path).to_path_buf();

                // 画像ファイル構造体を作成
                let mut image = match file::Image::new(path.to_path_buf(), relative_path, is_in_folder) {
                    Ok(image) => image,
                    Err(e) => {
                        log::error!("Error: {}", e);
                        return;
                    }
                };

                // 出力パスを作成
                let output_path = image.make_output_path(config.output_path());
                image.set_output_path(Some(output_path.clone()));

                // 同じパスが待機中か最適化中の場合はエラーを返す
                if self.is_standby_or_optimizing(&path) {
                    image.set_status(file::Status::Error("Input already in progress".to_string()));
                    self.images.push(image);
                    return;
                }

                // 同じパスはスキップ
                if *config.skip_same_path() && self.is_input_and_output(&path, &output_path) {
                    image.set_status(file::Status::Skipped);
                    self.images.push(image);
                    return;
                }

                // 画像ファイル構造体を追加
                self.images.push(image);
            }
            file::ScanResult::Failed { path, base_dir, error } => {
                // エラーを追加
                self.error_file(&path, &base_dir, error);
            }
        }
    }
}

/// private methods
impl OpenFiles {
    /// ファイルが待機中か最適化中かどうかを確認
    /// * `path` - ファイルのパス
    /// * `return` - 待機中か最適化中かどうか
    fn is_standby_or_optimizing(&self, path: &Path) -> bool {
        self.images.iter().any(|f| f.path() == path && f.is_standby_or_optimizing())
    }

    /// ファイルが入力パスと出力パスが同じかどうかを確認
    /// * `path` - ファイルのパス
    /// * `output_path` - 出力パス
    /// * `return` - 入力パスと出力パスが同じかどうか
    fn is_input_and_output(&self,
        path: &Path,
        output_path: &Path,
    ) -> bool {
        self.images.iter().any(|f| {
            f.path() == path
                && f.output_path().as_deref() == Some(output_path)
                && (f.is_optimized() || f.is_unchanged())
        })
    }

    /// ファイルをエラーで追加
    /// * `path` - ファイルのパス
    /// * `base_dir` - ドロップされたパスの親（相対パスの基準）
    /// * `error` - エラー内容
    fn error_file(&mut self,
        path: &Path,
        base_dir: &Path,
        error: String,
    ) {
        let relative_path = path.strip_prefix(base_dir);
        let is_in_folder = relative_path.as_ref().is_ok_and(|p| p.components().count() > 1);
        let relative_path = relative_path.unwrap_or(path).to_path_buf();

        let mut image = match file::Image::new(path.to_path_buf(), relative_path, is_in_folder) {
            Ok(image) => image,
            Err(e) => {
                log::error!("Error: {}", e);
                return;
            }
        };

        image.set_status(file::Status::Error(error));
        self.images.push(image);
    }
}
