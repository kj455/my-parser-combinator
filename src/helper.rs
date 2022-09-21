pub trait Parser<T>: Fn(&str) -> Option<(T, &str)> {}
impl<T, F> Parser<T> for F where F: Fn(&str) -> Option<(T, &str)> {}

// クロージャの型推論を補助するための関数
// cf. https://github.com/rust-lang/rust/issues/70263#issuecomment-623169045
pub fn parser_closure<T, F>(f: F) -> impl Parser<T>
where F: Fn(&str) -> Option<(T, &str)> {
    f
}