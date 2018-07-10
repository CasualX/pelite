/*!
Stringify image constants.
*/

use image::*;

/// Stringifies the `IMAGE_FILE_MACHINE_*` constants for [`IMAGE_FILE_HEADER::Machine`](../image/struct.IMAGE_FILE_HEADER.html#Machine.v).
///
/// # Examples
///
/// ```
/// let machine = pelite::image::IMAGE_FILE_MACHINE_AMD64;
///
/// assert_eq!(pelite::stringify::machine(machine), Some("AMD64"));
/// ```
pub fn machine(machine: u16) -> Option<&'static str> {
	match machine {
		IMAGE_FILE_MACHINE_I386 => Some("i386"),
		IMAGE_FILE_MACHINE_AMD64 => Some("AMD64"),
		IMAGE_FILE_MACHINE_IA64 => Some("IA64"),
		_ => None,
	}
}

/// Stringifies the `IMAGE_FILE_*` flag indices for [`IMAGE_FILE_HEADER::Characteristics`](../image/struct.IMAGE_FILE_HEADER.html#Characteristics.v).
///
/// # Examples
///
/// ```
/// let file_chars =
/// 	pelite::image::IMAGE_FILE_DLL |
/// 	pelite::image::IMAGE_FILE_LARGE_ADDRESS_AWARE;
///
/// let flags = (0..16)
/// 	.filter(|&index| file_chars & (1 << index) != 0)
/// 	.map(pelite::stringify::file_chars)
/// 	.collect::<Vec<Option<&str>>>();
///
/// assert_eq!(flags, &[
/// 	Some("LARGE_ADDRESS_AWARE"),
/// 	Some("DLL"),
/// ]);
/// ```
pub fn file_chars(index: u32) -> Option<&'static str> {
	IMAGE_FILE_CHARS_STRINGS.get(index as usize).and_then(Clone::clone)
}
static IMAGE_FILE_CHARS_STRINGS: [Option<&str>; 16] = [
	/*0001*/Some("RELOCS_STRIPPED"),
	/*0002*/Some("EXECUTABLE_IMAGE"),
	/*0004*/Some("LINE_NUMS_STRIPPED"),
	/*0008*/Some("LOCAL_SYMS_STRIPPED"),
	/*0010*/Some("AGGRESIVE_WS_TRIM"),
	/*0020*/Some("LARGE_ADDRESS_AWARE"),
	/*0040*/None,
	/*0080*/Some("BYTES_REVERSED_LO"),
	/*0100*/Some("32BIT_MACHINE"),
	/*0200*/Some("DEBUG_STRIPPED"),
	/*0400*/Some("REMOVABLE_RUN_FROM_SWAP"),
	/*0800*/Some("NET_RUN_FROM_SWAP"),
	/*1000*/Some("SYSTEM"),
	/*2000*/Some("DLL"),
	/*4000*/Some("UP_SYSTEM_ONLY"),
	/*8000*/Some("BYTES_REVERSED_HI"),
];

/// Stringifies the optional header's `Magic` value.
///
/// # Examples
///
/// ```
/// let magic = pelite::image::IMAGE_NT_OPTIONAL_HDR64_MAGIC;
///
/// assert_eq!(pelite::stringify::optional_magic(magic), Some("PE32+"));
/// ```
pub fn optional_magic(magic: u16) -> Option<&'static str> {
	match magic {
		IMAGE_NT_OPTIONAL_HDR32_MAGIC => Some("PE32"),
		IMAGE_NT_OPTIONAL_HDR64_MAGIC => Some("PE32+"),
		IMAGE_ROM_OPTIONAL_HDR_MAGIC => Some("ROM"),
		_ => None,
	}
}

