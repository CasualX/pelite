/*!
Stringify image constants.
*/

use core::mem;
use core::str::FromStr;

use crate::image::*;

//----------------------------------------------------------------
// Define some reflection macros for enums and flags

macro_rules! enum1 {
	(
		$(#[$meta:meta])*
		$Item:ident($item:ident: $ty:ty),
		$($name:ident => $desc:expr,)*
	) => {
		$(#[$meta])*
		#[derive(Copy, Clone)]
		pub struct $Item(pub $ty);
		impl $Item {
			/// Gets the code identifier name for the value.
			pub fn to_str(self) -> Option<&'static str> {
				match self.0 {
					$($name => Some(stringify!($name)),)*
					_ => None,
				}
			}
			/// Gets a longer description for the value.
			pub fn description(self) -> Option<&'static str> {
				match self.0 {
					$($name => Some($desc),)*
					_ => None,
				}
			}
		}
		impl FromStr for $Item {
			type Err = ();
			fn from_str(s: &str) -> Result<$Item, ()> {
				match s {
					$(stringify!($name) => Ok($Item($name)),)*
					_ => Err(()),
				}
			}
		}
	}
}

macro_rules! flags {
	(
		$(#[$meta:meta])*
		$vis:vis $Item:ident($item:ident: $ty:ty),
		$($index:expr, $name:expr => $desc:expr,)*
	) => {
		$(#[$meta])*
		#[allow(dead_code)]
		#[derive(Copy, Clone)]
		$vis struct $Item(pub $ty);
		#[allow(dead_code)]
		impl $Item {
			/// Gets the code identifier for a flag value given the bit index.
			pub fn flag_str(index: u32) -> Option<&'static str> {
				match index {
					$($index => Some(stringify!($name)),)*
					_ => None,
				}
			}
			/// Gets the description for a flag value given the bit index.
			pub fn flag_desc(index: u32) -> Option<&'static str> {
				match index {
					$($index => Some($desc),)*
					_ => None,
				}
			}
			pub fn parse_flag(s: &str) -> Option<$ty> {
				match s {
					$(stringify!($name) => Some($name),)*
					_ => None,
				}
			}
			/// Returns an Iterator over set flag bits returning their code identifiers.
			pub fn to_strs(self) -> impl Clone + Iterator<Item = &'static str> {
				(0..mem::size_of::<$ty>() as u32 * 8)
					.filter_map(move |i| {
						if self.0 & (1 << i) != 0 {
							Self::flag_str(i)
						}
						else {
							None
						}
					})
			}
		}
	};
}

//----------------------------------------------------------------

enum1! {
	/// Stringifies the `IMAGE_FILE_MACHINE_*` constants for [`IMAGE_FILE_HEADER::Machine`].
	Machine(machine: u16),
	IMAGE_FILE_MACHINE_I386 => "i386",
	IMAGE_FILE_MACHINE_AMD64 => "AMD64",
	IMAGE_FILE_MACHINE_ARM64 => "ARM64",
	IMAGE_FILE_MACHINE_IA64 => "IA64",
}

flags! {
	/// Stringifies the `IMAGE_FILE_*` flag indices for [`IMAGE_FILE_HEADER::Characteristics`].
	pub FileChars(file_chars: u16),
	/*0001*/ 0, IMAGE_FILE_RELOCS_STRIPPED => "RELOCS_STRIPPED",
	/*0002*/ 1, IMAGE_FILE_EXECUTABLE_IMAGE => "EXECUTABLE_IMAGE",
	/*0004*/ 2, IMAGE_FILE_LINE_NUMS_STRIPPED => "LINE_NUMS_STRIPPED",
	/*0008*/ 3, IMAGE_FILE_LOCAL_SYMS_STRIPPED => "LOCAL_SYMS_STRIPPED",
	/*0010*/ 4, IMAGE_FILE_AGGRESIVE_WS_TRIM => "AGGRESIVE_WS_TRIM",
	/*0020*/ 5, IMAGE_FILE_LARGE_ADDRESS_AWARE => "LARGE_ADDRESS_AWARE",
	/*0040*/ 6, IMAGE_FILE_6 => "Reserved",
	/*0080*/ 7, IMAGE_FILE_BYTES_REVERSED_LO => "BYTES_REVERSED_LO",
	/*0100*/ 8, IMAGE_FILE_32BIT_MACHINE => "32BIT_MACHINE",
	/*0200*/ 9, IMAGE_FILE_DEBUG_STRIPPED => "DEBUG_STRIPPED",
	/*0400*/10, IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP => "REMOVABLE_RUN_FROM_SWAP",
	/*0800*/11, IMAGE_FILE_NET_RUN_FROM_SWAP => "NET_RUN_FROM_SWAP",
	/*1000*/12, IMAGE_FILE_SYSTEM => "SYSTEM",
	/*2000*/13, IMAGE_FILE_DLL => "DLL",
	/*4000*/14, IMAGE_FILE_UP_SYSTEM_ONLY => "UP_SYSTEM_ONLY",
	/*8000*/15, IMAGE_FILE_BYTES_REVERSED_HI => "BYTES_REVERSED_HI",
}

