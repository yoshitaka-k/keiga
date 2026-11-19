use std::path::PathBuf;
use crate::file;

/// ファイルオープンダイアログを開いて選択結果を追加する
/// * `scan_job` - ファイルスキャンジョブ
/// * `label` - ファイルダイアログのラベル
pub(crate) fn file(
    scan_job: &file::ScanJob,
    label: &str,
) {
    let extensions = file::allowed_extensions();

    let paths = rfd::FileDialog::new()
        .add_filter(label, extensions)
        .pick_files();

    // ファイルを追加
    if let Some(paths) = paths {
        open_paths(scan_job, paths);
    }
}

/// ファイルオープンダイアログを開いて選択結果を追加する
/// * `scan_job` - ファイルスキャンジョブ
/// * `label` - ファイルダイアログのラベル
pub(crate) fn folder(
    scan_job: &file::ScanJob,
    label: &str,
) {
    let extensions = file::allowed_extensions();

    // Macのみファイルとフォルダを同時選択できる
    #[cfg(target_os = "macos")]
    let paths = rfd::FileDialog::new()
        .add_filter(label, extensions)
        .pick_files_or_folders();

    // Mac以外は複数フォルダ選択のみ
    #[cfg(not(target_os = "macos"))]
    let paths = rfd::FileDialog::new()
        .add_filter(label, extensions)
        .pick_folders();

    // ファイルを追加
    if let Some(paths) = paths {
        open_paths(scan_job, paths);
    }
}

/// ドロップされたファイルを処理
/// * `ctx` - コンテキスト
/// * `scan_job` - ファイルスキャンジョブ
/// * `dropped_files` - ドロップされたファイル
pub(crate) fn drop(
    ctx: &egui::Context,
    scan_job: &file::ScanJob,
    dropped_files: Vec<PathBuf>,
) {
    open_paths(scan_job, dropped_files);

    // ウィンドウを前面に持ってくる
    ctx.send_viewport_cmd_to(
        egui::ViewportId::ROOT,
        egui::ViewportCommand::Focus,
    );
}

/// ファイルを追加
/// * `scan_job` - ファイルスキャンジョブ
/// * `paths` - 追加するファイルのパス
fn open_paths(
    scan_job: &file::ScanJob,
    paths: Vec<PathBuf>,
) {
    scan_job.run(paths);
}
