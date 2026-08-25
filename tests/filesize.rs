use keiga::filesize_format;

#[test]
fn filesize_format_b() {
    assert_eq!(filesize_format(1), "1.00 B");
    assert_eq!(filesize_format(1023), "1023.00 B");
}

#[test]
fn filesize_format_kb() {
    assert_eq!(filesize_format(1024), "1.00 KB");
    assert_eq!(filesize_format(1023 * 1024), "1023.00 KB");
    assert_eq!(filesize_format(1024 * 1024 - 1), "1024.00 KB");
}

#[test]
fn filesize_format_mb() {
    assert_eq!(filesize_format(1024 * 1024), "1.00 MB");
    assert_eq!(filesize_format(1023 * 1024 * 1024), "1023.00 MB");
    assert_eq!(filesize_format(1024 * 1024 * 1024 - 1), "1024.00 MB");
}

#[test]
fn filesize_format_gb() {
    assert_eq!(filesize_format(1024 * 1024 * 1024), "1.00 GB");
    assert_eq!(filesize_format(1023i64 * 1024 * 1024 * 1024), "1023.00 GB");
    assert_eq!(filesize_format(1024i64 * 1024 * 1024 * 1024 - 1), "1024.00 GB");
}
