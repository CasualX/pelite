/*!
Version Information.

See [Microsoft's documentation](https://docs.microsoft.com/en-us/windows/desktop/menurc/version-information) for more information.

# Examples

See also the `examples/version_info.rs` example which reads and prints the version info of the given file.

```
use pelite::PeFile;

fn example(bin: PeFile<'_>) -> Result<(), pelite::resources::FindError> {
    let resources = bin.resources()?;
    let version_info = resources.version_info()?;

    // Get and print the fixed file info
    println!("FixedFileInfo: {:?}", version_info.fixed());

    // Get the first available language
    let lang = version_info.translation()[0];

    // Query some properties
    let company_name = version_info.value(lang, "CompanyName");

    // Print all the properties for this language
    version_info.strings(lang, |key, value| {
        println!("{}: {:?}", key, value);
    });

    // Dump the version info into hashmaps for later consumption or serialization
    let file_info = version_info.file_info();

    // Transform the version info back into source code
    let source_code = version_info.source_code();

    Ok(())
}
```

 */

#[cfg(not(feature = "std"))]
use hashbrown::HashMap;
#[cfg(feature = "std")]
use std::collections::HashMap;

use std::prelude::v1::*;
use std::{char, cmp, fmt, mem, slice};

use std::fmt::Write;

use crate::image::VS_FIXEDFILEINFO;
use crate::util::{wstrn, AlignTo, FmtUtf16};
use crate::{Error, Pod, Result};

//----------------------------------------------------------------

/// Language and charset pair.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Language {
    /// References [langID](https://docs.microsoft.com/en-us/windows/desktop/menurc/versioninfo-resource#langID) constants.
    pub lang_id: u16,
    /// References [charsetID](https://docs.microsoft.com/en-us/windows/desktop/menurc/versioninfo-resource#charsetID) constants.
    pub charset_id: u16,
}
unsafe impl Pod for Language {}
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
            if word >= 'a' as u16 {
                lower
            } else if word >= 'A' as u16 {
                upper
            } else {
                num
            }
        }
        let mut digits = [0u16; 8];
        for i in 0..8 {
            digits[i] = digit(lang[i]);
        }
        let lang_id = (digits[0] << 12) | (digits[1] << 8) | (digits[2] << 4) | digits[3];
        let charset_id = (digits[4] << 12) | (digits[5] << 8) | (digits[6] << 4) | digits[7];
        Ok(Language {
            lang_id,
            charset_id,
        })
    }
    fn from_slice<'a>(words: &'a [u16]) -> &'a [Language] {
        let len = words.len() / 2;
        unsafe { slice::from_raw_parts(words.as_ptr() as *const Language, len) }
    }
}
impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04X}{:04X}", self.lang_id, self.charset_id)
    }
}

//----------------------------------------------------------------

