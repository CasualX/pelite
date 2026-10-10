use super::*;

#[cfg(test)]
use std::str::FromStr;

/// Function metadata.
///
/// ```text
/// Fx1000 {"bytes":199,...}
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionFact {
	/// Function entry address.
	pub rva: u32,
	/// Opaque content, typically JSON but not validated.
	pub content: String,
}

impl FunctionFact {
	/// Recursively merge JSON objects; other values and non-JSON content are replaced.
	pub fn merge_content(&mut self, content: String) {
		fn merge(previous: &mut serde_json::Value, newer: serde_json::Value) {
			match (previous, newer) {
				(serde_json::Value::Object(previous), serde_json::Value::Object(newer)) => {
					for (key, value) in newer {
						if let Some(previous) = previous.get_mut(&key) {
							merge(previous, value);
						}
						else {
							previous.insert(key, value);
						}
					}
				},
				(previous, newer) => *previous = newer,
			}
		}

		self.content = match (serde_json::from_str::<serde_json::Value>(&self.content), serde_json::from_str::<serde_json::Value>(&content)) {
			(Ok(mut previous), Ok(newer)) => {
				merge(&mut previous, newer);
				previous.to_string()
			},
			_ => content,
		};
	}
}

impl str::FromStr for FunctionFact {
	type Err = ParseLineError;

	/// Parse `FxRVA content`, trimming surrounding whitespace from nonempty content.
	/// The content is preserved without validating it as JSON.
	fn from_str(line: &str) -> Result<Self, Self::Err> {
		let (rva, rest) = token(line)?;
		let rva = rva.strip_prefix('F').ok_or(ParseLineError::InvalidPrefix("F"))?;
		let rva = parse_rva(rva)?;
		let content = rest.trim_ascii();
		if content.is_empty() {
			return Err(ParseLineError::MissingField);
		}
		Ok(FunctionFact { rva, content: content.to_owned() })
	}
}

impl fmt::Display for FunctionFact {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Fx{:x} {}", self.rva, self.content)
	}
}

#[test]
fn merge_function_content_replaces_non_objects_and_invalid_json() {
	for (previous, newer, expected) in [
		(r#"{"x":1}"#, "null", "null"),
		("null", r#"{"x":1}"#, r#"{"x":1}"#),
		("[1,2]", "[3]", "[3]"),
		("1", "2", "2"),
		("invalid", r#"{ "x": 1 }"#, r#"{ "x": 1 }"#),
		(r#"{"x":1}"#, "arbitrary content", "arbitrary content"),
	] {
		let mut fact = FunctionFact { rva: 0x10, content: previous.into() };
		fact.merge_content(newer.into());
		assert_eq!(fact.content, expected);
	}
}

#[test]
fn reject_invalid_function() {
	for (line, error) in [
		("", ParseLineError::MissingField),
		("x1 {}", ParseLineError::InvalidPrefix("F")),
		("Cx1 {}", ParseLineError::InvalidPrefix("F")),
		("Fx1", ParseLineError::MissingField),
		("Fx1 \t", ParseLineError::MissingField),
		("F0x1 {}", ParseLineError::InvalidRva),
		("Fx {}", ParseLineError::InvalidRva),
		("Fxgg {}", ParseLineError::InvalidRva),
		("Fx100000000 {}", ParseLineError::InvalidRva),
	] {
		assert_eq!(FunctionFact::from_str(line).unwrap_err(), error, "{line}");
	}
}
