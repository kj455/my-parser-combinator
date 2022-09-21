use crate::{
    digits::digits,
    helper::{parser_closure, Parser},
};

pub fn map<A, B>(parser: impl Parser<A>, f: impl Fn(A) -> B) -> impl Parser<B> {
    parser_closure(move |str| parser(str).map(|(output, rest)| (f(output), rest)))
}

#[test]
fn parse() {
    let parser = map(digits, |val| val * 2);
    assert_eq!(parser("123abc"), Some((246, "abc")));
    assert_eq!(parser("abc"), None);
}
