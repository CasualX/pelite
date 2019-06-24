extern crate proc_macro;

use proc_macro::*;

/// Auto derive the `Pod` trait for structs.
///
/// The type is checked for requirements of the `Pod` trait:
///
/// * Require `#[repr(C)]` annotation.
/// * Require all struct members to implement `Pod`.
/// * Require the size of the struct equal to the sum of the sizes of its members.
///
#[proc_macro_derive(Pod)]
pub fn pod_derive(input: TokenStream) -> TokenStream {
	return format!("::pelite::derive_pod!{{ {} }}", input).parse().unwrap()
}

/// Implement the `Pod` trait for use in the `pelite` crate itself.
#[doc(hidden)]
#[proc_macro_derive(_Pod)]
pub fn _pod_derive(input: TokenStream) -> TokenStream {
	return format!("crate::derive_pod!{{ {} }}", input).parse().unwrap()
}

/// Compile time pattern parser.
///
/// Pending function-like proc-macro stabilisation this attribute does a textual replacement of `pattern!` invocations.
/// The macro invocation is replaced with a const array expression of the pattern atoms.
///
/// See `pelite::pattern` macro for the macro rules wrapper around this implementation detail.
///
/// ```
/// #[pattern_attribute]
/// const PATTERN: &[pelite::pattern::Atom] = &pattern!("pattern string");
/// ```
#[proc_macro_attribute]
pub fn pattern_attribute(_args: TokenStream, input: TokenStream) -> TokenStream {
	let input = input.to_string();
	let mut input: &str = &input;
	let mut result = String::new();
	while let Some(pos) = input.find("pattern!") {
		// Add all the text before the input
		result.push_str(&input[..pos]);
		// Find, parse and splice the pattern string
		let (consumed, string) = parse_str_literal(&input[pos + 8..]);
		let pattern = match pattern::parse(&string) {
			Ok(pattern) => pattern,
			Err(err) => panic!("invalid pattern syntax: {}", err),
		};
		result.push_str(&format!("{{ use ::pelite::pattern::Atom::*; {:?} }}", pattern));
		// Continue looking for other patterns to parse
		input = &input[pos + 8 + consumed..];
	}
	// Add all the remaining text and parse
	result.push_str(input);
	result.parse().unwrap()
}

fn parse_str_literal(input: &str) -> (usize, String) {
	let mut chars = input.chars();
	let mut string = String::new();
	if chars.next() != Some('(') {
		panic!("expected macro invocation with parenthesis")
	}
	if chars.next() != Some('"') {
		panic!("expected string literal starting with a `\"` and no extraneous whitespace");
	}
	loop {
		let chr = match chars.next() {
			Some('\\') => {
				match chars.next() {
					Some('\\') => '\\',
					Some('\'') => '\'',
					Some('\"') => '\"',
					Some('t') => '\t',
					Some('r') => '\r',
					Some('n') => '\n',
					Some('u') => panic!("unicode escape sequence not supported"),
					Some(chr) => panic!("unknown escape sequence: {}", chr),
					None => panic!(""),
				}
			},
			Some('"') => break,
			Some(chr) => chr,
			None => panic!("unexpected end of string literal, missing `\"` terminator?"),
		};
		string.push(chr);
	}
	if chars.next() != Some(')') {
		panic!("expected macro invocation with parenthesis and no extraneous whitespace");
	}
	let consumed = chars.as_str().as_ptr() as usize - input.as_ptr() as usize;
	(consumed, string)
}

// Total hack to get the pattern parse code in here :)
#[path = "../pattern.rs"]
#[allow(unused)]
mod pattern;
