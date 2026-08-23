use keiga::duration_format;

#[test]
fn duration_format_ms() {
    assert_eq!(duration_format!(1), "1.00 ms");
    assert_eq!(duration_format!(1000 - 1), "999.00 ms");
}

#[test]
fn duration_format_s() {
    assert_eq!(duration_format!(1000), "1.00 s");
    assert_eq!(duration_format!(59 * 1000), "59.00 s");
    assert_eq!(duration_format!(60 * 1000 - 1), "60.00 s");
}

#[test]
fn duration_format_m() {
    assert_eq!(duration_format!(60 * 1000), "1.00 m");
    assert_eq!(duration_format!(59 * 60 * 1000), "59.00 m");
    assert_eq!(duration_format!(60 * 60 * 1000 - 1), "60.00 m");
}

#[test]
fn duration_format_h() {
    assert_eq!(duration_format!(60 * 60 * 1000), "1.00 h");
    assert_eq!(duration_format!(60 * 60 * 12 * 1000), "12.00 h");
    assert_eq!(duration_format!(60 * 60 * 48 * 1000), "48.00 h");
}
