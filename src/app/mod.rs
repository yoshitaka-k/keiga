mod update;

pub(crate) use update::{UpdateCheck, UpdateJob, UpdatedToken};

use getset::{Getters, MutGetters};
use serde::{Deserialize, Serialize};
use oxipng::{Options, StripChunks};

use crate::optimize::options::PngPreset;

/// 最適化数のデフォルト値
const DEFAULT_OPTIMIZATION_NUM: u8 = 4;

/// PNG 最適化数のデフォルト値
const DEFAULT_PNG_OPTIMIZATION_NUM: u8 = 2;

/// JPEG 品質のデフォルト値
const DEFAULT_JPEG_QUALITY: u8 = 80;

/// PNG 最適化プリセットのデフォルト値
const DEFAULT_PNG_PRESET: PngPreset = PngPreset::Default;

/// GitHub リポジトリ URL
pub(crate) const GITHUB_URL: &str = "https://github.com/{repository}";

/// アップデート確認リクエスト URL
pub(crate) const REQUEST_URL: &str = "https://api.github.com/repos/{repository}/releases/latest";

/// アプリケーションを管理する構造体
#[derive(Clone, Getters, MutGetters)]
#[derive(Serialize, Deserialize)]
pub struct App {
    /// 読み込める拡張子
    #[getset(get = "pub")]
    #[serde(skip, default = "default_extensions")]
    extensions: Vec<&'static str>,

    /// 実行する最適化数
    #[getset(get = "pub", get_mut = "pub")]
    #[serde(default = "default_optimization_num")]
    optimization_num: u8,

    /// PNG 最適化数
    #[getset(get = "pub", get_mut = "pub")]
    #[serde(default = "default_png_optimization_num")]
    png_optimization_num: u8,

    /// JPEG 品質
    #[getset(get = "pub", get_mut = "pub")]
    jpeg_quality: u8,

    /// PNG 最適化プリセット
    #[getset(get = "pub", get_mut = "pub")]
    png_preset: PngPreset,

    /// 同じパスはスキップ
    #[getset(get = "pub", get_mut = "pub")]
    #[serde(default = "default_skip_same_path")]
    skip_same_path: bool,

    /// 出力パス
    #[getset(get = "pub", get_mut = "pub")]
    #[serde(default = "default_output_path")]
    output_path: String,
}

/// 最適化数のデフォルト値
/// * `return` - 最適化数のデフォルト値
fn default_optimization_num() -> u8 {
    DEFAULT_OPTIMIZATION_NUM
}

/// PNG 最適化数のデフォルト値
fn default_png_optimization_num() -> u8 {
    DEFAULT_PNG_OPTIMIZATION_NUM
}

/// 同じパスはスキップのデフォルト値
fn default_skip_same_path() -> bool {
    true
}

/// 出力パスのデフォルト値
fn default_output_path() -> String {
    "".to_string()
}

/// 読み込める拡張子
/// image crate でサポートされている拡張子
/// これらの拡張子は読み込めるが、全て最適化可能ではない
/// Unsupported メッセージを出す用にも利用するため
/// * `return` - 読み込める拡張子
fn default_extensions() -> Vec<&'static str> {
    vec!["png", "jpg", "jpeg", "gif", "webp", "bmp", "tiff", "tif", "ico", "avif", "qoi", "exr", "tga", "dds", "pbm", "pgm", "ppm", "pam", "hdr", "ff"]
}

impl App {
    /// 新しい App を作成
    /// * `return` - App のインスタンス
    pub fn new() -> Self {
        Self {
            extensions: default_extensions(),
            optimization_num: default_optimization_num(),
            png_optimization_num: default_png_optimization_num(),
            jpeg_quality: DEFAULT_JPEG_QUALITY,
            png_preset: DEFAULT_PNG_PRESET,
            skip_same_path: default_skip_same_path(),
            output_path: default_output_path(),
        }
    }

    /// 拡張子を文字列に変換
    /// * `return` - 拡張子のベクタ
    pub fn extensions_to_string(&self) -> Vec<String> {
        self.extensions.iter().map(|ext| ext.to_string()).collect()
    }

    /// PNG 最適化オプションを作成
    /// * `return` - 最適化オプション
    pub fn png_options(&self) -> Options {
        let mut options = self.png_preset().to_options();
        options.strip = StripChunks::Safe;
        options
    }
}
