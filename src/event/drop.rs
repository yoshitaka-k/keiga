use crate::{app, error, file};
use eframe::egui::DroppedFileHandle;

/// ドロップされたファイルを処理
/// * `dropped_files` - ドロップされたファイル
/// * `app` - アプリケーション
/// * `files` - 開いているファイル
pub(crate) fn drop_files(
    dropped_files: &[DroppedFileHandle],
    app: &app::App,
    files: &mut file::OpenFiles,
) -> error::Result<()> {
    if dropped_files.is_empty() {
        return Ok(());
    }

    for file in dropped_files {
        files.add_path(app, file.path().to_path_buf())?;
    }

    Ok(())
}
