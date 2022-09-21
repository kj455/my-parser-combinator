pub fn digits(s: &str) -> Option<(u64, &str)> {
  let end = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
    match s[..end].parse() {
        Ok(value) => Some((value, &s[end..])),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        assert_eq!(super::digits("825kaji"), Some((825, "kaji")));
        assert_eq!(super::digits("0825kaji"), Some((825, "kaji")));
        assert_eq!(super::digits("0000kaji"), Some((00, "kaji")));
        assert_eq!(super::digits("kaji"), None);
        assert_eq!(super::digits("=kaji"), None);
    }
}