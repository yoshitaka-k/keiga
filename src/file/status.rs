use std::fmt;
use crate::lang;

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
            Status::Standby => write!(f, "{}", lang::current().optimize_status.standby),
            Status::Optimizing => write!(f, "{}", lang::current().optimize_status.optimizing),
            Status::Optimized => write!(f, "{}", lang::current().optimize_status.optimized),
            Status::Unchanged => write!(f, "{}", lang::current().optimize_status.unchanged),
            Status::Skipped => write!(f, "{}", lang::current().optimize_status.skipped),
            Status::Canceled => write!(f, "{}", lang::current().optimize_status.canceled),
            Status::Error(message) => write!(f, "Error: {}", message),
        }
    }
}