/// Version Information.
#[derive(Copy, Clone)]
pub struct VersionInfo<'a> {
    words: &'a [u16],
}
impl<'a> VersionInfo<'a> {
    pub fn try_from(bytes: &'a [u8]) -> Result<VersionInfo<'a>> {
        // Alignment of 4 bytes is assumed everywhere,
        // unsafe code in this module relies on this
        if !bytes.as_ptr().aligned_to(4) {
            return Err(Error::Misaligned);
        }
        let words = unsafe { slice::from_raw_parts(bytes.as_ptr() as *const u16, bytes.len() / 2) };
        Ok(VersionInfo { words })
    }

    /// Gets the fixed file information if available.
    ///
    /// Queries `\`.
    pub fn fixed(self) -> Option<&'a VS_FIXEDFILEINFO> {
        let mut this = QueryFixed(None);
        self.visit(&mut this);
        this.0
    }
    /// Gets the available languages.
    ///
    /// Queries `\VarFileInfo\Translation`.
    pub fn translation(self) -> &'a [Language] {
        let mut this = QueryTranslation(&[]);
        self.visit(&mut this);
        this.0
    }
    /// Gets a string value by name.
    ///
    /// Queries `\StringFileInfo\{lang}\{key}`
    pub fn value(self, lang: Language, key: &str) -> Option<String> {
        let mut this = QueryValue {
            lang,
            key,
            value: None,
        };
        self.visit(&mut this);
        this.value
    }
    /// Iterates over all the strings' keys and values of a given language.
    ///
    /// Queries `\StringFileInfo\{lang}\*`
    pub fn strings<F: FnMut(&str, &str)>(self, lang: Language, f: F) {
        self.visit(&mut QueryStrings { lang, f });
    }
    /// Parse the version info into HashMaps.
    pub fn file_info(self) -> FileInfo<'a> {
        let mut file_info = FileInfo::default();
        self.visit(&mut file_info);
        file_info
    }
    /// Renders the version info back into its source code form.
    pub fn source_code(self) -> String {
        let mut source_code = String::new();
        self.visit(&mut source_code);
        source_code
    }

    /// Parse the version information.
    ///
    /// Because of the super convoluted format, the visitor pattern is used.
    /// Implement the [`Visit` trait](trait.Visit.html) to get the desired information.
    ///
    /// To keep the API simple all errors are ignored, any invalid or corrupted data is skipped.
    pub fn visit(self, visit: &mut dyn Visit<'a>) {
        for version_info in Parser::new_bytes(self.words).filter_map(Result::ok) {
            const VS_FIXEDFILEINFO_SIZEOF: usize = mem::size_of::<VS_FIXEDFILEINFO>();
            let fixed = match mem::size_of_val(version_info.value) {
                0 => None,
                VS_FIXEDFILEINFO_SIZEOF => {
                    let value =
                        unsafe { &*(version_info.value.as_ptr() as *const VS_FIXEDFILEINFO) };
                    Some(value)
                }
                _ => None,
            };

            if !visit.version_info(version_info.key, fixed) {
                continue;
            }

            // MS docs: This member is always equal to zero.
            visit.enter_scope(0);
            for file_info in Parser::new_zero(version_info.children).filter_map(Result::ok) {
                if !visit.file_info(file_info.key) {
                    continue;
                }

                // MS docs: L"StringFileInfo"
                visit.enter_scope(1);
                if file_info.key == &self::strings::StringFileInfo {
                    // MS docs: This member is always equal to zero.
                    for string_table in Parser::new_zero(file_info.children).filter_map(Result::ok)
                    {
                        if !visit.string_table(string_table.key) {
                            continue;
                        }

                        visit.enter_scope(2);
                        for string in
                            Parser::new_words(string_table.children).filter_map(Result::ok)
                        {
                            // Strip the nul terminator...
                            let value = if string.value.last() != Some(&0) {
                                string.value
                            } else {
                                &string.value[..string.value.len() - 1]
                            };
                            visit.string(string.key, value);
                        }
                        visit.exit_scope(2);
                    }
                }
                // MS docs: L"VarFileInfo"
                else if file_info.key == &self::strings::VarFileInfo {
                    for var in Parser::new_bytes(file_info.children).filter_map(Result::ok) {
                        visit.var(var.key, var.value);
                    }
                }
                visit.exit_scope(1);
            }
            visit.exit_scope(0);

            // Ignore any additional version infos...
            return;
        }
    }
}
impl fmt::Debug for VersionInfo<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let file_info = self.file_info();
        f.debug_struct("VersionInfo")
            .field("fixed", &file_info.fixed)
            .field("strings", &file_info.strings)
            .field("langs", &file_info.langs)
            .finish()
    }
}

//----------------------------------------------------------------

/// Visitor pattern to view the version information details.
#[allow(unused_variables)]
pub trait Visit<'a> {
    fn version_info(&mut self, key: &'a [u16], fixed: Option<&'a VS_FIXEDFILEINFO>) -> bool {
        true
    }
    fn file_info(&mut self, key: &'a [u16]) -> bool {
        true
    }
    fn string_table(&mut self, lang: &'a [u16]) -> bool {
        true
    }
    fn string(&mut self, key: &'a [u16], value: &'a [u16]) {}
    fn var(&mut self, key: &'a [u16], value: &'a [u16]) {}
    fn enter_scope(&mut self, depth: usize) {}
    fn exit_scope(&mut self, depth: usize) {}
}

