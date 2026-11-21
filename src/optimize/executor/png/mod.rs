mod lossy;
mod lossless;

use std::path::Path;
use crate::{error, optimize};

/// PNG 最適化を行う構造体
pub struct Png;
impl optimize::Optimizer for Png {
    type Options = optimize::PngOptions;

    /// PNG ファイルを最適化
    /// * `path` - 最適化する PNG のパス
    /// * `options` - 最適化オプション
    /// * `return` - 元のファイルサイズとエンコードされたデータ
    fn encode(
        path: &Path,
        options: Self::Options,
    ) -> error::Result<(usize, Vec<u8>)> {
        if options.lossy {
            lossy::optimize(path, options)
        } else {
            lossless::optimize(path, options)
        }
    }
}

// tests ディレクトリ直下は別クレートになって、非公開関数が呼べないので、
// 子モジュールとして読むためのパスを指定する
#[cfg(test)]
#[path = "../../../../tests/unit/optimize.rs"]
mod tests;
