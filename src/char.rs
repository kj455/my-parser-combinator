use crate::helper::{parser_closure, Parser};

pub fn char(c: char) -> impl Parser<char> {
    parser_closure(move |s| {
        if s.chars().next() == Some(c) {
            Some((c, &s[1..]))
        } else {
            None
        }
    })
}

#[test]
fn parse() {
    assert_eq!(char('a')("abc"), Some(('a', "bc")));
    assert_eq!(char('a')("aabc"), Some(('a', "abc")));
    assert_eq!(char('a')("bcd"), None);
    assert_eq!(char('a')(""), None);
}