enum1! {
	/// Stringifies the optional header's `Magic` value.
	OptionalMagic(magic: u16),
	IMAGE_NT_OPTIONAL_HDR32_MAGIC => "PE32",
	IMAGE_NT_OPTIONAL_HDR64_MAGIC => "PE32+",
	IMAGE_ROM_OPTIONAL_HDR_MAGIC => "ROM",
}

enum1! {
	/// Stringifies the `IMAGE_SUBSYSTEM_*` constants for [`IMAGE_OPTIONAL_HEADER::Subsystem`][IMAGE_OPTIONAL_HEADER64::Subsystem].
	Subsystem(subsystem: u16),
	IMAGE_SUBSYSTEM_UNKNOWN => "Unknown",
	IMAGE_SUBSYSTEM_NATIVE => "Native",
	IMAGE_SUBSYSTEM_WINDOWS_GUI => "Windows GUI",
	IMAGE_SUBSYSTEM_WINDOWS_CUI => "Windows CUI",
	IMAGE_SUBSYSTEM_OS2_CUI => "OS/2 CUI",
	IMAGE_SUBSYSTEM_POSIX_CUI => "POSIX CUI",
	IMAGE_SUBSYSTEM_NATIVE_WINDOWS => "Native Win9x driver",
	IMAGE_SUBSYSTEM_WINDOWS_CE_GUI => "Windows CE GUI",
	IMAGE_SUBSYSTEM_EFI_APPLICATION => "Windows EFI Application",
	IMAGE_SUBSYSTEM_EFI_BOOT_SERVICE_DRIVER => "Windows EFI Boot Service Driver",
	IMAGE_SUBSYSTEM_EFI_RUNTIME_DRIVER => "Windows EFI Runtime Driver",
	IMAGE_SUBSYSTEM_EFI_ROM => "Windows EFI ROM",
	IMAGE_SUBSYSTEM_XBOX => "XBOX",
	IMAGE_SUBSYSTEM_WINDOWS_BOOT_APPLICATION => "Windows Boot Application",
}

flags! {
	/// Stringifies the `IMAGE_DLLCHARACTERISTICS_*` flag indices for [`IMAGE_OPTIONAL_HEADER::DllCharacteristics`][IMAGE_OPTIONAL_HEADER64::DllCharacteristics].
	pub DllChars(dll_chars: u16),
	/*0001*/ 0, IMAGE_DLLCHARACTERISTICS_0 => "Reserved",
	/*0002*/ 1, IMAGE_DLLCHARACTERISTICS_1 => "Reserved",
	/*0004*/ 2, IMAGE_DLLCHARACTERISTICS_2 => "Reserved",
	/*0008*/ 3, IMAGE_DLLCHARACTERISTICS_3 => "Reserved",
	/*0010*/ 4, IMAGE_DLLCHARACTERISTICS_4 => "Reserved",
	/*0020*/ 5, IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA => "Image can handle a high entropy virtual address space",
	/*0040*/ 6, IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE => "Can be relocated at load time",
	/*0080*/ 7, IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY => "Code Integrity checks are enforced",
	/*0100*/ 8, IMAGE_DLLCHARACTERISTICS_NX_COMPAT => "Image is NX compatible",
	/*0200*/ 9, IMAGE_DLLCHARACTERISTICS_NO_ISOLATION => "Isolation aware, but do not isolate the image",
	/*0400*/10, IMAGE_DLLCHARACTERISTICS_NO_SEH => "Does not use SEH",
	/*0800*/11, IMAGE_DLLCHARACTERISTICS_NO_BIND => "Do not bind the image",
	/*1000*/12, IMAGE_DLLCHARACTERISTICS_APPCONTAINER => "Image must execute in an AppContainer",
	/*2000*/13, IMAGE_DLLCHARACTERISTICS_WDM_DRIVER => "A WDM driver",
	/*4000*/14, IMAGE_DLLCHARACTERISTICS_GUARD_CF => "Image supports Control Flow Guard",
	/*8000*/15, IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE => "Terminal Server aware",
}