/// Stringifies the `IMAGE_SUBSYSTEM_*` constants for [`IMAGE_OPTIONAL_HEADER::Subsystem`](../image/struct.IMAGE_OPTIONAL_HEADER64.html#Subsystem.v).
///
/// # Examples
///
/// ```
/// let subsystem = pelite::image::IMAGE_SUBSYSTEM_WINDOWS_GUI;
///
/// assert_eq!(pelite::stringify::subsystem(subsystem), Some("Windows GUI"));
/// ```
pub fn subsystem(subsystem: u16) -> Option<&'static str> {
	match subsystem {
		IMAGE_SUBSYSTEM_UNKNOWN => Some("Unknown"),
		IMAGE_SUBSYSTEM_NATIVE => Some("Native"),
		IMAGE_SUBSYSTEM_WINDOWS_GUI => Some("Windows GUI"),
		IMAGE_SUBSYSTEM_WINDOWS_CUI => Some("Windows CUI"),
		IMAGE_SUBSYSTEM_OS2_CUI => Some("OS2 CUI"),
		IMAGE_SUBSYSTEM_POSIX_CUI => Some("POSIX CUI"),
		IMAGE_SUBSYSTEM_WINDOWS_CE_GUI => Some("Windows CE GUI"),
		IMAGE_SUBSYSTEM_EFI_APPLICATION => Some("Windows EFI Application"),
		IMAGE_SUBSYSTEM_EFI_BOOT_SERVICE_DRIVER => Some("Windows EFI Boot Service Driver"),
		IMAGE_SUBSYSTEM_EFI_RUNTIME_DRIVER => Some("Windows EFI Runtime Driver"),
		IMAGE_SUBSYSTEM_EFI_ROM => Some("Windows EFI ROM"),
		IMAGE_SUBSYSTEM_XBOX => Some("XBOX"),
		IMAGE_SUBSYSTEM_WINDOWS_BOOT_APPLICATION => Some("Windows Boot Application"),
		_ => None,
	}
}

/// Stringifies the `IMAGE_DLLCHARACTERISTICS_*` flag indices for [`IMAGE_OPTIONAL_HEADER::DllCharacteristics`](../image/struct.IMAGE_OPTIONAL_HEADER64.html#DllCharacteristics.v).
///
/// # Examples
///
/// ```
/// let dll_chars =
/// 	pelite::image::IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE |
/// 	pelite::image::IMAGE_DLLCHARACTERISTICS_NX_COMPAT |
/// 	pelite::image::IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE;
///
/// let flags = (0..16)
/// 	.filter(|&index| dll_chars & (1 << index) != 0)
/// 	.map(pelite::stringify::dll_chars)
/// 	.collect::<Vec<Option<&str>>>();
///
/// assert_eq!(flags, &[
/// 	Some("Dynamic Base"),
/// 	Some("NX Compat"),
/// 	Some("Terminal Server Aware"),
/// ]);
/// ```
pub fn dll_chars(index: u32) -> Option<&'static str> {
	IMAGE_DLLCHARS_STRINGS.get(index as usize).and_then(Clone::clone)
}
static IMAGE_DLLCHARS_STRINGS: [Option<&str>; 16] = [
	/*0001*/None,
	/*0002*/None,
	/*0004*/None,
	/*0008*/None,
	/*0010*/None,
	/*0020*/Some("High Entropy VA"),
	/*0040*/Some("Dynamic Base"),
	/*0080*/Some("Force Integrity"),
	/*0100*/Some("NX Compat"),
	/*0200*/Some("No Isolation"),
	/*0400*/Some("No SEH"),
	/*0800*/Some("No Bind"),
	/*1000*/Some("AppContainer"),
	/*2000*/Some("WDM Driver"),
	/*4000*/Some("Guard CF"),
	/*8000*/Some("Terminal Server Aware"),
];

