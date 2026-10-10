use super::*;

/// Comment annotation.
///
/// ```text
/// Cx1000 "Comment"
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommentFact {
	/// Address of the annotation.
	pub rva: u32,
	/// Comment text.
	pub comment: String,
}

impl str::FromStr for CommentFact {
	type Err = ParseLineError;

	/// Parse `CxRVA "comment"`, decoding JSON string escapes in the comment.
	fn from_str(line: &str) -> Result<Self, Self::Err> {
		let (rva, rest) = token(line)?;
		let rva = rva.strip_prefix('C').ok_or(ParseLineError::InvalidPrefix("C"))?;
		let rva = parse_rva(rva)?;
		let (comment, rest) = token(rest)?;
		if !rest.trim_ascii().is_empty() {
			return Err(ParseLineError::UnexpectedText("comment"));
		}
		let comment = serde_json::from_str::<String>(comment)
			.map_err(|error| ParseLineError::InvalidComment(error.to_string()))?;
		Ok(CommentFact { rva, comment })
	}
}

impl fmt::Display for CommentFact {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let comment = serde_json::to_string(&self.comment).map_err(|_| fmt::Error)?;
		write!(f, "Cx{:x} {}", self.rva, comment)
	}
}

#[test]
fn parse_comment() {
	let fact = CommentFact::from_str(r#"Cx1234 "a \"quoted\" comment\nnext line""#).unwrap();
	assert_eq!(fact.rva, 0x1234);
	assert_eq!(fact.comment, "a \"quoted\" comment\nnext line");
	assert_eq!(fact.to_string(), r#"Cx1234 "a \"quoted\" comment\nnext line""#);
	assert_eq!(CommentFact::from_str(&fact.to_string()).unwrap(), fact);
	assert_eq!(CommentFact::from_str(" Cx0 \"\" \t").unwrap().comment, "");
}

#[test]
fn reject_invalid_comment() {
	for (line, error) in [
		("", ParseLineError::MissingField),
		("x1 \"text\"", ParseLineError::InvalidPrefix("C")),
		("Rx1 \"text\"", ParseLineError::InvalidPrefix("C")),
		("Cx1", ParseLineError::MissingField),
		("C0x1 \"text\"", ParseLineError::InvalidRva),
		("Cx100000000 \"text\"", ParseLineError::InvalidRva),
		("Cx1 \"unfinished", ParseLineError::UnterminatedQuotedField),
		("Cx1 \"text\" extra", ParseLineError::UnexpectedText("comment")),
	] {
		assert_eq!(CommentFact::from_str(line).unwrap_err(), error, "{line}");
	}
	assert!(matches!(CommentFact::from_str("Cx1 unquoted"), Err(ParseLineError::InvalidComment(_))));
	assert!(matches!(CommentFact::from_str(r#"Cx1 "bad\q""#), Err(ParseLineError::InvalidComment(_))));
}
