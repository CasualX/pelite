/*!
Version Information.

See [Microsoft's documentation](https://docs.microsoft.com/en-us/windows/desktop/menurc/version-information) for more information.
 */

use std::{char, cmp, fmt, mem, slice};
use std::collections::HashMap;

use crate::image::VS_FIXEDFILEINFO;
use crate::{Error, Result, _Pod as Pod};
use crate::util::{AlignTo, wstrn};

//----------------------------------------------------------------

/// Language and charset pair.
///
/// References [langID](https://docs.microsoft.com/en-us/windows/desktop/menurc/versioninfo-resource#langID) and [charsetID](https://docs.microsoft.com/en-us/windows/desktop/menurc/versioninfo-resource#charsetID).
#[derive(Copy, Clone, Debug, Pod, Eq, PartialEq)]
#[repr(C)]
pub struct Language {
	pub lang_id: u16,
	pub charset_id: u16,
}
impl Language {
	/// Parse language hex strings.
	pub fn parse(lang: &[u16]) -> std::result::Result<Language, &[u16]> {
		if lang.len() != 8 {
			return Err(lang);
		}
		fn digit(word: u16) -> u16 {
			let num = word.wrapping_sub('0' as u16);
			let upper = word.wrapping_sub('A' as u16).wrapping_add(10);
			let lower = word.wrapping_sub('a' as u16).wrapping_add(10);
			if word >= 'a' as u16 { lower }
			else if word >= 'A' as u16 { upper }
			else { num }
		}
		let mut digits = [0u16; 8];
		for i in 0..8 {
			digits[i] = digit(lang[i]);
		}
		let lang_id = (digits[0] << 12) | (digits[1] << 8) | (digits[2] << 4) | digits[3];
		let charset_id = (digits[4] << 12) | (digits[5] << 8) | (digits[6] << 4) | digits[7];
		Ok(Language { lang_id, charset_id })
	}
}
impl fmt::Display for Language {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:04X}{:04X}", self.lang_id, self.charset_id)
	}
}

//----------------------------------------------------------------

/// Version Information.
#[derive(Copy, Clone, Debug)]
pub struct VersionInfo<'a> {
	bytes: &'a [u8],
}
impl<'a> VersionInfo<'a> {
	pub fn try_from(bytes: &'a [u8]) -> Result<VersionInfo<'a>> {
		// Alignment of 4 bytes is assumed everywhere,
		// unsafe code in this module relies on this
		if !bytes.as_ptr().aligned_to(4) {
			return Err(Error::Misaligned);
		}
		Ok(VersionInfo { bytes })
	}

	/// Gets the fixed file information if available.
	pub fn fixed(self) -> Option<&'a VS_FIXEDFILEINFO> {
		let mut fixed = None;
		self.visit(&mut fixed);
		fixed
	}
	/// Queries a string value by name.
	///
	/// The returned string is UTF-16 encoded, convert to UTF-8 with `String::from_utf16` and friends.
	pub fn query_value<S: AsRef<str>>(self, key: &S) -> Option<&'a [u16]> {
		let mut this = QueryValue {
			key: key.as_ref(),
			value: None,
		};
		self.visit(&mut this);
		this.value
	}
	/// Iterates over all the strings.
	///
	/// The closure's arguments are the lang, name and value for each string pair in the version information.
	pub fn for_each_string<F: FnMut(&'a [u16], &'a [u16], &'a [u16])>(self, mut f: F) {
		self.visit(&mut ForEachString(&mut f));
	}
	/// Gets the strings in a hash map.
	pub fn to_hash_map(self) -> HashMap<String, String> {
		let mut hash_map = HashMap::new();
		self.visit(&mut hash_map);
		hash_map
	}

	/// Parse the version information.
	///
	/// Because of the super convoluted format, the visitor pattern is used.
	/// Implement the [`Visit` trait](trait.Visit.html) to get the desired information.
	///
	/// To keep the API simple all errors are ignored, any invalid or corrupted data is skipped.
	pub fn visit(self, visit: &mut dyn Visit<'a>) {
		let words = unsafe { slice::from_raw_parts(self.bytes.as_ptr() as *const u16, self.bytes.len() / 2) };

		for version_info_r in Parser::new_bytes(words) {
			if let Ok(version_info) = version_info_r {
				const VS_FIXEDFILEINFO_SIZEOF: usize = mem::size_of::<VS_FIXEDFILEINFO>();
				let fixed = match mem::size_of_val(version_info.value) {
					0 => None,
					VS_FIXEDFILEINFO_SIZEOF => {
						let value = unsafe { &*(version_info.value.as_ptr() as *const VS_FIXEDFILEINFO) };
						Some(value)
					},
					_ => None,//return Err(Error::Invalid),
				};

				if !visit.version_info(version_info.key, fixed) {
					continue;
				}

				// MS docs: This member is always equal to zero.
				for file_info_r in Parser::new_zero(version_info.children) {
					if let Ok(file_info) = file_info_r {
						if !visit.file_info(file_info.key) {
							continue;
						}

						// MS docs: L"StringFileInfo"
						if file_info.key == &self::strings::StringFileInfo {
							// MS docs: This member is always equal to zero.
							for string_table_r in Parser::new_zero(file_info.children) {
								if let Ok(string_table) = string_table_r {
									if !visit.string_table(string_table.key) {
										continue;
									}

									for string_r in Parser::new_words(string_table.children) {
										if let Ok(string) = string_r {
											// Strip the nul terminator...
											let value = if string.value.last() != Some(&0) { string.value }
											else { &string.value[..string.value.len() - 1] };
											visit.string(string_table.key, string.key, value);
										}
									}
								}
							}
						}
						// MS docs: L"VarFileInfo"
						else if file_info.key == &self::strings::VarFileInfo {
							for var_r in Parser::new_bytes(file_info.children) {
								if let Ok(var) = var_r {
									visit.var(var.key, var.value);
								}
							}
						}
					}
				}
			}
		}
	}
}

