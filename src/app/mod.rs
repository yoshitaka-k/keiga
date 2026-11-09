use getset::{Getters, MutGetters};
use serde::{Deserialize, Serialize};
use oxipng::{Options, StripChunks};

use crate::optimize::options::PngPreset;

/// JPEG 品質のデフォルト値
const DEFAULT_JPEG_QUALITY: u8 = 80;
/// PNG ディザリングのデフォルト値
const DEFAULT_PNG_DITHERING: u8 = 80;
/// PNG 最適化プリセットのデフォルト値
const DEFAULT_PNG_PRESET: PngPreset = PngPreset::Default;
/// 最適化数のデフォルト値
const DEFAULT_OPTIMIZATION_NUM: u8 = 4;
/// PNG 最適化数のデフォルト値
const DEFAULT_PNG_OPTIMIZATION_NUM: u8 = 2;
/// 効果音の音量のデフォルト値
const DEFAULT_SOUND_VOLUME: u8 = 5;

// JPEG の品質の最小値
pub(crate) const JPEG_QUALITY_MIN: u8 = 60;
// JPEG の品質の最大値
pub(crate) const JPEG_QUALITY_MAX: u8 = 100;

// PNG のディザリングの最小値
pub(crate) const PNG_DITHERING_MIN: u8 = 0;
// PNG のディザリングの最大値
pub(crate) const PNG_DITHERING_MAX: u8 = 100;

// 並行処理数の最小値
pub(crate) const OPTIMIZATION_NUM_MIN: u8 = 3;
// 並行処理数の最大値
pub(crate) const OPTIMIZATION_NUM_MAX: u8 = 8;

// PNG 最適化数の最小値
pub(crate) const PNG_OPTIMIZATION_NUM_MIN: u8 = 1;
// PNG 最適化数の最大値
pub(crate) const PNG_OPTIMIZATION_NUM_MAX: u8 = 3;

// 効果音の音量のスライダーの最小値
pub(crate) const SOUND_VOLUME_MIN: u8 = 1;
// 効果音の音量のスライダーの最大値
pub(crate) const SOUND_VOLUME_MAX: u8 = 10;

/// アプリケーションを管理する構造体
#[derive(Clone, Getters, MutGetters)]
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct App {
    /// 読み込める拡張子
    #[getset(get = "pub")]
    #[serde(skip)]
    extensions: Vec<&'static str>,

    /// 実行する最適化数
    #[getset(get = "pub", get_mut = "pub")]
    optimization_num: u8,

    /// 実行する最適化数 内の PNG 最適化数
    #[getset(get = "pub", get_mut = "pub")]
    png_optimization_num: u8,

    /// JPEG Lossy 最適化
    #[getset(get = "pub", get_mut = "pub")]
    jpeg_lossy: bool,

    /// JPEG 品質
    #[getset(get = "pub", get_mut = "pub")]
    jpeg_quality: u8,

    /// PNG Lossy 最適化
    #[getset(get = "pub", get_mut = "pub")]
    png_lossy: bool,

    /// PNG 最適化プリセット
    #[getset(get = "pub", get_mut = "pub")]
    png_preset: PngPreset,

    /// PNG ディザリング
    #[getset(get = "pub", get_mut = "pub")]
    png_dithering: u8,

    /// 同じパスはスキップ
    #[getset(get = "pub", get_mut = "pub")]
    skip_same_path: bool,

    /// 出力パス
    #[getset(get = "pub", get_mut = "pub")]
    output_path: String,

    /// 効果音鳴らすかどうか
    #[getset(get = "pub", get_mut = "pub")]
    play_sound: bool,

    /// 効果音の音量
    #[getset(get = "pub", get_mut = "pub")]
    sound_volume: u8,
}

/// App のデフォルト値
impl Default for App {
    fn default() -> Self {
        Self {
            extensions: default_extensions(),
            optimization_num: DEFAULT_OPTIMIZATION_NUM,
            png_optimization_num: DEFAULT_PNG_OPTIMIZATION_NUM,
            jpeg_lossy: false,
            jpeg_quality: DEFAULT_JPEG_QUALITY,
            png_lossy: false,
            png_dithering: DEFAULT_PNG_DITHERING,
            png_preset: DEFAULT_PNG_PRESET,
            skip_same_path: true,
            output_path: "".to_string(),
            play_sound: true,
            sound_volume: DEFAULT_SOUND_VOLUME,
        }
    }
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
        Self::default()
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

    /// 値をクランプする
    pub(crate) fn clamp_values(&mut self) {
        self.jpeg_quality = self.jpeg_quality.clamp(JPEG_QUALITY_MIN, JPEG_QUALITY_MAX);
        self.png_dithering = self.png_dithering.clamp(PNG_DITHERING_MIN, PNG_DITHERING_MAX);
        self.optimization_num = self.optimization_num.clamp(OPTIMIZATION_NUM_MIN, OPTIMIZATION_NUM_MAX);
        self.png_optimization_num = self.png_optimization_num.clamp(PNG_OPTIMIZATION_NUM_MIN, PNG_OPTIMIZATION_NUM_MAX);
        self.sound_volume = self.sound_volume.clamp(SOUND_VOLUME_MIN, SOUND_VOLUME_MAX);
    }
}
