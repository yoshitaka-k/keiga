mod job;
pub(crate) use job::UpdateJob;

use serde::Deserialize;
use getset::Getters;
use crate::error;

/// アップデート確認の結果
#[derive(Clone)]
pub(crate) enum UpdateCheck {
    Available { version: String, url: String },
    Latest,
    Failed(String),
}

/// 更新モーダルを表示するためのトークン
#[derive(Clone)]
pub(crate) struct UpdateToken {
    pub running: bool,
    pub open: bool,
    pub check: Option<UpdateCheck>,
}
impl UpdateToken {
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

/// バージョンを比較する
/// * `new` - 新しいバージョン
/// * `old` - 古いバージョン
/// * `return` - 新しいバージョンが古いバージョンより新しいかどうか
pub(crate) fn version_compare(new: &str, old: &str) -> error::Result<bool> {
    let new = new.replace("v", "");
    let old = old.replace("v", "");

    // 新しいバージョンをメジャー、マイナー、パッチに分割
    let parts: Vec<i32> = new.split(".")
        .map(|x| x.parse::<i32>())
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|_| error::KeigaError::invalid_version())?;
    let [new_major, new_minor, new_patch]: [i32; 3] = parts.try_into()
        .map_err(|_| error::KeigaError::invalid_version())?;

    // 古いバージョンをメジャー、マイナー、パッチに分割
    let parts: Vec<i32> = old.split(".")
        .map(|x| x.parse::<i32>())
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|_| error::KeigaError::invalid_version())?;
    let [old_major, old_minor, old_patch]: [i32; 3] = parts.try_into()
        .map_err(|_| error::KeigaError::invalid_version())?;

    // メジャー、マイナー、パッチを比較
    if new_major != old_major {
        // メジャー番号が異なる場合は、メジャー番号の比較
        Ok(new_major > old_major)
    } else if new_minor != old_minor {
        // マイナー番号が異なる場合は、マイナー番号を比較
        Ok(new_minor > old_minor)
    } else {
        // 上記以外の場合は、パッチ番号を比較
        Ok(new_patch > old_patch)
    }
}

// tests ディレクトリ直下は別クレートになって、非公開関数が呼べないので、
// 子モジュールとして読むためのパスを指定する
#[cfg(test)]
#[path = "../../tests/unit/update.rs"]
mod tests;
