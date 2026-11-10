pub(crate) mod drop;
pub(crate) mod open;
pub(crate) mod button;
pub(crate) mod key;
pub(crate) mod click;
pub(crate) mod input;

/// ファイル一覧のアクション
pub(crate) enum EventAction {
    Click { index: usize },
    DoubleClick { path: std::path::PathBuf },
    Up,
    Down,
    Enter { path: std::path::PathBuf },
    Space { path: std::path::PathBuf },
    Backspace,
}