struct QueryFixed<'a>(Option<&'a VS_FIXEDFILEINFO>);
impl<'a> Visit<'a> for QueryFixed<'a> {
    fn version_info(&mut self, _key: &'a [u16], fixed: Option<&'a VS_FIXEDFILEINFO>) -> bool {
        self.0 = fixed;
        true
    }
    fn file_info(&mut self, _key: &'a [u16]) -> bool {
        false
    }
}

struct QueryTranslation<'a>(&'a [Language]);
impl<'a> Visit<'a> for QueryTranslation<'a> {
    fn file_info(&mut self, key: &'a [u16]) -> bool {
        key == strings::VarFileInfo
    }
    fn var(&mut self, key: &'a [u16], value: &'a [u16]) {
        if key == strings::Translation {
            self.0 = Language::from_slice(value);
        }
    }
}

struct QueryValue<'z> {
    lang: Language,
    key: &'z str,
    value: Option<String>,
}
impl<'a, 'z> Visit<'a> for QueryValue<'z> {
    fn file_info(&mut self, key: &'a [u16]) -> bool {
        key == strings::StringFileInfo
    }
    fn string_table(&mut self, lang: &'a [u16]) -> bool {
        match Language::parse(lang) {
            Ok(lang) => lang == self.lang,
            Err(_) => false,
        }
    }
    fn string(&mut self, key: &'a [u16], value: &'a [u16]) {
        if Iterator::eq(
            self.key.chars().map(Ok),
            char::decode_utf16(key.iter().cloned()),
        ) {
            let value = String::from_utf16_lossy(value);
            self.value = Some(value);
        }
    }
}

struct QueryStrings<F> {
    lang: Language,
    f: F,
}
impl<'a, F: FnMut(&str, &str)> Visit<'a> for QueryStrings<F> {
    fn string_table(&mut self, lang: &'a [u16]) -> bool {
        match Language::parse(lang) {
            Ok(lang) => lang == self.lang,
            Err(_) => false,
        }
    }
    fn string(&mut self, key: &'a [u16], value: &'a [u16]) {
        let key = String::from_utf16_lossy(key);
        let value = String::from_utf16_lossy(value);
        (self.f)(&key, &value);
    }
}

impl<'a> Visit<'a> for String {
    fn version_info(&mut self, _key: &'a [u16], fixed: Option<&'a VS_FIXEDFILEINFO>) -> bool {
        if let Some(fixed) = fixed {
            let _ = writeln!(
                self,
                "\
1 VERSIONINFO
FILEVERSION {}, {}, {}, {}
PRODUCTVERSION {}, {}, {}, {}
FILEFLAGSMASK {:#x}
FILEFLAGS {:#x}
FILEOS ({} << 16) | {}
FILETYPE {}
FILESUBTYPE {}",
                fixed.dwFileVersion.Major,
                fixed.dwFileVersion.Minor,
                fixed.dwFileVersion.Patch,
                fixed.dwFileVersion.Build,
                fixed.dwProductVersion.Major,
                fixed.dwProductVersion.Minor,
                fixed.dwProductVersion.Patch,
                fixed.dwProductVersion.Build,
                fixed.dwFileFlagsMask,
                fixed.dwFileFlags,
                fixed.dwFileOS >> 16,
                fixed.dwFileOS & 0xffff,
                fixed.dwFileType,
                fixed.dwFileSubtype
            );
        }
        true
    }
    fn file_info(&mut self, key: &'a [u16]) -> bool {
        let _ = writeln!(self, "  BLOCK {:?}", FmtUtf16(key));
        true
    }
    fn string_table(&mut self, lang: &'a [u16]) -> bool {
        let _ = writeln!(self, "    BLOCK {:?}", FmtUtf16(lang));
        true
    }
    fn string(&mut self, key: &'a [u16], value: &'a [u16]) {
        let _ = writeln!(
            self,
            "      VALUE {:?}, {:?}",
            FmtUtf16(key),
            FmtUtf16(value)
        );
    }
    fn var(&mut self, key: &'a [u16], value: &'a [u16]) {
        // Don't know how to interpret any other Var key...
        if key != strings::Translation {
            return;
        }
        struct PrintLangs<'a>(&'a [Language]);
        impl fmt::Display for PrintLangs<'_> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                for lang in self.0 {
                    write!(f, ", {}, {}", lang.lang_id, lang.charset_id)?;
                }
                Ok(())
            }
        }
        let langs = Language::from_slice(value);
        let _ = writeln!(self, "    VALUE {:?}{}", FmtUtf16(key), PrintLangs(langs));
    }
    fn enter_scope(&mut self, depth: usize) {
        let _ = writeln!(self, "{}{{", &"        "[..depth * 2]);
    }
    fn exit_scope(&mut self, depth: usize) {
        let _ = writeln!(self, "{}}}", &"        "[..depth * 2]);
    }
}

