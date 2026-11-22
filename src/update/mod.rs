mod job;
pub(crate) use job::UpdateJob;

use serde::Deserialize;
use getset::Getters;

/// アップデート確認の結果
#[derive(Clone)]
pub(crate) enum UpdateCheck {
    Available { version: String, url: String },
    Latest,
    Failed,
}

/// 更新モーダルを表示するためのトークン
#[derive(Clone)]
pub(crate) struct UpdatedToken {
    pub running: bool,
    pub open: bool,
    pub check: Option<UpdateCheck>,
}
impl UpdatedToken {
    pub fn new() -> Self {
        Self {
            running: false,
            open: false,
            check: None,
        }
    }
}

/// GitHub レスポンスのアップデート情報
#[derive(Deserialize, Getters)]
#[getset(get)]
struct UpdateInfo {
    tag_name: String,
    html_url: String,
}
