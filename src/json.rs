use std::collections::HashMap;

use crate::{
    cat, char::char, helper::Parser, lexeme::lexeme, map::map, or, regex, rep::rep, split::split,
    string::string,
};

#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    False,
    True,
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

fn lex_string(target: &'static str) -> impl Parser<&'static str> {
    lexeme(string(target))
}

fn lex_char(c: char) -> impl Parser<char> {
    lexeme(char(c))
}

fn null(s: &str) -> Option<(Value, &str)> {
    map(lex_string("null"), |_| Value::Null)(s)
}

fn false_(s: &str) -> Option<(Value, &str)> {
    map(lex_string("false"), |_| Value::False)(s)
}

fn true_(s: &str) -> Option<(Value, &str)> {
    map(lex_string("true"), |_| Value::True)(s)
}

fn number(s: &str) -> Option<(Value, &str)> {
    const PATTERN: &str = r"^-?(0|[1-9][0-9]*)(\.[0-9]+)?([eE][+-]?[0-9]+)?";
    map(lexeme(regex!(PATTERN, |s| s.parse::<f64>().ok())), |x| {
        Value::Number(x)
    })(s)
}

fn json_string(s: &str) -> Option<(Value, &str)> {
    map(json_string_raw, Value::String)(s)
}

fn json_string_raw(s: &str) -> Option<(String, &str)> {
    // string = '"' character* '"'
    let p = crate::cat![char('"'), rep(json_character), char('"')];
    let p = lexeme(p);
    let p = map(p, |((_, chars), _)| chars.into_iter().collect());
    p(s)
}

fn json_character(s: &str) -> Option<(char, &str)> {
    // character = <Any codepoint except " or \ or control characters>
    //           | '\u' <4 hex digits>
    //           | '\"' | '\\' | '\/' | '\b' | '\f' | '\n' | '\r' | '\t'
    crate::or![
        crate::regex!(r#"^[^"\\[:cntrl:]]"#, |s| s.chars().next()),
        crate::regex!(r#"^\\u[0-9a-fA-F]{4}"#, hex_code),
        crate::regex!(r#"^\\."#, escape)
    ](s)
}

fn hex_code(code: &str) -> Option<char> {
    code.strip_prefix(r"\u").and_then(|hex| {
        u32::from_str_radix(hex, 16)
            .ok()
            .and_then(|cp| char::from_u32(cp))
    })
}

fn escape(s: &str) -> Option<char> {
    match s {
        "\\\"" => Some('"'),
        "\\\\" => Some('\\'),
        "\\/" => Some('/'),
        "\\b" => Some('\x08'),
        "\\f" => Some('\x0C'),
        "\\n" => Some('\n'),
        "\\r" => Some('\r'),
        "\\t" => Some('\t'),
        _ => None, // undefined escape sequence
    }
}

fn array(s: &str) -> Option<(Value, &str)> {
    map(
        cat!(
            lex_char('['),
            split(json_value, lex_char(',')),
            lex_char(']')
        ),
        |((_, val), _)| Value::Array(val),
    )(s)
}

fn object(s: &str) -> Option<(Value, &str)> {
    map(
        cat!(
            lex_char('{'),
            split(key_value, lex_char(',')),
            lex_char('}')
        ),
        |((_, entry), _)| {
            let hash_map = HashMap::from_iter(entry.into_iter());
            Value::Object(hash_map)
        },
    )(s)
}

fn key_value(s: &str) -> Option<((String, Value), &str)> {
    map(
        cat!(json_string_raw, lex_char(':'), json_value),
        |((key, _), val)| (key, val),
    )(s)
}

fn json_value(s: &str) -> Option<(Value, &str)> {
    or!(null, false_, true_, number, json_string, array, object)(s)
}

pub fn parse_json(s: &str) -> Option<Value> {
    json_value(s).and_then(|(val, rest)| {
        if rest.chars().all(|c| c.is_ascii_whitespace()) {
            Some(val)
        } else {
            None
        }
    })
}

#[test]
fn test_json() {
    assert_eq!(lex_string("ka")("  kaji"), Some(("ka", "ji")));
    assert_eq!(lex_string("ka")("  "), None);

    assert_eq!(lex_char('k')("  ka"), Some(('k', "a")));
    assert_eq!(lex_char('k')("  "), None);

    assert_eq!(null("nullable"), Some((Value::Null, "able")));
    assert_eq!(null("able"), None);

    assert_eq!(false_("falsetrue"), Some((Value::False, "true")));
    assert_eq!(false_("true"), None);

    assert_eq!(true_("truefalse"), Some((Value::True, "false")));
    assert_eq!(true_("false"), None);

    assert_eq!(
        number("123.456hoge"),
        Some((Value::Number(123.456), "hoge"))
    );
    assert_eq!(number("10e4hoge"), Some((Value::Number(10e4), "hoge")));
    assert_eq!(number("aaa"), None);

    assert_eq!(
        json_string(r#""foo""#),
        Some((Value::String("foo".to_string()), ""))
    );
    assert_eq!(json_string(r#"foo"#), None);
    assert_eq!(
        json_string(r#""825""#),
        Some((Value::String("825".to_string()), ""))
    );
    assert_eq!(
        json_string(r#""/foo""#),
        Some((Value::String("/foo".to_string()), ""))
    );

    assert_eq!(
        array(r#"[null, true, false, 100, "foo"]"#),
        Some((
            Value::Array(vec![
                Value::Null,
                Value::True,
                Value::False,
                Value::Number(100.0),
                Value::String("foo".to_string())
            ]),
            ""
        ))
    );

    assert_eq!(
        object(r#"{ "foo": "bar", "hoge": { "qux": null } }"#),
        Some((
            Value::Object(HashMap::from([
                ("foo".to_string(), Value::String("bar".to_string())),
                (
                    "hoge".to_string(),
                    Value::Object(HashMap::from([("qux".to_string(), Value::Null)]))
                )
            ])),
            ""
        ))
    );

    assert_eq!(
        parse_json(r#"{ "foo": "bar", "hoge": { "qux": null } }"#),
        Some(
            Value::Object(HashMap::from([
                ("foo".to_string(), Value::String("bar".to_string())),
                (
                    "hoge".to_string(),
                    Value::Object(HashMap::from([("qux".to_string(), Value::Null)]))
                )
            ]),
        ))
    );
}