/// VersionInfo parsed into HashMaps.
#[derive(Clone, Debug, Default)]
#[cfg_attr(all(feature = "std", feature = "serde"), derive(::serde::Serialize))]
pub struct FileInfo<'a> {
    pub fixed: Option<&'a VS_FIXEDFILEINFO>,
    pub strings: HashMap<Language, HashMap<String, String>>,
    pub langs: &'a [Language],
    #[cfg_attr(feature = "serde", serde(skip))]
    lang: Language,
}
impl<'a> Visit<'a> for FileInfo<'a> {
    fn version_info(&mut self, _key: &'a [u16], fixed: Option<&'a VS_FIXEDFILEINFO>) -> bool {
        self.fixed = fixed;
        true
    }
    fn string_table(&mut self, lang: &'a [u16]) -> bool {
        if let Ok(lang) = Language::parse(lang) {
            self.lang = lang;
            self.strings.insert(lang, HashMap::new());
            return true;
        }
        false
    }
    fn string(&mut self, key: &'a [u16], value: &'a [u16]) {
        if let Some(entry) = self.strings.get_mut(&self.lang) {
            let key = String::from_utf16_lossy(key);
            let value = String::from_utf16_lossy(value);
            entry.insert(key, value);
        }
    }
    fn var(&mut self, key: &'a [u16], value: &'a [u16]) {
        if key == strings::Translation {
            self.langs = Language::from_slice(value);
        }
    }
}

//----------------------------------------------------------------

/*
    "version_info": {
        "fixed": { .. },
        "strings": {
            "040904B0": { ... }
        },
        "langs": ["040904B0"],
    },
*/

#[cfg(all(feature = "std", feature = "serde"))]
mod serde {
    use super::{Language, VersionInfo};
    use crate::util::serde_helper::*;

    impl Serialize for Language {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serializer.collect_str(self)
        }
    }

    impl<'a> Serialize for VersionInfo<'a> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.file_info().serialize(serializer)
        }
    }
}

//----------------------------------------------------------------

mod strings {
    #![allow(non_upper_case_globals)]
    // static VS_VERSION_INFO: [u16; 15] = [86u16, 83, 95, 86, 69, 82, 83, 73, 79, 78, 95, 73, 78, 70, 79];
    pub(super) static StringFileInfo: [u16; 14] = [
        83u16, 116, 114, 105, 110, 103, 70, 105, 108, 101, 73, 110, 102, 111,
    ];
    pub(super) static VarFileInfo: [u16; 11] =
        [86u16, 97, 114, 70, 105, 108, 101, 73, 110, 102, 111];
    pub(super) static Translation: [u16; 11] =
        [84u16, 114, 97, 110, 115, 108, 97, 116, 105, 111, 110];
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
    let _langs = version_info.translation();
    let _file_info = version_info.file_info();
    let _source_code = version_info.source_code();
}

//----------------------------------------------------------------

/// Fixed file info constants.
pub mod image {
    pub const VS_FF_DEBUG: u32 = 0x01;
    pub const VS_FF_PRERELEASE: u32 = 0x02;
    pub const VS_FF_PATCHED: u32 = 0x04;
    pub const VS_FF_PRIVATEBUILD: u32 = 0x08;
    pub const VS_FF_INFOINFERRED: u32 = 0x10;
    pub const VS_FF_SPECIALBUILD: u32 = 0x20;

