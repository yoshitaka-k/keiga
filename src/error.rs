use std::{path, error, fmt, io};

pub type Result<T> = std::result::Result<T, KeigaError>;

#[derive(Debug)]
pub enum KeigaError {
    FileNotFound(path::PathBuf),
    FileError(String, path::PathBuf),
    UnsupportedExtension(path::PathBuf),
    OptimizedError(String, path::PathBuf),
    LockPoisoned,
    InvalidVersion,
    Io(io::Error),
}

/// KeigaError を表示
impl fmt::Display for KeigaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeigaError::FileNotFound(path) => write!(f, "File does not exist: {}", path.display()),
            KeigaError::FileError(e, path) => write!(f, "File error: {} \n\n{}", e, path.display()),
            KeigaError::UnsupportedExtension(path) => write!(f, "Unsupported extension: {}", path.display()),
            KeigaError::OptimizedError(e, path) => write!(f, "Optimized error: {} \n\n{}", e, path.display()),
            KeigaError::LockPoisoned => write!(f, "Lock poisoned"),
            KeigaError::InvalidVersion => write!(f, "Invalid version"),
            KeigaError::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

/// KeigaError を表示
impl error::Error for KeigaError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            KeigaError::FileNotFound(_) => None,
            KeigaError::FileError(_, _) => None,
            KeigaError::UnsupportedExtension(_) => None,
            KeigaError::OptimizedError(_, _) => None,
            KeigaError::LockPoisoned => None,
            KeigaError::InvalidVersion => None,
            KeigaError::Io(ref e) => Some(e),
        }
    }
}

/// std::io::Error を KeigaError に変換
impl From<io::Error> for KeigaError {
    fn from(e: io::Error) -> Self {
        KeigaError::Io(e)
    }
}
