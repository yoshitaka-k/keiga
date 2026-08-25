pub(crate) mod fonts;
pub(crate) mod appicon;
pub(crate) mod svg;
pub(crate) mod sounds;

use std::{fs, path::Path};

/// assets 配下のパスを include_bytes! 用に展開する
/// * `path` - assets/ から始まるパス
/// * `return` - include_bytes!(concat!(...)) に渡す文字列
pub(crate) fn include_assets_path(path: &str) -> String {
    if path.starts_with("assets/") {
        format!("concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/{}\")", path)
    } else {
        panic!("assets path must start with 'assets/' (was: {})", path);
    }
}

/// 識別子として使える名前に変換
/// 空白や記号を『 _ 』に変換して大文字にする
/// * `font_name` - フォント名
/// * `return` - 識別子として使える名前
pub(crate) fn to_const_name(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect::<String>()
        .to_uppercase()
}

/// ディレクトリ内の指定拡張子ファイルの stem を収集してソートする
/// * `dir` - ディレクトリのパス
/// * `extension` - 拡張子
/// * `return` - ファイルの stem のベクター
pub(crate) fn collect_asset_stems(dir: &Path, extension: &str) -> Vec<String> {
    let mut names = Vec::new();

    for entry in fs::read_dir(dir).unwrap_or_else(|_| panic!("failed to read {}", dir.display())) {
        let path = entry.expect("failed to read directory entry").path();
        if path.extension().and_then(|s| s.to_str()) != Some(extension) {
            continue;
        }
        let name = path.file_stem()
                       .and_then(|s| s.to_str())
                       .expect("invalid asset filename")
                       .to_string();
        names.push(name);
    }

    names.sort();

    names
}
