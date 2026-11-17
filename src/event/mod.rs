pub(crate) mod open;
pub(crate) mod button;
pub(crate) mod key;
pub(crate) mod click;
pub(crate) mod input;

/// ファイル一覧のアクション
pub(crate) enum EventAction {
    #[allow(dead_code)]
    FileOpen,
    FolderOpen,
    DropOpen(Vec<std::path::PathBuf>),
    SettingOpen(Option<egui::Pos2>),
    Click { index: Option<usize> },
    DoubleClick(std::path::PathBuf),
    Up,
    Down,
    Enter { path: std::path::PathBuf },
    Space { path: std::path::PathBuf },
    Backspace,
}
