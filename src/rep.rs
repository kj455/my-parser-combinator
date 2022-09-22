use crate::{
    char::char,
    digits::digits,
    helper::{parser_closure, Parser},
    lexeme::lexeme,
    or::or,
};

pub fn rep<T>(parser: impl Parser<T>) -> impl Parser<Vec<T>> {
    parser_closure(move |s| {
        let mut result: Vec<T> = vec![];
        let mut input = s;

        while let Some((val, rest)) = parser(input) {
            result.push(val);
            input = rest;
        }

        Some((result, input))
    })
}

#[test]
fn test_rep() {
    assert_eq!(rep(char('a'))("aaabc"), Some((vec!['a', 'a', 'a'], "bc")));
    assert_eq!(rep(char('a'))("bcd"), Some((vec![], "bcd")));
    assert_eq!(
        rep(or(char('+'), char('-')))("+-+-+-abc"),
        Some((vec!['+', '-', '+', '-', '+', '-'], "abc"))
    );
    assert_eq!(
        rep(lexeme(digits))("10 20 30"),
        Some((vec![10, 20, 30], ""))
    );
}