enum1! {
	/// Stringifies the `IMAGE_DIRECTORY_ENTRY_*` constants for [`IMAGE_OPTIONAL_HEADER::DataDirectory`][IMAGE_OPTIONAL_HEADER64::DataDirectory].
	DirectoryEntry(entry: usize),
	IMAGE_DIRECTORY_ENTRY_EXPORT => "Export Directory",
	IMAGE_DIRECTORY_ENTRY_IMPORT => "Import Directory",
	IMAGE_DIRECTORY_ENTRY_RESOURCE => "Resource Directory",
	IMAGE_DIRECTORY_ENTRY_EXCEPTION => "Exception Directory",
	IMAGE_DIRECTORY_ENTRY_SECURITY => "Security Directory",
	IMAGE_DIRECTORY_ENTRY_BASERELOC => "Base Relocation Table",
	IMAGE_DIRECTORY_ENTRY_DEBUG => "Debug Directory",
	IMAGE_DIRECTORY_ENTRY_ARCHITECTURE => "Architecture Specific Data",
	IMAGE_DIRECTORY_ENTRY_GLOBALPTR => "RVA of GlobalPtr",
	IMAGE_DIRECTORY_ENTRY_TLS => "TLS Directory",
	IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG => "Load Configuration Directory",
	IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT => "Bound Import Directory",
	IMAGE_DIRECTORY_ENTRY_IAT => "Import Address Table",
	IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT => "Delay Load Import Descriptors",
	IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR => "COM Runtime Descriptor",
}

flags! {
	/// Stringifies the single-bit `IMAGE_SCN_*` flags for [`IMAGE_SECTION_HEADER::Characteristics`].
	SectionFlag(section_flag: u32),
	/*00000001*/ 0, IMAGE_SCN_0 => "Reserved",
	/*00000002*/ 1, IMAGE_SCN_1 => "Reserved",
	/*00000004*/ 2, IMAGE_SCN_2 => "Reserved",
	/*00000008*/ 3, IMAGE_SCN_TYPE_NO_PAD => "TYPE_NO_PAD",
	/*00000010*/ 4, IMAGE_SCN_4 => "Reserved",
	/*00000020*/ 5, IMAGE_SCN_CNT_CODE => "Contains executable code",
	/*00000040*/ 6, IMAGE_SCN_CNT_INITIALIZED_DATA => "Contains initialized data",
	/*00000080*/ 7, IMAGE_SCN_CNT_UNINITIALIZED_DATA => "Contains uninitialized data",
	/*00000100*/ 8, IMAGE_SCN_LNK_OTHER => "LNK_OTHER",
	/*00000200*/ 9, IMAGE_SCN_LNK_INFO => "LNK_INFO",
	/*00000400*/10, IMAGE_SCN_10 => "Reserved",
	/*00000800*/11, IMAGE_SCN_LNK_REMOVE => "LNK_REMOVE",
	/*00001000*/12, IMAGE_SCN_LNK_COMDAT => "LNK_COMDAT",
	/*00002000*/13, IMAGE_SCN_13 => "Reserved",
	/*00004000*/14, IMAGE_SCN_NO_DEFER_SPEC_EXC => "NO_DEFER_SPEC_EXC",
	/*00008000*/15, IMAGE_SCN_GPREL => "GPREL",
	/*00010000*/16, IMAGE_SCN_16 => "Reserved",
	/*00020000*/17, IMAGE_SCN_MEM_PURGEABLE => "MEM_PURGEABLE",
	/*00040000*/18, IMAGE_SCN_MEM_LOCKED => "MEM_LOCKED",
	/*00080000*/19, IMAGE_SCN_MEM_PRELOAD => "MEM_PRELOAD",
	/*01000000*/24, IMAGE_SCN_LNK_NRELOC_OVFL => "LNK_NRELOC_OVFL",
	/*02000000*/25, IMAGE_SCN_MEM_DISCARDABLE => "MEM_DISCARDABLE",
	/*04000000*/26, IMAGE_SCN_MEM_NOT_CACHED => "MEM_NOT_CACHED",
	/*08000000*/27, IMAGE_SCN_MEM_NOT_PAGED => "MEM_NOT_PAGED",
	/*10000000*/28, IMAGE_SCN_MEM_SHARED => "MEM_SHARED",
	/*20000000*/29, IMAGE_SCN_MEM_EXECUTE => "MEM_EXECUTE",
	/*40000000*/30, IMAGE_SCN_MEM_READ => "MEM_READ",
	/*80000000*/31, IMAGE_SCN_MEM_WRITE => "MEM_WRITE",
}

