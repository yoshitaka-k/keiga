use std::path::PathBuf;
use crate::optimize;

/// PNG 最適化を行う構造体
pub struct Png;

impl optimize::Optimizer for Png {
    type Options = optimize::PngOptions;

    /// PNG ファイルを最適化
    /// * `path` - 最適化する PNG のパス
    /// * `options` - 最適化オプション
    /// * `return` - エンコードされたファイルのサイズとデータ
    fn encode(
        path: &PathBuf,
        options: Self::Options,
    ) -> Result<(usize, Vec<u8>), Box<dyn std::error::Error>> {
        // 先にファイルを読み込んでおく
        let input = std::fs::read(path)?;

        // oxipng でロスレス最適化（パレット維持・ビット深度削減・再圧縮）
        let output = oxipng::optimize_from_memory(&input, &options.options)?;

        Ok((input.len(), output))
    }
}
