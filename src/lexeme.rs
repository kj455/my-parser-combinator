use crate::{
    char::char,
    digits::digits,
    helper::{parser_closure, Parser},
};

pub fn lexeme<T>(parser: impl Parser<T>) -> impl Parser<T> {
    parser_closure(move |s| parser(s.trim_start()))
}

#[test]
fn parse() {
    assert_eq!(lexeme(char('a'))("   abc"), Some(('a', "bc")));
    assert_eq!(lexeme(char('a'))("abc"), Some(('a', "bc")));
    assert_eq!(lexeme(char('a'))("  bcd"), None);
    assert_eq!(lexeme(digits)("123abc"), Some((123, "abc")));
    assert_eq!(lexeme(digits)("   123abc"), Some((123, "abc")));
    assert_eq!(lexeme(digits)("   abc"), None);
}