enum1! {
	/// Stringifies the encoded `IMAGE_SCN_ALIGN_*BYTES` section alignment values.
	SectionAlignment(section_alignment: u32),
	IMAGE_SCN_ALIGN_1BYTES => "1-byte alignment",
	IMAGE_SCN_ALIGN_2BYTES => "2-byte alignment",
	IMAGE_SCN_ALIGN_4BYTES => "4-byte alignment",
	IMAGE_SCN_ALIGN_8BYTES => "8-byte alignment",
	IMAGE_SCN_ALIGN_16BYTES => "16-byte alignment",
	IMAGE_SCN_ALIGN_32BYTES => "32-byte alignment",
	IMAGE_SCN_ALIGN_64BYTES => "64-byte alignment",
	IMAGE_SCN_ALIGN_128BYTES => "128-byte alignment",
	IMAGE_SCN_ALIGN_256BYTES => "256-byte alignment",
	IMAGE_SCN_ALIGN_512BYTES => "512-byte alignment",
	IMAGE_SCN_ALIGN_1024BYTES => "1024-byte alignment",
	IMAGE_SCN_ALIGN_2048BYTES => "2048-byte alignment",
	IMAGE_SCN_ALIGN_4096BYTES => "4096-byte alignment",
	IMAGE_SCN_ALIGN_8192BYTES => "8192-byte alignment",
}

/// Stringifies the `IMAGE_SCN_*` values in [`IMAGE_SECTION_HEADER::Characteristics`].
#[derive(Copy, Clone)]
pub struct SectionChars(pub u32);

impl SectionChars {
	const ALIGN_MASK: u32 = 0x00f00000;

	/// Gets the code identifier for a single-bit flag given its bit index.
	///
	/// Indices 20 through 23 contain the encoded alignment field and therefore
	/// are not individual flags. Use [`SectionChars::alignment`] for that field.
	pub fn flag_str(index: u32) -> Option<&'static str> {
		SectionFlag::flag_str(index)
	}

	/// Gets the description for a single-bit flag given its bit index.
	pub fn flag_desc(index: u32) -> Option<&'static str> {
		SectionFlag::flag_desc(index)
	}

	/// Parses either a single-bit flag or an encoded alignment value.
	pub fn parse_flag(s: &str) -> Option<u32> {
		SectionFlag::parse_flag(s).or_else(|| s.parse::<SectionAlignment>().ok().map(|value| value.0))
	}

	/// Gets the encoded section alignment, if one is present and valid.
	pub fn alignment(self) -> Option<SectionAlignment> {
		let alignment = SectionAlignment(self.0 & Self::ALIGN_MASK);
		alignment.to_str().map(|_| alignment)
	}

	/// Returns an iterator over set flags and the encoded alignment field.
	pub fn to_strs(self) -> impl Clone + Iterator<Item = &'static str> {
		// Treat the four alignment bits as one field while retaining bit order.
		(0..29).filter_map(move |slot| {
			if slot == 20 {
				self.alignment().and_then(SectionAlignment::to_str)
			}
			else {
				let index = if slot < 20 { slot } else { slot + 3 };
				if self.0 & (1u32 << index) != 0 {
					SectionFlag::flag_str(index)
				}
				else {
					None
				}
			}
		})
	}
}

