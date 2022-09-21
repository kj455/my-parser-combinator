use crate::{
    char::char,
    digits::digits,
    helper::{parser_closure, Parser},
    string::string, or::or,
};

pub fn cat<T, U>(p1: impl Parser<T>, p2: impl Parser<U>) -> impl Parser<(T, U)> {
    parser_closure(move |s| {
        p1(s).and_then(|(val1, rest1)| p2(rest1).and_then(|(val2, rest2)| Some(((val1, val2), rest2))))
    })
}

#[test]
fn test_cat() {
    assert_eq!(cat(char('-'), digits)("-123abc"), Some((('-', 123), "abc")));
    assert_eq!(cat(char('-'), digits)("123abc"), None);
    assert_eq!(
        cat(digits, string("null"))("123null"),
        Some(((123, "null"), ""))
    );

    assert_eq!(cat(or(char('-'), char('+')), digits)("-123abc"), Some((('-', 123), "abc")));
    assert_eq!(cat(or(char('-'), char('+')), digits)("+123abc"), Some((('+', 123), "abc")));
}
