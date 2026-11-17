use crate::{error, state};

/// スクロールエリアを表示するためのトークン
pub(crate) struct ScrollToken {
    pub selected: bool,
}
impl ScrollToken {
    pub fn new() -> Self {
        Self {
            selected: false,
        }
    }
}

/// 設定ウィンドウを表示するためのトークン
pub(crate) struct SettingToken {
    pub open: bool,
    pub pos: Option<egui::Pos2>,
    pub tab: state::SettingTab,
}
impl SettingToken {
    pub fn new() -> Self {
        Self {
            open: false,
            pos: None,
            tab: state::SettingTab::General,
        }
    }
}

/// エラーモーダルを表示するためのトークン
pub(crate) struct ErrorToken {
    pub open: bool,
    pub value: Option<error::KeigaError>,
}
impl ErrorToken {
    pub fn new() -> Self {
        Self {
            open: false,
            value: None,
        }
    }
}