//----------------------------------------------------------------

/// Visitor pattern to view the version information details.
#[allow(unused_variables)]
pub trait Visit<'a> {
	fn version_info(&mut self, key: &'a [u16], fixed: Option<&'a VS_FIXEDFILEINFO>) -> bool { true }
	fn file_info(&mut self, key: &'a [u16]) -> bool { true }
	fn string_table(&mut self, lang: &'a [u16]) -> bool { true }
	fn string(&mut self, lang: &'a [u16], key: &'a [u16], value: &'a [u16]) {}
	fn var(&mut self, key: &'a [u16], pairs: &'a [u16]) {}
}

impl<'a> Visit<'a> for HashMap<String, String> {
	fn string(&mut self, _lang: &'a [u16], key: &'a [u16], value: &'a [u16]) {
		self.insert(
			String::from_utf16_lossy(key),
			String::from_utf16_lossy(value),
		);
	}
}
impl<'a> Visit<'a> for Option<&'a VS_FIXEDFILEINFO> {
	fn version_info(&mut self, _key: &'a [u16], fixed: Option<&'a VS_FIXEDFILEINFO>) -> bool {
		*self = fixed;
		false
	}
}

struct ForEachString<F>(F);
impl<'a, F: FnMut(&'a [u16], &'a [u16], &'a [u16])> Visit<'a> for ForEachString<F> {
	fn string(&mut self, lang: &'a [u16], key: &'a [u16], value: &'a [u16]) {
		(self.0)(lang, key, value);
	}
}

struct QueryValue<'a, 's> {
	key: &'s str,
	value: Option<&'a [u16]>,
}
impl<'a, 's> Visit<'a> for QueryValue<'a, 's> {
	fn string(&mut self, _lang: &'a [u16], key: &'a [u16], value: &'a [u16]) {
		if Iterator::eq(self.key.chars().map(Ok), char::decode_utf16(key.iter().cloned())) {
			self.value = Some(value);
		}
	}
}

//----------------------------------------------------------------

/*
	"version_info": {
		"fixed": { .. },
		"strings": { .. },
	},
*/

#[cfg(feature = "serde")]
mod serde {
	use crate::util::serde_helper::*;
	use super::{VersionInfo};

	impl<'a> Serialize for VersionInfo<'a> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let mut state = serializer.serialize_struct("VersionInfo", 2)?;
			state.serialize_field("fixed", &self.fixed())?;
			state.serialize_field("strings", &self.to_hash_map())?;
			state.end()
		}
	}
}

//----------------------------------------------------------------

