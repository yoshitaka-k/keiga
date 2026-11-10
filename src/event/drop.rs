use crate::{config, error, file};
use eframe::egui::DroppedFileHandle;

/// ドロップされたファイルを処理
/// * `dropped_files` - ドロップされたファイル
/// * `config` - 設定
/// * `files` - 開いているファイル
pub(crate) fn drop_files(
    dropped_files: &[DroppedFileHandle],
    config: &config::Config,
    files: &mut file::OpenFiles,
) -> error::Result<()> {
    if dropped_files.is_empty() {
        return Ok(());
    }

    for file in dropped_files {
        files.add_path(config, file.path().to_path_buf())?;
    }

    Ok(())
}
