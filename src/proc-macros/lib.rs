use proc_macro::*;

/// Compile time pattern parser.
///
/// ```ignore
/// const PATTERN: &[pelite::pattern::Atom] = pattern!("pattern string");
/// ```
#[proc_macro]
pub fn pattern(input: TokenStream) -> TokenStream {
    let input = input.into_iter().collect::<Vec<_>>();

    let string = match &input[..] {
        [TokenTree::Literal(lit)] => parse_str_literal(&lit),
        _ => panic!("expected a single string literal to parse"),
    };

    let pattern = match pattern::parse(&string) {
        Ok(pattern) => pattern,
        Err(err) => panic!("invalid pattern syntax: {}", err),
    };

    format!("{{ use ::pelite::pattern::Atom::*; &{:?} }}", pattern)
        .parse()
        .unwrap()
}

fn parse_str_literal(input: &Literal) -> String {
    let input = input.to_string();
    let mut chars = input.chars();
    let mut string = String::new();
    if chars.next() != Some('"') {
        panic!("expected string literal starting with a `\"` and no extraneous whitespace");
    }
    loop {
        let chr = match chars.next() {
            Some('\\') => match chars.next() {
                Some('\\') => '\\',
                Some('\'') => '\'',
                Some('\"') => '\"',
                Some('t') => '\t',
                Some('r') => '\r',
                Some('n') => '\n',
                Some('u') => panic!("unicode escape sequence not supported"),
                Some(chr) => panic!("unknown escape sequence: {}", chr),
                None => panic!(""),
            },
            Some('"') => break,
            Some(chr) => chr,
            None => panic!("unexpected end of string literal, missing `\"` terminator?"),
        };
        string.push(chr);
    }
    string
}

// Total hack to get the pattern parse code in here :)
#[allow(unused)]
mod pattern;
