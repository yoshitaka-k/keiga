use crate::file;

/// ファイルオープンダイアログを開いて選択結果を追加する
/// * `extensions` - 許可する拡張子
/// * `files` - 開いているファイル
pub(crate) fn open_files(
    extensions: &Vec<String>,
    files: &mut file::OpenFiles,
) -> Result<(), Box<dyn std::error::Error>> {
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
            files.add_path(path)?;
        }
    }

    Ok(())
}
