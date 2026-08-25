#[path = "build/mod.rs"]
pub(crate) mod build;

use std::{env, path::Path};

/// メイン関数
fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_path = Path::new(&manifest_dir);
    let out_dir = env::var("OUT_DIR").unwrap();

    // フォントのディレクトリを取得
    let fonts_dir = manifest_path.join("assets/fonts");
    println!("cargo:rerun-if-changed={}", fonts_dir.display());
    build::fonts::generate_fonts_generated(&fonts_dir, &out_dir);

    // SVGのディレクトリを取得
    let svg_dir = manifest_path.join("assets/svg");
    println!("cargo:rerun-if-changed={}", svg_dir.display());
    build::svg::generate_svg_generated(&svg_dir, &out_dir);

    // 効果音のディレクトリを取得
    let sounds_dir = manifest_path.join("assets/sounds");
    println!("cargo:rerun-if-changed={}", sounds_dir.display());
    build::sounds::generate_sounds_generated(&sounds_dir, &out_dir);

    // バンドル用アイコンと Windows の実行ファイルアイコンを生成する
    build::appicon::generate_icons(manifest_path, &out_dir);
}
