#[cfg(test)]
use super::*;

#[cfg(test)]
pub(crate) fn test_resources<'a, P: Pe<'a>>(pe: P) -> Result<()> {
	pe.resources().and_then(crate::resources::test)
}
