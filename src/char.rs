use crate::helper::{parser_closure, Parser};

pub fn char(c: char) -> impl Parser<()> {
    parser_closure(move |s| {
        if s.chars().next() == Some(c) {
            Some(((), &s[1..]))
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        assert_eq!(super::char('a')("abc"), Some(((), "bc")));
        assert_eq!(super::char('a')("aabc"), Some(((), "abc")));
        assert_eq!(super::char('a')("bcd"), None);
        assert_eq!(super::char('a')(""), None);
    }
}