enum1! {
	/// Stringifies the `RT_*` constants for [`IMAGE_RESOURCE_DIRECTORY_ENTRY::Name`].
	ResourceName(name: u16),
	RT_CURSOR => "RT_CURSOR",
	RT_BITMAP => "RT_BITMAP",
	RT_ICON => "RT_ICON",
	RT_MENU => "RT_MENU",
	RT_DIALOG => "RT_DIALOG",
	RT_STRING => "RT_STRING",
	RT_FONTDIR => "RT_FONTDIR",
	RT_FONT => "RT_FONT",
	RT_ACCELERATOR => "RT_ACCELERATOR",
	RT_RCDATA => "RT_RCDATA",
	RT_MESSAGETABLE => "RT_MESSAGETABLE",
	RT_GROUP_CURSOR => "RT_GROUP_CURSOR",
	RT_GROUP_ICON => "RT_GROUP_ICON",
	RT_VERSION => "RT_VERSION",
	RT_DLGINCLUDE => "RT_DLGINCLUDE",
	RT_PLUGPLAY => "RT_PLUGPLAY",
	RT_VXD => "RT_VXD",
	RT_ANICURSOR => "RT_ANICURSOR",
	RT_ANIICON => "RT_ANIICON",
	RT_HTML => "RT_HTML",
	RT_MANIFEST => "RT_MANIFEST",
}

enum1! {
	/// Stringifies the `IMAGE_REL_BASED_*` constants for [`IMAGE_BASE_RELOCATION`] types.
	RelocType(reloc_type: u8),
	IMAGE_REL_BASED_ABSOLUTE => "ABSOLUTE",
	IMAGE_REL_BASED_HIGH => "HIGH",
	IMAGE_REL_BASED_LOW => "LOW",
	IMAGE_REL_BASED_HIGHLOW => "HIGHLOW",
	IMAGE_REL_BASED_HIGHADJ => "HIGHADJ",
	IMAGE_REL_BASED_MACHINE_SPECIFIC_5 => "MACHINE_SPECIFIC_5",
	IMAGE_REL_BASED_RESERVED => "RESERVED",
	IMAGE_REL_BASED_MACHINE_SPECIFIC_7 => "MACHINE_SPECIFIC_7",
	IMAGE_REL_BASED_MACHINE_SPECIFIC_8 => "MACHINE_SPECIFIC_8",
	IMAGE_REL_BASED_MACHINE_SPECIFIC_9 => "MACHINE_SPECIFIC_9",
	IMAGE_REL_BASED_DIR64 => "DIR64",
}

enum1! {
	/// Stringifies the `UWOP_*` constants for [`UNWIND_CODE`] operations.
	UnwindOp(unwind_op: u8),
	UWOP_PUSH_NONVOL => "push nonvol",
	UWOP_ALLOC_LARGE => "alloc large",
	UWOP_ALLOC_SMALL => "alloc small",
	UWOP_SET_FPREG => "set fpreg",
	UWOP_SAVE_NONVOL => "save nonvol",
	UWOP_SAVE_NONVOL_FAR => "save nonvol large",
	UWOP_SAVE_XMM128 => "save xmm128",
	UWOP_SAVE_XMM128_FAR => "save xmm128 far",
	UWOP_PUSH_MACHFRAME => "push machframe",
}

