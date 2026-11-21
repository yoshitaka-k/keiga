use std::fmt;
use serde::{Deserialize, Serialize};

/// PNG プリセット
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum PngPreset {
    Min,
    Fast,
    Default,
    Best,
    Max,
}

impl PngPreset {
    /// PNG プリセットを oxipng の oxipng::Options に変換
    /// * `return` - プリセットの oxipng::Options
    pub fn to_options(&self) -> oxipng::Options {
        match self {
            PngPreset::Min => oxipng::Options::from_preset(0),
            PngPreset::Fast => oxipng::Options::from_preset(1),
            PngPreset::Default => oxipng::Options::from_preset(2),
            PngPreset::Best => oxipng::Options::from_preset(4),
            PngPreset::Max => oxipng::Options::from_preset(6),
        }
    }
}

impl fmt::Display for PngPreset {
    /// PNG プリセットを文字列に変換
    /// * `return` - プリセットの文字列
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PngPreset::Min => write!(f, "Min"),
            PngPreset::Fast => write!(f, "Fast"),
            PngPreset::Default => write!(f, "Default"),
            PngPreset::Best => write!(f, "Best"),
            PngPreset::Max => write!(f, "Max"),
        }
    }
}
