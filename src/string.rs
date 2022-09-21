use crate::{
    helper::{parser_closure, Parser},
    lexeme::lexeme,
};

pub fn string(target: &'static str) -> impl Parser<&str> {
    parser_closure(move |input| input.strip_prefix(target).and_then(|s| Some((target, s))))
}

#[test]
fn test_string() {
    assert_eq!(string("kaji")("kajiibuki"), Some(("kaji", "ibuki")));
    assert_eq!(string("kaji")("kazaiibuki"), None);
    assert_eq!(string("kaji")(""), None);
    assert_eq!(string("kaji")("  kajiibuki"), None);
    assert_eq!(lexeme(string("kaji"))("  kajiibuki"), Some(("kaji", "ibuki")));
}
