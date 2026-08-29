pub(crate) mod drop;
pub(crate) mod open;
pub(crate) mod button;
pub(crate) mod key;
pub(crate) mod click;
pub(crate) mod input;

/// ファイル一覧のアクション
pub(crate) enum EventAction {
    Click { id: u64 },
    DoubleClick { path: std::path::PathBuf },
    Up { id: u64 },
    Down { id: u64 },
    Enter { path: std::path::PathBuf },
    Space { path: std::path::PathBuf },
    Backspace,
}
