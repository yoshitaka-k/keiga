use super::optimize;
use crate::optimize::Optimizer;
use std::path::Path;

#[test]
fn lossy_decodes_png_with_jpg_extension() {
    let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sample/png.jpg"));
    let options = optimize::PngOptions {
        lossy: true,
        dithering: 0,
        options: oxipng::Options::default(),
    };

    let (size, output) = optimize::Png::lossy(path, options).expect("png content with jpg extension");

    assert!(size > 0);
    assert!(output.starts_with(&[0x89, b'P', b'N', b'G']));
}
