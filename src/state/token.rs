use crate::{error, state};

/// ファイル一覧を表示するためのトークン
pub(crate) struct ListRowToken {
    pub range: std::ops::Range<usize>,
    pub height: f32,
}

/// ファイルダイアログを表示するためのトークン
pub(crate) struct OpenDialogToken {
    pub file_dialog: bool,
    pub folder_dialog: bool,
}

/// 設定ウィンドウを表示するためのトークン
pub(crate) struct SettingToken {
    pub open: bool,
    pub pos: Option<egui::Pos2>,
    pub tab: state::SettingTab,
}

/// エラーモーダルを表示するためのトークン
pub(crate) struct ErrorToken {
    pub open: bool,
    pub value: Option<error::KeigaError>,
}