/// Stringifies the `IMAGE_DIRECTORY_ENTRY_*` constants for [`IMAGE_OPTIONAL_HEADER::DataDirectory`](../image/struct.IMAGE_OPTIONAL_HEADER64.html#DataDirectory.v).
///
/// # Examples
///
/// ```
/// let directory_entry = pelite::image::IMAGE_DIRECTORY_ENTRY_IMPORT;
///
/// assert_eq!(pelite::stringify::directory_entry(directory_entry), Some("Import"));
/// ```
pub fn directory_entry(entry: usize) -> Option<&'static str> {
	match entry {
		IMAGE_DIRECTORY_ENTRY_EXPORT => Some("Export"),
		IMAGE_DIRECTORY_ENTRY_IMPORT => Some("Import"),
		IMAGE_DIRECTORY_ENTRY_RESOURCE => Some("Resource"),
		IMAGE_DIRECTORY_ENTRY_EXCEPTION => Some("Exception"),
		IMAGE_DIRECTORY_ENTRY_SECURITY => Some("Security"),
		IMAGE_DIRECTORY_ENTRY_BASERELOC => Some("BaseReloc"),
		IMAGE_DIRECTORY_ENTRY_DEBUG => Some("Debug"),
		IMAGE_DIRECTORY_ENTRY_ARCHITECTURE => Some("Architecture"),
		IMAGE_DIRECTORY_ENTRY_GLOBALPTR => Some("GlobalPtr"),
		IMAGE_DIRECTORY_ENTRY_TLS => Some("TLS"),
		IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG => Some("Load Config"),
		IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT => Some("Bound Import"),
		IMAGE_DIRECTORY_ENTRY_IAT => Some("IAT"),
		IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT => Some("Delay Import"),
		IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR => Some("COM Descriptor"),
		_ => None,
	}
}

/// Stringifies the `IMAGE_SCN_*` flag indices for [`IMAGE_SECTION_HEADER::Characteristics`](../image/struct.IMAGE_SECTION_HEADER.html#Characteristics.v).
///
/// # Examples
///
/// ```
/// let section_chars =
/// 	pelite::image::IMAGE_SCN_CNT_CODE |
/// 	pelite::image::IMAGE_SCN_MEM_EXECUTE |
/// 	pelite::image::IMAGE_SCN_MEM_READ;
///
/// let flags = (0..32)
/// 	.filter(|&index| section_chars & (1 << index) != 0)
/// 	.map(pelite::stringify::section_chars)
/// 	.collect::<Vec<Option<&str>>>();
///
/// assert_eq!(flags, &[
/// 	Some("CNT_CODE"),
/// 	Some("MEM_EXECUTE"),
/// 	Some("MEM_READ"),
/// ]);
/// ```
pub fn section_chars(index: u32) -> Option<&'static str> {
	IMAGE_SCN_STRINGS.get(index as usize).and_then(Clone::clone)
}
static IMAGE_SCN_STRINGS: [Option<&str>; 32] = [
	/*00000001*/None,
	/*00000002*/None,
	/*00000004*/None,
	/*00000008*/Some("TYPE_NO_PAD"),
	/*00000010*/None,
	/*00000020*/Some("CNT_CODE"),
	/*00000040*/Some("CNT_INITIALIZED_DATA"),
	/*00000080*/Some("CNT_UNINITIALIZED_DATA"),
	/*00000100*/Some("LNK_OTHER"),
	/*00000200*/Some("LNK_INFO"),
	/*00000400*/None,
	/*00000800*/Some("LNK_REMOVE"),
	/*00001000*/Some("LNK_COMDAT"),
	/*00002000*/None,
	/*00004000*/Some("NO_DEFER_SPEC_EXC"),
	/*00008000*/Some("GPREL"),
	/*00010000*/None,
	/*00020000*/Some("MEM_PURGEABLE"),
	/*00040000*/Some("MEM_LOCKED"),
	/*00080000*/Some("MEM_PRELOAD"),
	/*00100000*/None,
	/*00200000*/None,
	/*00400000*/None,
	/*00800000*/None,
	/*01000000*/Some("LNK_NRELOC_OVFL"),
	/*02000000*/Some("MEM_DISCARDABLE"),
	/*04000000*/Some("MEM_NOT_CACHED"),
	/*08000000*/Some("MEM_NOT_PAGED"),
	/*10000000*/Some("MEM_SHARED"),
	/*20000000*/Some("MEM_EXECUTE"),
	/*40000000*/Some("MEM_READ"),
	/*80000000*/Some("MEM_WRITE"),
];