    pub const VOS_UNKNOWN: u32 = 0x00000000;
    pub const VOS_DOS: u32 = 0x00010000;
    pub const VOS_OS216: u32 = 0x00020000;
    pub const VOS_OS232: u32 = 0x00030000;
    pub const VOS_NT: u32 = 0x00040000;
    pub const VOS__WINDOWS16: u32 = 0x00000001;
    pub const VOS__PM16: u32 = 0x00000002;
    pub const VOS__PM32: u32 = 0x00000003;
    pub const VOS__WINDOWS32: u32 = 0x00000004;

    pub const VFT_UNKNOWN: u32 = 0x00000000;
    pub const VFT_APP: u32 = 0x00000001;
    pub const VFT_DLL: u32 = 0x00000002;
    pub const VFT_DRV: u32 = 0x00000003;
    pub const VFT_FONT: u32 = 0x00000004;
    pub const VFT_VXD: u32 = 0x00000005;
    pub const VFT_STATIC_LIB: u32 = 0x00000007;

    pub const VFT2_UNKNOWN: u32 = 0x00000000;

    pub const VFT2_DRV_PRINTER: u32 = 0x00000001;
    pub const VFT2_DRV_KEYBOARD: u32 = 0x00000002;
    pub const VFT2_DRV_LANGUAGE: u32 = 0x00000003;
    pub const VFT2_DRV_DISPLAY: u32 = 0x00000004;
    pub const VFT2_DRV_MOUSE: u32 = 0x00000005;
    pub const VFT2_DRV_NETWORK: u32 = 0x00000006;
    pub const VFT2_DRV_SYSTEM: u32 = 0x00000007;
    pub const VFT2_DRV_INSTALLABLE: u32 = 0x00000008;
    pub const VFT2_DRV_SOUND: u32 = 0x00000009;
    pub const VFT2_DRV_COMM: u32 = 0x0000000A;
    pub const VFT2_DRV_VERSIONED_PRINTER: u32 = 0x0000000C;

    pub const VFT2_FONT_RASTER: u32 = 0x00000001;
    pub const VFT2_FONT_VECTOR: u32 = 0x00000002;
    pub const VFT2_FONT_TRUETYPE: u32 = 0x00000003;
}

