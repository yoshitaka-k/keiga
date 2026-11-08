use std::fmt;
use serde::{Deserialize, Serialize};
use oxipng::Options;

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
    /// PNG プリセットを oxipng の Options に変換
    /// * `return` - プリセットの Options
    pub fn to_options(&self) -> Options {
        match self {
            PngPreset::Min => Options::from_preset(0),
            PngPreset::Fast => Options::from_preset(1),
            PngPreset::Default => Options::from_preset(2),
            PngPreset::Best => Options::from_preset(4),
            PngPreset::Max => Options::from_preset(6),
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
