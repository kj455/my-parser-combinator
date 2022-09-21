use crate::{
    digits::digits,
    helper::{parser_closure, Parser},
    map::map,
    string::string,
};

pub fn or<T>(p1: impl Parser<T>, p2: impl Parser<T>) -> impl Parser<T> {
    parser_closure(move |s| p1(s).or_else(|| p2(s)))
}

#[test]
fn test_or() {
    assert_eq!(
        or(digits, map(string("null"), |_| 0))("123abc"),
        Some((123, "abc"))
    );
    assert_eq!(
        or(digits, map(string("null"), |_| 0))("nullabc"),
        Some((0, "abc"))
    );
    assert_eq!(or(digits, map(string("null"), |_| 0))("abc"), None);
}

#[macro_export]
macro_rules! or {
    ($parser0:expr, $($parser:expr),*) => {{
        let p = $parser0;
        $(
            let p = $crate::or::or(p, $parser);
        )*
        p
    }};
}

#[test]
fn test_or_macro() {
    let parser = or![
        map(string("zero"), |_| 0),
        map(string("one"), |_| 1),
        digits
    ];
    assert_eq!(parser("zero"), Some((0, "")));
    assert_eq!(parser("one"), Some((1, "")));
    assert_eq!(parser("42"), Some((42, "")));
    assert_eq!(parser("hoge"), None);
}