mod strings {
	#![allow(non_upper_case_globals)]
	// static VS_VERSION_INFO: [u16; 15] = [86u16, 83, 95, 86, 69, 82, 83, 73, 79, 78, 95, 73, 78, 70, 79];
	pub(crate) static StringFileInfo: [u16; 14] = [83u16, 116, 114, 105, 110, 103, 70, 105, 108, 101, 73, 110, 102, 111];
	pub(crate) static VarFileInfo: [u16; 11] = [86u16, 97, 114, 70, 105, 108, 101, 73, 110, 102, 111];
	// static Translation: [u16; 11] = [84u16, 114, 97, 110, 115, 108, 97, 116, 105, 111, 110];
	// static Comments: [u16; 8] = [67u16, 111, 109, 109, 101, 110, 116, 115];
	// static CompanyName: [u16; 11] = [67u16, 111, 109, 112, 97, 110, 121, 78, 97, 109, 101];
	// static FileDescription: [u16; 15] = [70u16, 105, 108, 101, 68, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110];
	// static FileVersion: [u16; 11] = [70u16, 105, 108, 101, 86, 101, 114, 115, 105, 111, 110];
	// static InternalName: [u16; 12] = [73u16, 110, 116, 101, 114, 110, 97, 108, 78, 97, 109, 101];
	// static LegalCopyright: [u16; 14] = [76u16, 101, 103, 97, 108, 67, 111, 112, 121, 114, 105, 103, 104, 116];
	// static LegalTrademarks: [u16; 15] = [76u16, 101, 103, 97, 108, 84, 114, 97, 100, 101, 109, 97, 114, 107, 115];
	// static OriginalFilename: [u16; 16] = [79u16, 114, 105, 103, 105, 110, 97, 108, 70, 105, 108, 101, 110, 97, 109, 101];
	// static PrivateBuild: [u16; 12] = [80u16, 114, 105, 118, 97, 116, 101, 66, 117, 105, 108, 100];
	// static ProductName: [u16; 11] = [80u16, 114, 111, 100, 117, 99, 116, 78, 97, 109, 101];
	// static ProductVersion: [u16; 14] = [80u16, 114, 111, 100, 117, 99, 116, 86, 101, 114, 115, 105, 111, 110];
	// static SpecialBuild: [u16; 12] = [83u16, 112, 101, 99, 105, 97, 108, 66, 117, 105, 108, 100];
}

//----------------------------------------------------------------

#[cfg(test)]
pub(crate) fn test(version_info: VersionInfo<'_>) {
	let _fixed = version_info.fixed();
	let _hash_map = version_info.to_hash_map();
}

//----------------------------------------------------------------

/// Fixed file info constants.
pub mod image {
pub const VS_FF_DEBUG: u32        = 0x01;
pub const VS_FF_PRERELEASE: u32   = 0x02;
pub const VS_FF_PATCHED: u32      = 0x04;
pub const VS_FF_PRIVATEBUILD: u32 = 0x08;
pub const VS_FF_INFOINFERRED: u32 = 0x10;
pub const VS_FF_SPECIALBUILD: u32 = 0x20;

pub const VOS_UNKNOWN: u32    = 0x00000000;
pub const VOS_DOS: u32        = 0x00010000;
pub const VOS_OS216: u32      = 0x00020000;
pub const VOS_OS232: u32      = 0x00030000;
pub const VOS_NT: u32         = 0x00040000;
pub const VOS__WINDOWS16: u32 = 0x00000001;
pub const VOS__PM16: u32      = 0x00000002;
pub const VOS__PM32: u32      = 0x00000003;
pub const VOS__WINDOWS32: u32 = 0x00000004;

pub const VFT_UNKNOWN: u32    = 0x00000000;
pub const VFT_APP: u32        = 0x00000001;
pub const VFT_DLL: u32        = 0x00000002;
pub const VFT_DRV: u32        = 0x00000003;
pub const VFT_FONT: u32       = 0x00000004;
pub const VFT_VXD: u32        = 0x00000005;
pub const VFT_STATIC_LIB: u32 = 0x00000007;

pub const VFT2_UNKNOWN: u32               = 0x00000000;

pub const VFT2_DRV_PRINTER: u32           = 0x00000001;
pub const VFT2_DRV_KEYBOARD: u32          = 0x00000002;
pub const VFT2_DRV_LANGUAGE: u32          = 0x00000003;
pub const VFT2_DRV_DISPLAY: u32           = 0x00000004;
pub const VFT2_DRV_MOUSE: u32             = 0x00000005;
pub const VFT2_DRV_NETWORK: u32           = 0x00000006;
pub const VFT2_DRV_SYSTEM: u32            = 0x00000007;
pub const VFT2_DRV_INSTALLABLE: u32       = 0x00000008;
pub const VFT2_DRV_SOUND: u32             = 0x00000009;
pub const VFT2_DRV_COMM: u32              = 0x0000000A;
pub const VFT2_DRV_VERSIONED_PRINTER: u32 = 0x0000000C;

pub const VFT2_FONT_RASTER: u32           = 0x00000001;
pub const VFT2_FONT_VECTOR: u32           = 0x00000002;
pub const VFT2_FONT_TRUETYPE: u32         = 0x00000003;
}

