use crate::helper::{self, parser_closure, Parser};

pub fn lexeme<T>(parser: impl Parser<T>) -> impl Parser<T> {
    parser_closure(move |s| parser(s.trim_start()))
}

#[cfg(test)]
mod tests {
    use crate::{char::char, digits::digits};

    #[test]
    fn parse() {
        assert_eq!(super::lexeme(char('a'))("   abc"), Some(((), "bc")));
        assert_eq!(super::lexeme(char('a'))("abc"), Some(((), "bc")));
        assert_eq!(super::lexeme(char('a'))("  bcd"), None);
        assert_eq!(super::lexeme(digits)("123abc"), Some((123, "abc")));
        assert_eq!(super::lexeme(digits)("   123abc"), Some((123, "abc")));
        assert_eq!(super::lexeme(digits)("   abc"), None);
    }
}
