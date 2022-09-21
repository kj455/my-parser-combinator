use crate::{helper::{Parser, parser_closure}, digits::digits, char::char, string::string};

pub fn cat<T, U>(p1: impl Parser<T>, p2: impl Parser<U>) -> impl Parser<(T, U)> {
  parser_closure(move |s| {
    if let Some(v) = p1(s) {
      if let Some(v2) = p2(v.1) {
        return Some(((v.0, v2.0), v2.1));
      }
    }
    None
  })
}

#[test]
fn test_cat() {
  assert_eq!(cat(char('-'), digits)("-123abc"), Some((('-', 123), "abc")));
  assert_eq!(cat(char('-'), digits)("123abc"), None);
  assert_eq!(cat(digits, string("null"))("123null"), Some(((123, "null"), "")));
}