//----------------------------------------------------------------
// This is an absolutely god awful format...

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct TLV<'a> {
	pub key: &'a [u16],
	pub value: &'a [u16], // DWORD aligned
	pub children: &'a [u16], // DWORD aligned
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum ValueLengthType { Zero, Bytes, Words }
#[derive(Clone)]
struct Parser<'a> {
	words: &'a [u16],
	vlt: ValueLengthType,
}
impl<'a> Iterator for Parser<'a> {
	type Item = Result<TLV<'a>>;
	fn next(&mut self) -> Option<Result<TLV<'a>>> {
		if self.words.len() == 0 {
			return None;
		}
		let result = parse_tlv(self);
		// If the parser errors, ensure the Iterator stops
		if result.is_err() {
			self.words = &self.words[self.words.len()..];
		}
		Some(result)
	}
}
impl<'a> Parser<'a> {
	pub(crate) fn new_zero(words: &'a [u16]) -> Parser<'a> {
		Parser { words, vlt: ValueLengthType::Zero }
	}
	pub(crate) fn new_bytes(words: &'a [u16]) -> Parser<'a> {
		Parser { words, vlt: ValueLengthType::Bytes }
	}
	pub(crate) fn new_words(words: &'a [u16]) -> Parser<'a> {
		Parser { words, vlt: ValueLengthType::Words }
	}
}
fn parse_tlv<'a>(state: &mut Parser<'a>) -> Result<TLV<'a>> {
	let mut words = state.words;
	// Parse the first three words from the TLV structure:
	// wLength, wValueLength and wType (plus at least zero terminator of szKey)
	if words.len() < 4 {
		return Err(Error::Invalid);
	}
	// This is tricky, the struct contains a fixed and variable length parts
	// However the length field includes the size of the fixed part
	// Further complicating things, if the variable length part is absent the total length is set to zero (?!)
	let length = cmp::max(4, words[0] as usize / 2).align_to(2);
	// Oh god why, interpret the value_length
	let value_length = match state.vlt {
		ValueLengthType::Zero if words[1] == 0 => 0,
		ValueLengthType::Zero => return Err(Error::Invalid),
		ValueLengthType::Bytes => words[1] as usize / 2,
		ValueLengthType::Words => words[1] as usize,
	};
	// let wType = words[2];

	// Split the input where this structure ends and the next sibling begins
	if length > words.len() {
		return Err(Error::Invalid);
	}
	state.words = &words[length..];
	words = &words[..length];

	// Parse the nul terminated szKey
	let key = wstrn(&words[3..]);
	if words[3..].len() == key.len() {
		return Err(Error::Invalid);
	}

	// Padding for the Value
	words = &words[key.len().align_to(2) + 4..];

	// Split the remaining words between the Value and Children
	if value_length > words.len() {
		return Err(Error::Invalid);
	}
	let value = &words[..value_length];
	let children = &words[value.len().align_to(2)..];

	Ok(TLV { key, value, children })
}

#[test]
fn test_parse_tlv_oob()
{
	let mut parser;

	// TLV header too short
	parser = Parser::new_zero(&[0, 0]);
	assert_eq!(parser.next(), Some(Err(Error::Invalid)));
	assert_eq!(parser.next(), None);

	// TLV length field larger than the data
	parser = Parser::new_zero(&[12, 0, 0, 0]);
	assert_eq!(parser.next(), Some(Err(Error::Invalid)));
	assert_eq!(parser.next(), None);

	// TLV key not nul terminated
	parser = Parser::new_zero(&[16, 0, 1, 20, 20, 20, 20, 20]);
	assert_eq!(parser.next(), Some(Err(Error::Invalid)));
	assert_eq!(parser.next(), None);

	// TLV value field larger than the data
	parser = Parser::new_zero(&[8, 10, 0, 0, 0, 0]);
	assert_eq!(parser.next(), Some(Err(Error::Invalid)));
	assert_eq!(parser.next(), None);
}
