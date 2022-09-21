use crate::helper::{parser_closure, Parser};

pub fn string(target: &'static str) -> impl Parser<()> {
    parser_closure(move |input| {
        input.strip_prefix(target).map(|after| ((), after))
    })
}

#[cfg(test)]
mod tests {
    use crate::{string::string, lexeme::{lexeme}};

    #[test]
    fn parse() {
        assert_eq!(string("kaji")("kajiibuki"), Some(((), "ibuki")));
        assert_eq!(string("kaji")("kazaiibuki"), None);
        assert_eq!(string("kaji")(""), None);
        assert_eq!(string("kaji")("  kajiibuki"), None);
        assert_eq!(lexeme(string("kaji"))("  kajiibuki"), Some(((), "ibuki")));
    }
}