/// Stringifies the `RT_*` constants for [`IMAGE_RESOURCE_DIRECTORY_ENTRY::Name`](../image/struct.IMAGE_RESOURCE_DIRECTORY_ENTRY.html#Name.v).
///
/// # Examples
///
/// ```
/// let name = pelite::image::RT_MANIFEST;
///
/// assert_eq!(pelite::stringify::rsrc_name(name), Some("Manifest"));
/// ```
pub fn rsrc_name(name: u16) -> Option<&'static str> {
	RSRC_TYPES.get(name as usize).and_then(Clone::clone)
}
pub(crate) static RSRC_TYPES: [Option<&str>; 25] = [
	/* 0*/ None, Some("Cursors"), Some("Bitmaps"), Some("Icons"), Some("Menus"),
	/* 5*/ Some("Dialogs"), Some("Strings"), Some("Font Directory"), Some("Fonts"), Some("Accelerators"),
	/*10*/ Some("Raw Data"), Some("Message Tables"), Some("Group Cursors"), None, Some("Group Icons"),
	/*15*/ None, Some("Version"), Some("DlgInclude"), None, Some("Plug and Play"),
	/*20*/ Some("VXD"), Some("Animated Cursors"), Some("Animated Icons"), Some("HTML"), Some("Manifest"),
];

/// Stringifies the `IMAGE_REL_BASED_*` constants for [`IMAGE_BASE_RELOCATION` types](../image/struct.IMAGE_BASE_RELOCATION.html).
///
/// # Examples
///
/// ```
/// let reloc_type = pelite::image::IMAGE_REL_BASED_HIGHLOW;
///
/// assert_eq!(pelite::stringify::reloc_type(reloc_type), Some("HIGHLOW"));
/// ```
pub fn reloc_type(reloc_type: u8) -> Option<&'static str> {
	match reloc_type {
		IMAGE_REL_BASED_ABSOLUTE => Some("ABSOLUTE"),
		IMAGE_REL_BASED_HIGH => Some("HIGH"),
		IMAGE_REL_BASED_LOW => Some("LOW"),
		IMAGE_REL_BASED_HIGHLOW => Some("HIGHLOW"),
		IMAGE_REL_BASED_HIGHADJ => Some("HIGHADJ"),
		IMAGE_REL_BASED_MACHINE_SPECIFIC_5 => Some("MACHINE_SPECIFIC_5"),
		IMAGE_REL_BASED_MACHINE_SPECIFIC_7 => Some("MACHINE_SPECIFIC_7"),
		IMAGE_REL_BASED_MACHINE_SPECIFIC_9 => Some("MACHINE_SPECIFIC_9"),
		IMAGE_REL_BASED_DIR64 => Some("DIR64"),
		_ => None,
	}
}

/// Stringifies the `IMAGE_DEBUG_TYPE_*` constants for [`IMAGE_DEBUG_DIRECTORY::Type`](../image/struct.IMAGE_DEBUG_DIRECTORY.html#Type.v).
///
/// # Examples
///
/// ```
/// let debug_type = pelite::image::IMAGE_DEBUG_TYPE_CODEVIEW;
///
/// assert_eq!(pelite::stringify::debug_type(debug_type), Some("CodeView"));
/// ```
pub fn debug_type(debug_type: u32) -> Option<&'static str> {
	match debug_type {
		IMAGE_DEBUG_TYPE_UNKNOWN => Some("Unknown"),
		IMAGE_DEBUG_TYPE_COFF => Some("COFF"),
		IMAGE_DEBUG_TYPE_CODEVIEW => Some("CodeView"),
		IMAGE_DEBUG_TYPE_FPO => Some("FPO"),
		IMAGE_DEBUG_TYPE_MISC => Some("DBG"),
		IMAGE_DEBUG_TYPE_EXCEPTION => Some("Exception"),
		IMAGE_DEBUG_TYPE_FIXUP => Some("Fixup"),
		IMAGE_DEBUG_TYPE_OMAP_TO_SRC => Some("OMAP to src"),
		IMAGE_DEBUG_TYPE_OMAP_FROM_SRC => Some("OMAP from src"),
		IMAGE_DEBUG_TYPE_BORLAND => Some("Borland"),
		IMAGE_DEBUG_TYPE_CLSID => Some("CLSID"),
		_ => None,
	}
}
