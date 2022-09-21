use crate::{
    helper::{parser_closure, Parser},
    lexeme::lexeme,
};

pub fn string(target: &'static str) -> impl Parser<()> {
    parser_closure(move |input| input.strip_prefix(target).map(|after| ((), after)))
}

#[test]
fn test_string() {
    assert_eq!(string("kaji")("kajiibuki"), Some(((), "ibuki")));
    assert_eq!(string("kaji")("kazaiibuki"), None);
    assert_eq!(string("kaji")(""), None);
    assert_eq!(string("kaji")("  kajiibuki"), None);
    assert_eq!(lexeme(string("kaji"))("  kajiibuki"), Some(((), "ibuki")));
}