enum1! {
	/// Stringifies the `UNW_FLAG_*` constants for [`UNWIND_INFO`] flags.
	UnwindFlag(unwind_flag: u8),
	UNW_FLAG_NHANDLER => "NHANDLER",
	UNW_FLAG_EHANDLER => "EHANDLER",
	UNW_FLAG_UHANDLER => "UHANDLER",
	UNW_FLAG_FHANDLER => "FHANDLER",
	UNW_FLAG_CHAININFO => "CHAININFO",
}

enum1! {
	/// Stringifies the `IMAGE_DEBUG_TYPE_*` constants for [`IMAGE_DEBUG_DIRECTORY::Type`].
	DebugType(debug_type: u32),
	IMAGE_DEBUG_TYPE_UNKNOWN => "Unknown",
	IMAGE_DEBUG_TYPE_COFF => "COFF",
	IMAGE_DEBUG_TYPE_CODEVIEW => "CodeView",
	IMAGE_DEBUG_TYPE_FPO => "FPO",
	IMAGE_DEBUG_TYPE_MISC => "DBG",
	IMAGE_DEBUG_TYPE_EXCEPTION => "Exception",
	IMAGE_DEBUG_TYPE_FIXUP => "Fixup",
	IMAGE_DEBUG_TYPE_OMAP_TO_SRC => "OMAP to src",
	IMAGE_DEBUG_TYPE_OMAP_FROM_SRC => "OMAP from src",
	IMAGE_DEBUG_TYPE_BORLAND => "Borland",
	IMAGE_DEBUG_TYPE_RESERVED10 => "Reserved10",
	IMAGE_DEBUG_TYPE_CLSID => "CLSID",
	IMAGE_DEBUG_TYPE_VC_FEATURE => "VCFeature",
	IMAGE_DEBUG_TYPE_POGO => "POGO",
	IMAGE_DEBUG_TYPE_ILTCG => "ILTCG",
	IMAGE_DEBUG_TYPE_MPX => "MPX",
	IMAGE_DEBUG_TYPE_REPRO => "Repro",
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn section_alignment_round_trips() {
		let alignments = [
			IMAGE_SCN_ALIGN_1BYTES, IMAGE_SCN_ALIGN_2BYTES,
			IMAGE_SCN_ALIGN_4BYTES, IMAGE_SCN_ALIGN_8BYTES,
			IMAGE_SCN_ALIGN_16BYTES, IMAGE_SCN_ALIGN_32BYTES,
			IMAGE_SCN_ALIGN_64BYTES, IMAGE_SCN_ALIGN_128BYTES,
			IMAGE_SCN_ALIGN_256BYTES, IMAGE_SCN_ALIGN_512BYTES,
			IMAGE_SCN_ALIGN_1024BYTES, IMAGE_SCN_ALIGN_2048BYTES,
			IMAGE_SCN_ALIGN_4096BYTES, IMAGE_SCN_ALIGN_8192BYTES,
		];

		for value in alignments {
			let alignment = SectionAlignment(value);
			let name = alignment.to_str().unwrap();
			assert_eq!(name.parse::<SectionAlignment>().unwrap().0, value);
			assert_eq!(SectionChars::parse_flag(name), Some(value));
			assert_eq!(SectionChars(value).to_strs().collect::<Vec<_>>(), [name]);
		}
	}

	#[test]
	fn section_flags_include_alignment_once_and_in_order() {
		let chars = IMAGE_SCN_CNT_CODE | IMAGE_SCN_ALIGN_4BYTES | IMAGE_SCN_MEM_READ;
		assert_eq!(SectionChars(chars).to_strs().collect::<Vec<_>>(), [
			"IMAGE_SCN_CNT_CODE",
			"IMAGE_SCN_ALIGN_4BYTES",
			"IMAGE_SCN_MEM_READ",
		]);
		assert_eq!(SectionChars::flag_str(20), None);
	}

	#[test]
	fn relocation_types_cover_defined_values() {
		assert_eq!(RelocType(IMAGE_REL_BASED_RESERVED).to_str(), Some("IMAGE_REL_BASED_RESERVED"));
		assert_eq!(RelocType(IMAGE_REL_BASED_MACHINE_SPECIFIC_8).to_str(), Some("IMAGE_REL_BASED_MACHINE_SPECIFIC_8"));
	}
}
