use std::{path, error, fmt, io};

pub type Result<T> = std::result::Result<T, KeigaError>;

#[derive(Debug)]
pub enum KeigaError {
    FileNotFound(path::PathBuf),
    Io(io::Error),
    LockPoisoned,
    FileError(String, path::PathBuf),
}

/// KeigaError を表示
impl fmt::Display for KeigaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeigaError::FileNotFound(path) => write!(f, "File does not exist: {}", path.display()),
            KeigaError::Io(e) => write!(f, "IO error: {}", e),
            KeigaError::LockPoisoned => write!(f, "Lock poisoned"),
            KeigaError::FileError(e, path) => write!(f, "File error: {} \n\n{}", e, path.display()),
        }
    }
}

/// KeigaError を表示
impl error::Error for KeigaError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            KeigaError::FileNotFound(_) => None,
            KeigaError::Io(ref e) => Some(e),
            KeigaError::LockPoisoned => None,
            KeigaError::FileError(_, _) => None,
        }
    }
}

/// std::io::Error を KeigaError に変換
impl From<io::Error> for KeigaError {
    fn from(e: io::Error) -> Self {
        KeigaError::Io(e)
    }
}
