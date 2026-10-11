use super::KeigaErrorError;

#[test]
fn index_error_uses_call_site() {
    let err = KeigaError::index_error(7);
    let here = line!();
    let (file, line) = err.location();
    assert!(matches!(err, KeigaError::IndexError(7, _, _)));
    assert!(file.ends_with("error.rs"), "{file}");
    assert_eq!(line, here - 1);
}

#[test]
fn file_to_code_uses_initials() {
    assert_eq!(KeigaError::file_to_code("src/lib.rs"), "LIB");
    assert_eq!(KeigaError::file_to_code("src/file/open_file.rs"), "FOF");
    assert_eq!(KeigaError::file_to_code("src/file/book.rs"), "FB");
    assert_eq!(KeigaError::file_to_code("src/model/book.rs"), "MB");
    assert_eq!(KeigaError::file_to_code("src/ui/mod.rs"), "UI");
    assert_eq!(KeigaError::file_to_code("src/ui/modal.rs"), "UM");
    assert_eq!(KeigaError::file_to_code("src/ui/main/middle.rs"), "UMM");
    assert_eq!(KeigaError::file_to_code(r"C:\work\resizer\src\file\book.rs"), "FB");
}
