mod generated {
    include!(concat!(env!("OUT_DIR"), "/lang_generated.rs"));
}

pub(crate) use generated::*;

use std::sync::RwLock;

/// 現在の言語
static CURRENT: RwLock<&'static Labels> = RwLock::new(&EN);

/// 現在の言語を返す
/// * `return` - 現在の言語
pub fn current() -> &'static Labels {
    *CURRENT.read().expect("lang lock poisoned")
}

/// 言語を設定する
/// * `code` - 設定する言語
pub fn set(code: &str) {
    let labels = LIST.iter().copied()
        .find(|label| label.code == code)
        .unwrap_or(&EN);
    *CURRENT.write().expect("Failed to write language") = labels;
}
