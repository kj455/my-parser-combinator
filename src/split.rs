use std::vec;

use crate::{
    char::char,
    digits::digits,
    helper::{parser_closure, Parser},
    lexeme::lexeme,
};

pub fn split<T, U>(parser: impl Parser<T>, separator: impl Parser<U>) -> impl Parser<Vec<T>> {
    parser_closure(move |s| {
        let mut result: Vec<T> = vec![];
        let mut input = s;

        loop {
            if let Some((val, rest)) = parser(input) {
                result.push(val);
                input = rest;
            } else {
                break;
            }
            if let Some((_, rest)) = separator(input) {
                input = rest;
            }
        }
        Some((result, input))
    })
}

#[test]
fn test_split() {
    assert_eq!(
        split(digits, char(','))("10,20,30"),
        Some((vec![10, 20, 30], ""))
    );
    assert_eq!(split(digits, char(','))(""), Some((vec![], "")));
    assert_eq!(
        split(lexeme(digits), lexeme(char(',')))("10, 20, 30"),
        Some((vec![10, 20, 30], ""))
    );
    assert_eq!(
        split(lexeme(digits), lexeme(char(',')))("10, 20, 30,"),
        Some((vec![10, 20, 30], ""))
    );
}
