mod common;
mod lossy;
mod lossless;

use std::path::Path;
use crate::{error, optimize};

/// JPEG 最適化を行う構造体
pub struct Jpeg;
impl optimize::Optimizer for Jpeg {
    type Options = optimize::JpegOptions;

    /// JPEG ファイルを最適化
    /// * `path` - 最適化する JPEG のパス
    /// * `options` - 最適化オプション
    /// * `return` - 元のファイルサイズとエンコードされたデータ
    fn encode(
        path: &Path,
        options: Self::Options,
    ) -> error::Result<(usize, Vec<u8>)> {
        if options.lossy {
            lossy::optimize(path, options)
        } else {
            lossless::optimize(path)
        }
    }
}
