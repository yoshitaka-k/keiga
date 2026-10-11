use std::{path, error, fmt, io};

pub type Result<T> = std::result::Result<T, KeigaError>;

#[derive(Debug)]
pub enum KeigaError {
    FileNotFound(String, path::PathBuf, String, u32),
    FileError(String, path::PathBuf, String, u32),
    UnsupportedExtension(path::PathBuf, String, u32),
    OptimizedError(String, path::PathBuf, String, u32),
    LockPoisoned(String, u32),
    InvalidVersion(String, u32),
    Io(io::Error, String, u32),
}

/// KeigaError を表示
impl fmt::Display for KeigaError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = self.code();
        match self {
            KeigaError::FileNotFound(e, p, _, _) => write!(fmt, "[{}] {}: {}", code, e, p.display()),
            KeigaError::FileError(e, p, _, _) => write!(fmt, "[{}] File error: {} \n\n{}", code, e, p.display()),
            KeigaError::UnsupportedExtension(p, _, _) => write!(fmt, "[{}] Unsupported extension: {}", code, p.display()),
            KeigaError::OptimizedError(e, p, _, _) => write!(fmt, "[{}] Optimized error: {} \n\n{}", code, e, p.display()),
            KeigaError::LockPoisoned(_, _) => write!(fmt, "[{}] Lock poisoned", code),
            KeigaError::InvalidVersion(_, _) => write!(fmt, "[{}] Invalid version", code),
            KeigaError::Io(e, _, _) => write!(fmt, "[{}] IO error: {}", code, e),
        }
    }
}

/// KeigaError を表示
impl error::Error for KeigaError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            KeigaError::FileNotFound(_, _, _, _) => None,
            KeigaError::FileError(_, _, _, _) => None,
            KeigaError::UnsupportedExtension(_, _, _) => None,
            KeigaError::OptimizedError(_, _, _, _) => None,
            KeigaError::LockPoisoned(_, _) => None,
            KeigaError::InvalidVersion(_, _) => None,
            KeigaError::Io(ref e, _, _) => Some(e),
        }
    }
}

/// std::io::Error を KeigaError に変換
impl From<io::Error> for KeigaError {
    #[track_caller]
    fn from(e: io::Error) -> Self {
        Self::io(e)
    }
}

impl KeigaError {
    /// 呼び出し元のファイル名と行番号
    /// * `return` - 呼び出し元のファイル名と行番号
    #[track_caller]
    fn caller() -> (String, u32) {
        let location = std::panic::Location::caller();
        (location.file().to_string(), location.line())
    }

    /// ファイルが見つからない
    /// * `message` - エラーメッセージ
    /// * `path` - ファイルパス
    /// * `return` - ファイルが見つからないエラー
    #[track_caller]
    pub fn file_not_found(message: impl Into<String>, path: path::PathBuf) -> Self {
        let (file, line) = Self::caller();
        Self::FileNotFound(message.into(), path, file, line)
    }

    /// ファイルエラー
    /// * `message` - エラーメッセージ
    /// * `path` - ファイルパス
    /// * `return` - ファイルエラー
    #[track_caller]
    pub fn file_error(message: impl Into<String>, path: path::PathBuf) -> Self {
        let (file, line) = Self::caller();
        Self::FileError(message.into(), path, file, line)
    }

    /// サポートされていないファイル拡張子
    /// * `path` - ファイルパス
    /// * `return` - サポートされていないファイル拡張子
    #[track_caller]
    pub fn unsupported_extension(path: path::PathBuf) -> Self {
        let (file, line) = Self::caller();
        Self::UnsupportedExtension(path, file, line)
    }

    /// 最適化エラー
    /// * `message` - エラーメッセージ
    /// * `path` - ファイルパス
    /// * `return` - 最適化エラー
    #[track_caller]
    pub fn optimized_error(message: impl Into<String>, path: path::PathBuf) -> Self {
        let (file, line) = Self::caller();
        Self::OptimizedError(message.into(), path, file, line)
    }

    /// ロックがポイズンされた
    /// * `return` - ロックがポイズンされた
    #[track_caller]
    pub fn lock_poisoned() -> Self {
        let (file, line) = Self::caller();
        Self::LockPoisoned(file, line)
    }

    /// バージョンが不正
    /// * `return` - バージョンが不正
    #[track_caller]
    pub fn invalid_version() -> Self {
        let (file, line) = Self::caller();
        Self::InvalidVersion(file, line)
    }

    /// IO エラー
    /// * `error` - IO エラー
    /// * `return` - IO エラー
    #[track_caller]
    pub fn io(error: io::Error) -> Self {
        let (file, line) = Self::caller();
        Self::Io(error, file, line)
    }

    /// 発生箇所のファイル名と行番号
    /// * `return` - ファイル名と行番号
    pub fn location(&self) -> (&str, u32) {
        match self {
            Self::FileNotFound(_, _, file, line)
            | Self::FileError(_, _, file, line)
            | Self::UnsupportedExtension(_, file, line)
            | Self::OptimizedError(_, _, file, line)
            | Self::LockPoisoned(file, line)
            | Self::InvalidVersion(file, line)
            | Self::Io(_, file, line) => (file, *line),
        }
    }

    /// エラーコード
    /// エラー項目・ファイル名・行番号を組み合わせて作る
    /// * `return` - エラーコード
    pub fn code(&self) -> String {
        match self {
            Self::FileNotFound(_, _, f, l) => format!("FNF_{}_{}", Self::file_to_code(f), l),
            Self::FileError(_, _, f, l) => format!("FE_{}_{}", Self::file_to_code(f), l),
            Self::UnsupportedExtension(_, f, l) => format!("UE_{}_{}", Self::file_to_code(f), l),
            Self::OptimizedError(_, _, f, l) => format!("OE_{}_{}", Self::file_to_code(f), l),
            Self::LockPoisoned(f, l) => format!("LP_{}_{}", Self::file_to_code(f), l),
            Self::InvalidVersion(f, l) => format!("IVE_{}_{}", Self::file_to_code(f), l),
            Self::Io(_, f, l) => format!("IO_{}_{}", Self::file_to_code(f), l),
        }
    }

    /// ファイルパスからコードを作る
    /// `src/` 以降を `/` と `_` で区切り、各語の頭文字をつなぐ
    /// 語が1つならその語をそのまま大文字にする。`mod` は除く
    /// * `file` - ファイルパス
    /// * `return` - コード
    fn file_to_code(file: &str) -> String {
        let normalized = file.replace('\\', "/");
        let relative = normalized
            .rsplit_once("src/")
            .map(|(_, rest)| rest)
            .unwrap_or(normalized.as_str());
        let words: Vec<&str> = relative
            .trim_end_matches(".rs")
            .split(['/', '_'])
            .filter(|word| !word.is_empty() && *word != "mod")
            .collect();

        match words.as_slice() {
            [] => "UNKNOWN".to_string(),
            [word] => word.to_uppercase(),
            words => words
                .iter()
                .filter_map(|word| word.chars().next())
                .flat_map(|ch| ch.to_uppercase())
                .collect(),
        }
    }
}

// tests ディレクトリ直下は別クレートになって、非公開関数が呼べないので、
// 子モジュールとして読むためのパスを指定する
#[cfg(test)]
#[path = "../tests/unit/error.rs"]
mod tests;
