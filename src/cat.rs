use crate::{
    char::char,
    digits::digits,
    helper::{parser_closure, Parser},
    string::string, or::or, lexeme::lexeme,
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

pub fn cat_multiple<T>(list: Vec<impl Parser<T>>) -> impl Parser<Vec<T>> {
    parser_closure(move |s| {
        let mut result = vec![];
        let mut input = s;

        for p in &list {
            if let Some((val, rest)) = p(input) {
                result.push(val);
                input = rest;
            } else {
                return None;
            }
        }
        Some((result, input))
    })
}

#[test]
fn test_cat_multiple() {
    let list = vec![char('k'), char('a'), char('j'), char('i')];
    let parser = cat_multiple(list);
    assert_eq!(cat_multiple(vec![char('k'), char('a'), char('j'), char('i')])("kajiibuki"), Some((vec!['k', 'a', 'j', 'i'], "ibuki")));
    assert_eq!(cat_multiple(vec![char('k'), char('a'), char('j'), char('i')])("kaibuki"), None);
    assert_eq!(cat_multiple(vec![char('k'), char('a'), char('j'), char('i')])(""), None);
}

#[macro_export]
macro_rules! cat {
    ($parser0:expr, $($parser:expr),*) => {{
        let p = $parser0;
        $(
            let p = $crate::cat::cat(p, $parser);
        )*
        p
    }};
}

#[test]
fn test_cat_macro() {
    // スペース区切りで数値をちょうど3つ受け付けるパーサー
    let parser = cat![
        lexeme(digits),
        lexeme(digits),
        lexeme(digits)
    ];
    assert_eq!(parser("10 20 30"), Some((((10, 20), 30), "")));
    assert_eq!(parser("10 20 AA"), None);
}