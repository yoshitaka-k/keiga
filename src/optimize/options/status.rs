/// 最適化ステータス
#[derive(Clone, PartialEq)]
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

impl OptimizeStatus {
    /// 最適化ステータスを文字列に変換
    /// * `return` - 最適化ステータスを文字列に変換
    pub fn to_string(&self) -> String {
        match self {
            OptimizeStatus::Standby => "Standby".to_string(),
            OptimizeStatus::Optimizing => "Optimizing".to_string(),
            OptimizeStatus::Optimized => "Optimized".to_string(),
            OptimizeStatus::Unchanged => "Unchanged".to_string(),
            OptimizeStatus::Skipped => "Skipped".to_string(),
            OptimizeStatus::Canceled => "Canceled".to_string(),
            OptimizeStatus::Error(message) => format!("Error: {}", message),
        }
    }
}
