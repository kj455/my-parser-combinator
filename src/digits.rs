pub fn digits(s: &str) -> Option<(u64, &str)> {
    let end = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
    match s[..end].parse() {
        Ok(value) => Some((value, &s[end..])),
        Err(_) => None,
    }
}

#[test]
fn parse() {
    assert_eq!(digits("825kaji"), Some((825, "kaji")));
    assert_eq!(digits("0825kaji"), Some((825, "kaji")));
    assert_eq!(digits("0000kaji"), Some((00, "kaji")));
    assert_eq!(digits("kaji"), None);
    assert_eq!(digits("=kaji"), None);
}
