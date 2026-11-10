use std::fmt;

/// ファイルステータス
#[derive(Clone)]
pub enum Status {
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

impl fmt::Display for Status {
    /// 最適化ステータスを文字列に変換
    /// * `return` - 最適化ステータスを文字列に変換
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Standby => write!(f, "Standby"),
            Status::Optimizing => write!(f, "Optimizing"),
            Status::Optimized => write!(f, "Optimized"),
            Status::Unchanged => write!(f, "Unchanged"),
            Status::Skipped => write!(f, "Skipped"),
            Status::Canceled => write!(f, "Canceled"),
            Status::Error(message) => write!(f, "Error: {}", message),
        }
    }
}
