use std::ffi::OsStr;

#[derive(Clone)]
pub enum Extension {
    Jpeg,
    Png,
    None,
}

impl Extension {
    /// 文字列から Extension を作成
    /// * `extension` - 文字列
    /// * `return` - Extension
    pub fn from_str(extension: &OsStr) -> Self {
        match extension.to_ascii_lowercase().to_string_lossy().as_ref() {
            "jpg" | "jpeg" => Self::Jpeg,
            "png" => Self::Png,
            _ => Self::None,
        }
    }
}
