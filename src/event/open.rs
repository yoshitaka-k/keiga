use crate::{config, error, file};

/// ファイルオープンダイアログを開いて選択結果を追加する
/// * `config` - 設定
/// * `files` - 開いているファイル
/// * `label` - ファイルダイアログのラベル
pub(crate) fn file(
    config: &config::Config,
    files: &mut file::OpenFiles,
    label: &str,
) -> error::Result<()> {
    let extensions = file::allowed_extensions();

    let paths = rfd::FileDialog::new()
        .add_filter(label, extensions)
        .pick_files();

    // ファイルを追加
    if let Some(paths) = paths {
        for path in paths {
            files.add_path(config, path)?;
        }
    }

    Ok(())
}

/// ファイルオープンダイアログを開いて選択結果を追加する
/// * `config` - 設定
/// * `files` - 開いているファイル
/// * `label` - ファイルダイアログのラベル
pub(crate) fn folder(
    config: &config::Config,
    files: &mut file::OpenFiles,
    label: &str,
) -> error::Result<()> {
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
        for path in paths {
            files.add_path(config, path)?;
        }
    }

    Ok(())
}

/// ドロップされたファイルを処理
/// * `dropped_files` - ドロップされたファイル
/// * `config` - 設定
/// * `files` - 開いているファイル
pub(crate) fn drop(
    ctx: &egui::Context,
    dropped_files: &Vec<std::path::PathBuf>,
    config: &config::Config,
    files: &mut file::OpenFiles,
) -> error::Result<()> {
    for file in dropped_files {
        files.add_path(config, file.to_path_buf())?;
    }

    // ウィンドウを前面に持ってくる
    ctx.send_viewport_cmd_to(
        egui::ViewportId::ROOT,
        egui::ViewportCommand::Focus,
    );

    Ok(())
}
