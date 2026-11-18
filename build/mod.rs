pub(crate) mod appicon;
pub(crate) mod fonts;
pub(crate) mod lang;
pub(crate) mod sounds;
pub(crate) mod svg;

use std::{fs, path::Path};

/// 文字列をパスカルケースに変換する
/// * `s` - 文字列
/// * `return` - パスカルケースの文字列
pub(crate) fn to_pascal(s: &str) -> String {
    s.split(|c: char| c == '_' || c == '-' || c.is_whitespace()) // 区切り文字で分割
     .filter(|word| !word.is_empty())
     .map(|word| {
         let mut chars = word.chars();
         match chars.next() {
             None => String::new(),
             Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
         }
     })
     .collect()
}

/// JSON ファイルを読み込んで serde_json::Value に変換する
/// * `path` - JSON ファイルのパス
/// * `file` - JSON ファイルのファイル名
/// * `return` - serde_json::Value
pub(crate) fn load_json(path: &Path, file: &str) -> serde_json::Value {
    let json_file = path.join(file);

    let json_string = &fs::read_to_string(&json_file)
        .unwrap_or_else(|_| panic!("failed to read {}", json_file.display()));

    serde_json::from_str(json_string)
        .unwrap_or_else(|_| panic!("{}: must be a valid JSON", file))
}

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
