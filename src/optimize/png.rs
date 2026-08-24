use std::path::PathBuf;
use crate::optimize::{create_output_path, replace_file, OptimToken, OptimizeStatus, TEMP_EXTENSION};

/// PNG 最適化を行う構造体
pub struct Png;

impl Png {
    /// PNG ファイルを最適化
    /// * `path` - 最適化する PNG のパス
    /// * `options` - PNG 最適化オプション
    /// * `token` - 最適化トークン
    /// * `return` - 最適化の結果
    pub fn optimize(path: &PathBuf,
        output_path: &PathBuf,
        options: oxipng::Options,
        token: OptimToken
    ) -> Result<OptimizeStatus, Box<dyn std::error::Error>> {
        // 最適化中止された場合は処理を中断
        if token.is_canceled()? {
            return Ok(OptimizeStatus::Canceled);
        }

        // 先にファイルを読み込んでおく
        let input = std::fs::read(path)?;

        // oxipng でロスレス最適化（パレット維持・ビット深度削減・再圧縮）
        let output = oxipng::optimize_from_memory(&input, &options)?;

        // 最適化中止された場合は処理を中断
        if token.is_canceled()? {
            return Ok(OptimizeStatus::Canceled);
        }

        // 最適化後のサイズが元のサイズより大きい場合は最適化しない
        let size = input.len() as usize;
        let new_size = output.len() as usize;
        if size <= new_size {
            // 出力ファイルが存在しない場合は作成
            if path != output_path && !output_path.exists() {
                create_output_path(&output_path)?;
                std::fs::copy(&path, &output_path)?;
            }
            return Ok(OptimizeStatus::Unchanged);
        }

        // 出力ファイルのパスを作成
        if let Err(e) = create_output_path(&output_path) {
            return Err(e);
        }

        // 一時ファイルを作成して最適化後のデータを保存
        let temp_path = output_path.with_added_extension(TEMP_EXTENSION);
        std::fs::write(&temp_path, &output)?;

        // 最適化中止された場合は処理を中断
        if token.is_canceled()? {
            std::fs::remove_file(&temp_path)?;
            return Ok(OptimizeStatus::Canceled);
        }

        // 一時ファイルを元のファイルに上書き
        replace_file(&temp_path, &output_path)?;

        Ok(OptimizeStatus::Optimized)
    }
}
