/// ファイル情報を管理する構造体
#[derive(Clone, Copy)]
pub(crate) struct FileInfo {
    pub standby_len: u32,
    pub optimizing_len: OptimizingLen,
    pub optimized_len: OptimizedLen,
    pub unchanged_len: u32,
    pub skipped_len: u32,
    pub canceled_len: u32,
    pub error_len: u32,

    pub total_size: SizeInfo,
    pub total_new_size: NewSizeInfo,
    pub total_duration: DurationInfo,
}
impl FileInfo {
    pub fn new() -> Self {
        Self {
            standby_len: 0,
            optimizing_len: OptimizingLen::new(),
            optimized_len: OptimizedLen::new(),
            unchanged_len: 0,
            skipped_len: 0,
            canceled_len: 0,
            error_len: 0,
            total_size: SizeInfo::new(),
            total_new_size: NewSizeInfo::new(),
            total_duration: DurationInfo::new(),
        }
    }
}

/// 最適化中のファイル数を管理する構造体
#[derive(Clone, Copy)]
pub(crate) struct OptimizingLen {
    pub jpeg: u32,
    pub png: u32,
}
impl OptimizingLen {
    pub fn new() -> Self {
        Self { jpeg: 0, png: 0 }
    }
    pub fn jpeg(&self) -> u32 {
        self.jpeg
    }
    pub fn png(&self) -> u32 {
        self.png
    }
    pub fn total(&self) -> u32 {
        self.jpeg + self.png
    }
}

/// 最適化済みのファイル数を管理する構造体
#[derive(Clone, Copy)]
pub(crate) struct OptimizedLen {
    pub jpeg: u32,
    pub png: u32,
}
impl OptimizedLen {
    pub fn new() -> Self {
        Self { jpeg: 0, png: 0 }
    }
    pub fn total(&self) -> u32 {
        self.jpeg + self.png
    }
}

/// 最適化時間を管理する構造体
#[derive(Clone, Copy)]
pub(crate) struct DurationInfo {
    pub jpeg: u64,
    pub png: u64,
}
impl DurationInfo {
    pub fn new() -> Self {
        Self { jpeg: 0, png: 0 }
    }
    pub fn total(&self) -> u64 {
        self.jpeg + self.png
    }
}

/// ファイルサイズを管理する構造体
#[derive(Clone, Copy)]
pub(crate) struct SizeInfo {
    pub jpeg: u64,
    pub png: u64,
}
impl SizeInfo {
    pub fn new() -> Self {
        Self { jpeg: 0, png: 0 }
    }
    pub fn total(&self) -> u64 {
        self.jpeg + self.png
    }
}

/// 最適化後のファイルサイズを管理する構造体
#[derive(Clone, Copy)]
pub(crate) struct NewSizeInfo {
    pub jpeg: u64,
    pub png: u64,
}
impl NewSizeInfo {
    pub fn new() -> Self {
        Self { jpeg: 0, png: 0 }
    }
    pub fn total(&self) -> u64 {
        self.jpeg + self.png
    }
}
