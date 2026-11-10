use crate::{config, error, file};

/// ファイルオープンダイアログを開いて選択結果を追加する
/// * `config` - 設定
/// * `files` - 開いているファイル
pub(crate) fn folder(
    config: &config::Config,
    files: &mut file::OpenFiles,
) -> error::Result<()> {
    let extensions = &config.extensions_to_string();

    // Macのみファイルとフォルダを同時選択できる
    #[cfg(target_os = "macos")]
    let paths = rfd::FileDialog::new()
        .add_filter("Images", extensions)
        .pick_files_or_folders();

    // Mac以外は複数フォルダ選択のみ
    #[cfg(not(target_os = "macos"))]
    let paths = rfd::FileDialog::new()
        .add_filter("Images", extensions)
        .pick_folders();

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
pub(crate) fn file(
    config: &config::Config,
    files: &mut file::OpenFiles,
) -> error::Result<()> {
    let extensions = &config.extensions_to_string();

    let paths = rfd::FileDialog::new()
        .add_filter("Images", extensions)
        .pick_files();

    // ファイルを追加
    if let Some(paths) = paths {
        for path in paths {
            files.add_path(config, path)?;
        }
    }

    Ok(())
}
