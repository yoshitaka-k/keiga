use std::path::PathBuf;

/// 最適化ガード
/// * `keep` - 一時ファイルを保持するかどうか
/// * `path` - 一時ファイルのパス
pub(crate) struct OptimizeGuard {
    keep: bool,
    path: PathBuf,
}

impl OptimizeGuard {
    pub(crate) fn new(path: PathBuf) -> Self {
        Self {
            keep: false,
            path,
        }
    }

    /// 削除しない
    pub(crate) fn keep(&mut self) {
        self.keep = true;
    }
}

impl Drop for OptimizeGuard {
    fn drop(&mut self) {
        // 一時ファイルを削除
        if !self.keep {
            let _ = std::fs::remove_file(&self.path);
        }
    }
}