//----------------------------------------------------------------
// This is an absolutely god awful format...

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct TLV<'a> {
    pub key: &'a [u16],
    pub value: &'a [u16],    // DWORD aligned
    pub children: &'a [u16], // DWORD aligned
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum ValueLengthType {
    Zero,
    Bytes,
    Words,
}
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
        Parser {
            words,
            vlt: ValueLengthType::Zero,
        }
    }
    pub(crate) fn new_bytes(words: &'a [u16]) -> Parser<'a> {
        Parser {
            words,
            vlt: ValueLengthType::Bytes,
        }
    }
    pub(crate) fn new_words(words: &'a [u16]) -> Parser<'a> {
        Parser {
            words,
            vlt: ValueLengthType::Words,
        }
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
    let length = cmp::max(4, words[0] as usize / 2);
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
    // The length does not contain padding to align to a 32-bit boundary
    state.words = &words[cmp::min(length.align_to(2), words.len())..];
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
    // The length does not contain padding to align to a 32-bit boundary
    let children = &words[cmp::min(value.len().align_to(2), words.len())..];

    Ok(TLV {
        key,
        value,
        children,
    })
}

#[test]
fn test_parse_tlv_oob() {
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

#[test]
fn test_parse_254() {
    static WORDS: [u16; 397] = [
        794, 52, 0, 86, 83, 95, 86, 69, 82, 83, 73, 79, 78, 95, 73, 78, 70, 79, 0, 0, 1213, 65263,
        0, 1, 607, 22, 25, 2013, 607, 22, 25, 2013, 63, 0, 0, 0, 4, 0, 2, 0, 0, 0, 0, 0, 0, 0, 68,
        0, 1, 86, 97, 114, 70, 105, 108, 101, 73, 110, 102, 111, 0, 0, 36, 4, 0, 84, 114, 97, 110,
        115, 108, 97, 116, 105, 111, 110, 0, 0, 0, 1200, 634, 0, 1, 83, 116, 114, 105, 110, 103,
        70, 105, 108, 101, 73, 110, 102, 111, 0, 598, 0, 1, 48, 48, 48, 48, 48, 52, 98, 48, 0, 58,
        13, 1, 67, 111, 109, 112, 97, 110, 121, 78, 97, 109, 101, 0, 0, 66, 69, 46, 69, 115, 115,
        101, 110, 116, 105, 97, 108, 0, 0, 66, 13, 1, 70, 105, 108, 101, 68, 101, 115, 99, 114,
        105, 112, 116, 105, 111, 110, 0, 0, 66, 69, 46, 69, 115, 115, 101, 110, 116, 105, 97, 108,
        0, 0, 62, 15, 1, 70, 105, 108, 101, 86, 101, 114, 115, 105, 111, 110, 0, 0, 50, 50, 46, 54,
        48, 55, 46, 50, 48, 49, 51, 46, 50, 53, 0, 0, 66, 17, 1, 73, 110, 116, 101, 114, 110, 97,
        108, 78, 97, 109, 101, 0, 66, 69, 46, 69, 115, 115, 101, 110, 116, 105, 97, 108, 46, 100,
        108, 108, 0, 0, 40, 2, 1, 76, 101, 103, 97, 108, 67, 111, 112, 121, 114, 105, 103, 104,
        116, 0, 32, 0, 74, 17, 1, 79, 114, 105, 103, 105, 110, 97, 108, 70, 105, 108, 101, 110, 97,
        109, 101, 0, 66, 69, 46, 69, 115, 115, 101, 110, 116, 105, 97, 108, 46, 100, 108, 108, 0,
        0, 58, 13, 1, 80, 114, 111, 100, 117, 99, 116, 78, 97, 109, 101, 0, 0, 66, 69, 46, 69, 115,
        115, 101, 110, 116, 105, 97, 108, 0, 0, 66, 15, 1, 80, 114, 111, 100, 117, 99, 116, 86,
        101, 114, 115, 105, 111, 110, 0, 50, 50, 46, 54, 48, 55, 46, 50, 48, 49, 51, 46, 50, 53, 0,
        0, 70, 15, 1, 65, 115, 115, 101, 109, 98, 108, 121, 32, 86, 101, 114, 115, 105, 111, 110,
        0, 50, 50, 46, 54, 48, 55, 46, 50, 48, 49, 51, 46, 50, 53, 0,
    ];

    let vi = VersionInfo { words: &WORDS };
    let fi = vi.file_info();
    assert!(fi.fixed.is_some());

    let mut strings = HashMap::new();
    strings.insert(
        Language {
            lang_id: 0,
            charset_id: 1200,
        },
        {
            let mut strings = HashMap::new();
            strings.insert(
                String::from("FileDescription"),
                String::from("BE.Essential"),
            );
            strings.insert(
                String::from("Assembly Version"),
                String::from("22.607.2013.25"),
            );
            strings.insert(
                String::from("ProductVersion"),
                String::from("22.607.2013.25"),
            );
            strings.insert(String::from("CompanyName"), String::from("BE.Essential"));
            strings.insert(
                String::from("OriginalFilename"),
                String::from("BE.Essential.dll"),
            );
            strings.insert(String::from("FileVersion"), String::from("22.607.2013.25"));
            strings.insert(
                String::from("InternalName"),
                String::from("BE.Essential.dll"),
            );
            strings.insert(String::from("LegalCopyright"), String::from(" "));
            strings.insert(String::from("ProductName"), String::from("BE.Essential"));
            strings
        },
    );

    assert_eq!(fi.strings, strings);
    assert_eq!(
        fi.langs,
        &[Language {
            lang_id: 0,
            charset_id: 1200
        }]
    );
    // panic!("{:#?}", fi);
}
