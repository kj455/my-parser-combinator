use crate::helper::{parser_closure, Parser};
use regex::Regex;

pub fn regex<'a, T: 'a>(re: &'a Regex, f: impl Fn(&str) -> Option<T> + 'a) -> impl Parser<T> + 'a {
    parser_closure(move |s| {
        re.find(s).and_then(|matched| {
            f(matched.as_str()).map(|value| {
                let rest = &s[matched.end()..];
                (value, rest)
            })
        })
    })
}

#[test]
fn test_regex() {
    let re = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
    let parser = regex(&re, |s| Some(s.to_owned()));
    assert_eq!(parser("x_1=0"), Some(("x_1".to_owned(), "=0")));
    assert_eq!(parser("0_1=0"), None);
}

#[macro_export]
macro_rules! regex {
    ($pattern:expr, $f:expr) => {{
        use once_cell::sync::Lazy;
        use regex::Regex;
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new($pattern).unwrap());
        $crate::regexp::regex(&RE, $f)
    }};
}
