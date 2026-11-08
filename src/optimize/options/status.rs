use std::fmt;

/// 最適化ステータス
#[derive(Clone)]
pub enum OptimizeStatus {
    /// 最適化未実行
    Standby,
    /// 最適化中
    Optimizing,
    /// 最適化完了
    Optimized,
    /// 最適化不要
    Unchanged,
    /// 最適化スキップ
    Skipped,
    /// 最適化キャンセル
    Canceled,
    /// 最適化エラー（メッセージ）
    Error(String),
}

impl fmt::Display for OptimizeStatus {
    /// 最適化ステータスを文字列に変換
    /// * `return` - 最適化ステータスを文字列に変換
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptimizeStatus::Standby => write!(f, "Standby"),
            OptimizeStatus::Optimizing => write!(f, "Optimizing"),
            OptimizeStatus::Optimized => write!(f, "Optimized"),
            OptimizeStatus::Unchanged => write!(f, "Unchanged"),
            OptimizeStatus::Skipped => write!(f, "Skipped"),
            OptimizeStatus::Canceled => write!(f, "Canceled"),
            OptimizeStatus::Error(message) => write!(f, "Error: {}", message),
        }
    }
}
