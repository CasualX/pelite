/*!
PE headers.
 */

use super::image::*;

/*
	"headers": {
		"DosHeader": { .. }
		"NtHeaders": {
			"Signature": ..,
			"FileHeader": { .. }
			"OptionalHeader": { .. }
		}
		"DataDirectory": [ .. ]
		"SectionHeaders": [ .. ]
	}
*/

/// Describes the PE headers.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[allow(non_snake_case)]
pub struct Headers<'a> {
	pub DosHeader: &'a IMAGE_DOS_HEADER,
	pub NtHeaders: &'a IMAGE_NT_HEADERS,
	pub DataDirectory: &'a [IMAGE_DATA_DIRECTORY],
	pub SectionHeaders: &'a [IMAGE_SECTION_HEADER],
}
