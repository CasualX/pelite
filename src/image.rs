/*!
Image structures as defined by Windows.

No attempt is made to abstract over endianness.

Sources:

* [winnt.h](https://source.winehq.org/source/include/winnt.h), ctrl-f `IMAGE_DOS_HEADER`
* [ImageHlp Structures](https://msdn.microsoft.com/en-us/library/windows/desktop/ms680198.aspx)
* [Peering Inside the PE: A Tour of the Win32 Portable Executable File Format](https://msdn.microsoft.com/en-us/library/ms809762.aspx)

*/

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![cfg_attr(rustfmt, rustfmt::skip)]

use std::{fmt, mem};

use crate::Pod;

#[cfg(windows)]
extern "C" {
	#[cfg(target_env = "msvc")]
	static __ImageBase: IMAGE_DOS_HEADER;
	#[cfg(target_env = "gnu")]
	static __image_base__: IMAGE_DOS_HEADER;
}

/// Gets the base address of the module this code is linked with.
///
/// This uses a linker pseudovariable and is only available on windows targets.
#[cfg(all(windows, target_env = "msvc"))]
#[inline]
pub fn image_base() -> &'static IMAGE_DOS_HEADER {
	unsafe { &__ImageBase }
}
/// Gets the base address of the module this code is linked with.
///
/// This uses a linker pseudovariable and is only available on windows targets.
#[cfg(all(windows, target_env = "gnu", target_pointer_width = "64"))]
#[inline]
pub fn image_base() -> &'static IMAGE_DOS_HEADER {
	unsafe { &__image_base__ }
}
/// Gets the base address of the module this code is linked with.
///
/// When linked with GCC on 32-bit targets the linker pseudovariable is undefined?
/// Since this function isn't critical, panic instead of failing to link.
///
/// FIXME! I've been scouring the internet for a solution but no dice.
#[cfg(all(windows, target_env = "gnu", not(target_pointer_width = "64")))]
#[inline]
pub fn image_base() -> &'static IMAGE_DOS_HEADER {
	panic!("undefined reference to `__image_base__'")
}

#[cfg(all(windows, target_env = "gnu", not(target_pointer_width = "64")))]
#[allow(dead_code)]
pub(crate) const IMAGE_BASE_PANICS: bool = true;
#[cfg(not(all(windows, target_env = "gnu", not(target_pointer_width = "64"))))]
#[allow(dead_code)]
pub(crate) const IMAGE_BASE_PANICS: bool = false;

//----------------------------------------------------------------

pub const IMAGE_DOS_SIGNATURE: u16 = 0x5A4D;

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct IMAGE_DOS_HEADER {
	pub e_magic: u16,
	pub e_cblp: u16,
	pub e_cp: u16,
	pub e_crlc: u16,
	pub e_cparhdr: u16,
	pub e_minalloc: u16,
	pub e_maxalloc: u16,
	pub e_ss: u16,
	pub e_sp: u16,
	pub e_csum: u16,
	pub e_ip: u16,
	pub e_cs: u16,
	pub e_lfarlc: u16,
	pub e_ovno: u16,
	pub e_res: [u16; 4],
	pub e_oemid: u16,
	pub e_oeminfo: u16,
	pub e_res2: [u16; 10],
	pub e_lfanew: u32,
}

//----------------------------------------------------------------

pub const IMAGE_FILE_MACHINE_I386: u16  = 0x014c;
pub const IMAGE_FILE_MACHINE_IA64: u16  = 0x0200;
pub const IMAGE_FILE_MACHINE_AMD64: u16 = 0x8664;
pub const IMAGE_FILE_MACHINE_ARM64: u16 = 0xaa64;

pub const IMAGE_FILE_RELOCS_STRIPPED: u16         = 0x0001;
pub const IMAGE_FILE_EXECUTABLE_IMAGE: u16        = 0x0002;
pub const IMAGE_FILE_LINE_NUMS_STRIPPED: u16      = 0x0004;
pub const IMAGE_FILE_LOCAL_SYMS_STRIPPED: u16     = 0x0008;
pub const IMAGE_FILE_AGGRESIVE_WS_TRIM: u16       = 0x0010;
pub const IMAGE_FILE_LARGE_ADDRESS_AWARE: u16     = 0x0020;
pub(crate) const IMAGE_FILE_6: u16                = 0x0040;
pub const IMAGE_FILE_BYTES_REVERSED_LO: u16       = 0x0080;
pub const IMAGE_FILE_32BIT_MACHINE: u16           = 0x0100;
pub const IMAGE_FILE_DEBUG_STRIPPED: u16          = 0x0200;
pub const IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP: u16 = 0x0400;
pub const IMAGE_FILE_NET_RUN_FROM_SWAP: u16       = 0x0800;
pub const IMAGE_FILE_SYSTEM: u16                  = 0x1000;
pub const IMAGE_FILE_DLL: u16                     = 0x2000;
pub const IMAGE_FILE_UP_SYSTEM_ONLY: u16          = 0x4000;
pub const IMAGE_FILE_BYTES_REVERSED_HI: u16       = 0x8000;

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct IMAGE_FILE_HEADER {
	pub Machine: u16,
	pub NumberOfSections: u16,
	pub TimeDateStamp: u32,
	pub PointerToSymbolTable: u32,
	pub NumberOfSymbols: u32,
	pub SizeOfOptionalHeader: u16,
	pub Characteristics: u16,
}

//----------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct IMAGE_DATA_DIRECTORY {
	pub VirtualAddress: u32,
	pub Size: u32,
}

pub const IMAGE_DIRECTORY_ENTRY_EXPORT: usize         = 0;
pub const IMAGE_DIRECTORY_ENTRY_IMPORT: usize         = 1;
pub const IMAGE_DIRECTORY_ENTRY_RESOURCE: usize       = 2;
pub const IMAGE_DIRECTORY_ENTRY_EXCEPTION: usize      = 3;
pub const IMAGE_DIRECTORY_ENTRY_SECURITY: usize       = 4;
pub const IMAGE_DIRECTORY_ENTRY_BASERELOC: usize      = 5;
pub const IMAGE_DIRECTORY_ENTRY_DEBUG: usize          = 6;
pub const IMAGE_DIRECTORY_ENTRY_ARCHITECTURE: usize   = 7;
pub const IMAGE_DIRECTORY_ENTRY_GLOBALPTR: usize      = 8;
pub const IMAGE_DIRECTORY_ENTRY_TLS: usize            = 9;
pub const IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG: usize    = 10;
pub const IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT: usize   = 11;
pub const IMAGE_DIRECTORY_ENTRY_IAT: usize            = 12;
pub const IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT: usize   = 13;
pub const IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR: usize = 14;

pub const IMAGE_NUMBEROF_DIRECTORY_ENTRIES: usize   = 16;

//----------------------------------------------------------------

// Helper struct, makes serialization nicer
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(C)]
pub struct IMAGE_VERSION<T> {
	pub Major: T,
	pub Minor: T,
}
#[cfg(feature = "serde")]
impl<T: fmt::Display> serde::Serialize for IMAGE_VERSION<T> {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
		serializer.collect_str(self)
	}
}
impl<T: fmt::Display> fmt::Debug for IMAGE_VERSION<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "\"{}.{}\"", self.Major, self.Minor)
	}
}
impl<T: fmt::Display> fmt::Display for IMAGE_VERSION<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}.{}", self.Major, self.Minor)
	}
}

//----------------------------------------------------------------

pub const IMAGE_NT_OPTIONAL_HDR32_MAGIC: u16 = 0x10b;
pub const IMAGE_NT_OPTIONAL_HDR64_MAGIC: u16 = 0x20b;
pub const IMAGE_ROM_OPTIONAL_HDR_MAGIC: u16  = 0x107;

pub const IMAGE_SUBSYSTEM_UNKNOWN: u16                  = 0;
pub const IMAGE_SUBSYSTEM_NATIVE: u16                   = 1;
pub const IMAGE_SUBSYSTEM_WINDOWS_GUI: u16              = 2;
pub const IMAGE_SUBSYSTEM_WINDOWS_CUI: u16              = 3;
pub const IMAGE_SUBSYSTEM_OS2_CUI: u16                  = 5;
pub const IMAGE_SUBSYSTEM_POSIX_CUI: u16                = 7;
pub const IMAGE_SUBSYSTEM_NATIVE_WINDOWS: u16           = 8;
pub const IMAGE_SUBSYSTEM_WINDOWS_CE_GUI: u16           = 9;
pub const IMAGE_SUBSYSTEM_EFI_APPLICATION: u16          = 10;
pub const IMAGE_SUBSYSTEM_EFI_BOOT_SERVICE_DRIVER: u16  = 11;
pub const IMAGE_SUBSYSTEM_EFI_RUNTIME_DRIVER: u16       = 12;
pub const IMAGE_SUBSYSTEM_EFI_ROM: u16                  = 13;
pub const IMAGE_SUBSYSTEM_XBOX: u16                     = 14;
pub const IMAGE_SUBSYSTEM_WINDOWS_BOOT_APPLICATION: u16 = 16;

pub(crate) const IMAGE_DLLCHARACTERISTICS_0: u16              = 0x0001;
pub(crate) const IMAGE_DLLCHARACTERISTICS_1: u16              = 0x0002;
pub(crate) const IMAGE_DLLCHARACTERISTICS_2: u16              = 0x0004;
pub(crate) const IMAGE_DLLCHARACTERISTICS_3: u16              = 0x0008;
pub(crate) const IMAGE_DLLCHARACTERISTICS_4: u16              = 0x0010;
pub const IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA: u16       = 0x0020;
pub const IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE: u16          = 0x0040;
pub const IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY: u16       = 0x0080;
pub const IMAGE_DLLCHARACTERISTICS_NX_COMPAT: u16             = 0x0100;
pub const IMAGE_DLLCHARACTERISTICS_NO_ISOLATION: u16          = 0x0200;
pub const IMAGE_DLLCHARACTERISTICS_NO_SEH: u16                = 0x0400;
pub const IMAGE_DLLCHARACTERISTICS_NO_BIND: u16               = 0x0800;
pub const IMAGE_DLLCHARACTERISTICS_APPCONTAINER: u16          = 0x1000;
pub const IMAGE_DLLCHARACTERISTICS_WDM_DRIVER: u16            = 0x2000;
pub const IMAGE_DLLCHARACTERISTICS_GUARD_CF: u16              = 0x4000;
pub const IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE: u16 = 0x8000;

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct IMAGE_OPTIONAL_HEADER32 {
	pub Magic: u16,
	pub LinkerVersion: IMAGE_VERSION<u8>,
	pub SizeOfCode: u32,
	pub SizeOfInitializedData: u32,
	pub SizeOfUninitializedData: u32,
	pub AddressOfEntryPoint: u32,
	pub BaseOfCode: u32,
	pub BaseOfData: u32,
	pub ImageBase: u32,
	pub SectionAlignment: u32,
	pub FileAlignment: u32,
	pub OperatingSystemVersion: IMAGE_VERSION<u16>,
	pub ImageVersion: IMAGE_VERSION<u16>,
	pub SubsystemVersion: IMAGE_VERSION<u16>,
	pub Win32VersionValue: u32,
	pub SizeOfImage: u32,
	pub SizeOfHeaders: u32,
	pub CheckSum: u32,
	pub Subsystem: u16,
	pub DllCharacteristics: u16,
	pub SizeOfStackReserve: u32,
	pub SizeOfStackCommit: u32,
	pub SizeOfHeapReserve: u32,
	pub SizeOfHeapCommit: u32,
	pub LoaderFlags: u32,
	pub NumberOfRvaAndSizes: u32,
	#[cfg_attr(feature = "serde", serde(skip))]
	pub DataDirectory: [IMAGE_DATA_DIRECTORY; 0],
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct IMAGE_OPTIONAL_HEADER64 {
	pub Magic: u16,
	pub LinkerVersion: IMAGE_VERSION<u8>,
	pub SizeOfCode: u32,
	pub SizeOfInitializedData: u32,
	pub SizeOfUninitializedData: u32,
	pub AddressOfEntryPoint: u32,
	pub BaseOfCode: u32,
	pub ImageBase: u64,
	pub SectionAlignment: u32,
	pub FileAlignment: u32,
	pub OperatingSystemVersion: IMAGE_VERSION<u16>,
	pub ImageVersion: IMAGE_VERSION<u16>,
	pub SubsystemVersion: IMAGE_VERSION<u16>,
	pub Win32VersionValue: u32,
	pub SizeOfImage: u32,
	pub SizeOfHeaders: u32,
	pub CheckSum: u32,
	pub Subsystem: u16,
	pub DllCharacteristics: u16,
	pub SizeOfStackReserve: u64,
	pub SizeOfStackCommit: u64,
	pub SizeOfHeapReserve: u64,
	pub SizeOfHeapCommit: u64,
	pub LoaderFlags: u32,
	pub NumberOfRvaAndSizes: u32,
	#[cfg_attr(feature = "serde", serde(skip))]
	pub DataDirectory: [IMAGE_DATA_DIRECTORY; 0],
}

//----------------------------------------------------------------

pub const IMAGE_NT_HEADERS_SIGNATURE: u32 = 0x00004550;

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct IMAGE_NT_HEADERS32 {
	pub Signature: u32,
	pub FileHeader: IMAGE_FILE_HEADER,
	pub OptionalHeader: IMAGE_OPTIONAL_HEADER32,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct IMAGE_NT_HEADERS64 {
	pub Signature: u32,
	pub FileHeader: IMAGE_FILE_HEADER,
	pub OptionalHeader: IMAGE_OPTIONAL_HEADER64,
}

//----------------------------------------------------------------

pub const IMAGE_SIZEOF_SHORT_NAME: usize = 8;

pub(crate) const IMAGE_SCN_0: u32               = 0x00000001;
pub(crate) const IMAGE_SCN_1: u32               = 0x00000002;
pub(crate) const IMAGE_SCN_2: u32               = 0x00000004;
pub const IMAGE_SCN_TYPE_NO_PAD: u32            = 0x00000008;
pub(crate) const IMAGE_SCN_4: u32               = 0x00000010;
pub const IMAGE_SCN_CNT_CODE: u32               = 0x00000020;
pub const IMAGE_SCN_CNT_INITIALIZED_DATA: u32   = 0x00000040;
pub const IMAGE_SCN_CNT_UNINITIALIZED_DATA: u32 = 0x00000080;
pub const IMAGE_SCN_LNK_OTHER: u32              = 0x00000100;
pub const IMAGE_SCN_LNK_INFO: u32               = 0x00000200;
pub(crate) const IMAGE_SCN_10: u32              = 0x00000400;
pub const IMAGE_SCN_LNK_REMOVE: u32             = 0x00000800;
pub const IMAGE_SCN_LNK_COMDAT: u32             = 0x00001000;
pub(crate) const IMAGE_SCN_13: u32              = 0x00002000;
pub const IMAGE_SCN_NO_DEFER_SPEC_EXC: u32      = 0x00004000;
pub const IMAGE_SCN_GPREL: u32                  = 0x00008000;
pub(crate) const IMAGE_SCN_16: u32               = 0x00010000;
pub const IMAGE_SCN_MEM_PURGEABLE: u32          = 0x00020000;
pub const IMAGE_SCN_MEM_LOCKED: u32             = 0x00040000;
pub const IMAGE_SCN_MEM_PRELOAD: u32            = 0x00080000;
pub(crate) const IMAGE_SCN_ALIGN_1: u32         = 0x00100000;
pub(crate) const IMAGE_SCN_ALIGN_2: u32         = 0x00200000;
pub(crate) const IMAGE_SCN_ALIGN_4: u32         = 0x00400000;
pub(crate) const IMAGE_SCN_ALIGN_8: u32         = 0x00800000;
pub const IMAGE_SCN_ALIGN_1BYTES: u32           = 0x00100000;
pub const IMAGE_SCN_ALIGN_2BYTES: u32           = 0x00200000;
pub const IMAGE_SCN_ALIGN_4BYTES: u32           = 0x00300000;
pub const IMAGE_SCN_ALIGN_8BYTES: u32           = 0x00400000;
pub const IMAGE_SCN_ALIGN_16BYTES: u32          = 0x00500000;
pub const IMAGE_SCN_ALIGN_32BYTES: u32          = 0x00600000;
pub const IMAGE_SCN_ALIGN_64BYTES: u32          = 0x00700000;
pub const IMAGE_SCN_ALIGN_128BYTES: u32         = 0x00800000;
pub const IMAGE_SCN_ALIGN_256BYTES: u32         = 0x00900000;
pub const IMAGE_SCN_ALIGN_512BYTES: u32         = 0x00A00000;
pub const IMAGE_SCN_ALIGN_1024BYTES: u32        = 0x00B00000;
pub const IMAGE_SCN_ALIGN_2048BYTES: u32        = 0x00C00000;
pub const IMAGE_SCN_ALIGN_4096BYTES: u32        = 0x00D00000;
pub const IMAGE_SCN_ALIGN_8192BYTES: u32        = 0x00E00000;
pub const IMAGE_SCN_LNK_NRELOC_OVFL: u32        = 0x01000000;
pub const IMAGE_SCN_MEM_DISCARDABLE: u32        = 0x02000000;
pub const IMAGE_SCN_MEM_NOT_CACHED: u32         = 0x04000000;
pub const IMAGE_SCN_MEM_NOT_PAGED: u32          = 0x08000000;
pub const IMAGE_SCN_MEM_SHARED: u32             = 0x10000000;
pub const IMAGE_SCN_MEM_EXECUTE: u32            = 0x20000000;
pub const IMAGE_SCN_MEM_READ: u32               = 0x40000000;
pub const IMAGE_SCN_MEM_WRITE: u32              = 0x80000000;

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct IMAGE_SECTION_HEADER {
	#[cfg_attr(feature = "serde", serde(serialize_with = "crate::wrap::sections::serialize_name"))]
	pub Name: [u8; IMAGE_SIZEOF_SHORT_NAME],
	pub VirtualSize: u32,
	pub VirtualAddress: u32,
	pub SizeOfRawData: u32,
	pub PointerToRawData: u32,
	pub PointerToRelocations: u32,
	pub PointerToLinenumbers: u32,
	pub NumberOfRelocations: u16,
	pub NumberOfLinenumbers: u16,
	pub Characteristics: u32,
}

//----------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct IMAGE_EXPORT_DIRECTORY {
	pub Characteristics: u32,
	pub TimeDateStamp: u32,
	pub Version: IMAGE_VERSION<u16>,
	pub Name: u32,
	pub Base: u32,
	pub NumberOfFunctions: u32,
	pub NumberOfNames: u32,
	pub AddressOfFunctions: u32,    // RVA from base of image
	pub AddressOfNames: u32,        // RVA from base of image
	pub AddressOfNameOrdinals: u32, // RVA from base of image
}

//----------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct IMAGE_IMPORT_DESCRIPTOR {
	pub OriginalFirstThunk: u32,
	pub TimeDateStamp: u32,
	pub ForwarderChain: u32,
	pub Name: u32,
	pub FirstThunk: u32,
}
impl IMAGE_IMPORT_DESCRIPTOR {
	pub fn is_null(&self) -> bool {
		// This is all that really marks an empty import descriptor
		self.FirstThunk == 0
	}
}

pub const IMAGE_ORDINAL_FLAG32: u32 = 0x80000000;
pub const IMAGE_ORDINAL_FLAG64: u64 = 0x8000000000000000;

//----------------------------------------------------------------

pub const RT_CURSOR: u16       = 1;
pub const RT_BITMAP: u16       = 2;
pub const RT_ICON: u16         = 3;
pub const RT_MENU: u16         = 4;
pub const RT_DIALOG: u16       = 5;
pub const RT_STRING: u16       = 6;
pub const RT_FONTDIR: u16      = 7;
pub const RT_FONT: u16         = 8;
pub const RT_ACCELERATOR: u16  = 9;
pub const RT_RCDATA: u16       = 10;
pub const RT_MESSAGETABLE: u16 = 11;
pub const RT_GROUP_CURSOR: u16 = 12;
pub const RT_GROUP_ICON: u16   = 14;
pub const RT_VERSION: u16      = 16;
pub const RT_DLGINCLUDE: u16   = 17;
pub const RT_PLUGPLAY: u16     = 19;
pub const RT_VXD: u16          = 20;
pub const RT_ANICURSOR: u16    = 21;
pub const RT_ANIICON: u16      = 22;
pub const RT_HTML: u16         = 23;
pub const RT_MANIFEST: u16     = 24;

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct IMAGE_RESOURCE_DIRECTORY {
	pub Characteristics: u32,
	pub TimeDateStamp: u32,
	pub Version: IMAGE_VERSION<u16>,
	pub NumberOfNamedEntries: u16,
	pub NumberOfIdEntries: u16,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct IMAGE_RESOURCE_DIRECTORY_ENTRY {
	// High bit set means the lower 31 bits are an RVA to its name string otherwise this is a 16 bit WORD id
	// Name string is encoded in WORDs and is prefixed with a WORD indicating its length (in WORDs)
	pub Name: u32,
	// High bit set means this is offset points to an IMAGE_RESOURCE_DIRECTORY otherwise an IMAGE_RESOURCE_DATA_ENTRY
	pub Offset: u32,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct IMAGE_RESOURCE_DATA_ENTRY {
	pub OffsetToData: u32,
	pub Size: u32,
	pub CodePage: u32,
	pub Reserved: u32,
}

//----------------------------------------------------------------

pub const VS_FIXEDFILEINFO_SIGNATURE: u32 = 0xFEEF04BD;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(C)]
pub struct VS_VERSION {
	pub Minor: u16,
	pub Major: u16,
	pub Build: u16,
	pub Patch: u16,
}
#[cfg(feature = "serde")]
impl serde::Serialize for VS_VERSION {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
		serializer.collect_str(self)
	}
}
impl fmt::Debug for VS_VERSION {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "\"{}.{}.{}.{}\"", self.Major, self.Minor, self.Patch, self.Build)
	}
}
impl fmt::Display for VS_VERSION {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}.{}.{}.{}", self.Major, self.Minor, self.Patch, self.Build)
	}
}

#[cfg(feature = "serde")]
fn ser_fixed_file_info_struc_version<S: serde::Serializer>(&version: &u32, serializer: S) -> std::result::Result<S::Ok, S::Error> {
	let version = IMAGE_VERSION {
		Major: (version >> 16) as u16,
		Minor: (version & 0xffff) as u16,
	};
	serializer.collect_str(&version)
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct VS_FIXEDFILEINFO {
	pub dwSignature: u32,
	#[cfg_attr(feature = "serde", serde(serialize_with = "ser_fixed_file_info_struc_version"))]
	pub dwStrucVersion: u32,
	pub dwFileVersion: VS_VERSION,
	pub dwProductVersion: VS_VERSION,
	pub dwFileFlagsMask: u32,
	pub dwFileFlags: u32,
	pub dwFileOS: u32,
	pub dwFileType: u32,
	pub dwFileSubtype: u32,
	pub dwFileDateMS: u32,
	pub dwFileDateLS: u32,
}

//----------------------------------------------------------------
// Sourced from <Windows.h>

pub const IMAGE_REL_BASED_ABSOLUTE: u8 = 0;
pub const IMAGE_REL_BASED_HIGH: u8 = 1;
pub const IMAGE_REL_BASED_LOW: u8 = 2;
pub const IMAGE_REL_BASED_HIGHLOW: u8 = 3;
pub const IMAGE_REL_BASED_HIGHADJ: u8 = 4;
pub const IMAGE_REL_BASED_MACHINE_SPECIFIC_5: u8 = 5;
pub const IMAGE_REL_BASED_RESERVED: u8 = 6;
pub const IMAGE_REL_BASED_MACHINE_SPECIFIC_7: u8 = 7;
pub const IMAGE_REL_BASED_MACHINE_SPECIFIC_8: u8 = 8;
pub const IMAGE_REL_BASED_MACHINE_SPECIFIC_9: u8 = 9;
pub const IMAGE_REL_BASED_DIR64: u8 = 10;

pub const IMAGE_REL_BASED_IA64IMM64: u8 = 9;

pub const IMAGE_REL_BASED_MIPSJMPADDR: u8 = 5;
pub const IMAGE_REL_BASED_MIPSJMPADDR16: u8 = 9;

pub const IMAGE_REL_BASED_ARM_MOV32: u8 = 5;
pub const IMAGE_REL_BASED_THUMB_MOV32: u8 = 7;

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct IMAGE_BASE_RELOCATION {
	pub VirtualAddress: u32,
	pub SizeOfBlock: u32,
}

//----------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct IMAGE_LOAD_CONFIG_DIRECTORY32 {
	pub Size: u32,
	pub TimeDateStamp: u32,
	pub Version: IMAGE_VERSION<u16>,
	pub GlobalFlagsClear: u32,
	pub GlobalFlagsSet: u32,
	pub CriticalSectionDefaultTimeout: u32,
	pub DeCommitFreeBlockThreshold: u32,
	pub DeCommitTotalFreeThreshold: u32,
	pub LockPrefixTable: u32,
	pub MaximumAllocationSize: u32,
	pub VirtualMemoryThreshold: u32,
	pub ProcessAffinityMask: u32,
	pub ProcessHeapFlags: u32,
	pub CSDVersion: u16,
	pub DependentLoadFlags: u16,
	pub EditList: u32,
	pub SecurityCookie: u32,
	pub SEHandlerTable: u32,
	pub SEHandlerCount: u32,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct IMAGE_LOAD_CONFIG_DIRECTORY64 {
	pub Size: u32,
	pub TimeDateStamp: u32,
	pub Version: IMAGE_VERSION<u16>,
	pub GlobalFlagsClear: u32,
	pub GlobalFlagsSet: u32,
	pub CriticalSectionDefaultTimeout: u32,
	pub DeCommitFreeBlockThreshold: u64,
	pub DeCommitTotalFreeThreshold: u64,
	pub LockPrefixTable: u64,
	pub MaximumAllocationSize: u64,
	pub VirtualMemoryThreshold: u64,
	pub ProcessAffinityMask: u64,
	pub ProcessHeapFlags: u32,
	pub CSDVersion: u16,
	pub DependentLoadFlags: u16,
	pub EditList: u64,
	pub SecurityCookie: u64,
	pub SEHandlerTable: u64,
	pub SEHandlerCount: u64,
}

//----------------------------------------------------------------
// Control flow guard bits of the LoadConfig
//
// Note that IMAGE_GUARDCF is not an official name but to keep it separate from IMAGE_LOAD_CONFIG.
//
// References:
// https://docs.microsoft.com/en-us/windows/desktop/api/winnt/ns-winnt-_image_load_config_directory32
// https://docs.microsoft.com/en-us/windows/desktop/api/winnt/ns-winnt-_image_load_config_directory64
// https://redplait.blogspot.com/2016/10/imageloadconfigdirectory-from-sdk-14951.html
// https://lucasg.github.io/2017/02/05/Control-Flow-Guard/

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct IMAGE_LOAD_CONFIG_CODE_INTEGRITY {
	pub Flags: u16,
	pub Catalog: u16,
	pub CatalogOffset: u32,
	pub Reserved: u32,
}

pub const IMAGE_DYNAMIC_RELOCATION_GUARD_RF_PROLOGUE: u32 = 0x00000001;
pub const IMAGE_DYNAMIC_RELOCATION_GUARD_RF_EPILOGUE: u32 = 0x00000002;

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct IMAGE_DYNAMIC_RELOCATION_TABLE {
	pub Version: u32,
	pub Size: u32,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct IMAGE_DYNAMIC_RELOCATION32 {
	pub Symbol: u32,
	pub BaseRelocSize: u32,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct IMAGE_DYNAMIC_RELOCATION64 {
	pub Symbol: [u32; 2], // 64-bit packed int
	pub BaseRelocSize: u32,
}

pub const IMAGE_GUARD_CF_INSTRUMENTED: u32                    = 0x00000100;
pub const IMAGE_GUARD_CFW_INSTRUMENTED: u32                   = 0x00000200;
pub const IMAGE_GUARD_CF_FUNCTION_TABLE_PRESENT: u32          = 0x00000400;
pub const IMAGE_GUARD_SECURITY_COOKIE_UNUSED: u32             = 0x00000800;
pub const IMAGE_GUARD_PROTECT_DELAYLOAD_IAT: u32              = 0x00001000;
pub const IMAGE_GUARD_DELAYLOAD_IAT_IN_ITS_OWN_SECTION: u32   = 0x00002000;
pub const IMAGE_GUARD_CF_EXPORT_SUPPRESSION_INFO_PRESENT: u32 = 0x00004000;
pub const IMAGE_GUARD_CF_ENABLE_EXPORT_SUPPRESSION: u32       = 0x00008000;
pub const IMAGE_GUARD_CF_LONGJUMP_TABLE_PRESENT: u32          = 0x00010000;
pub const IMAGE_GUARD_RF_INSTRUMENTED: u32                    = 0x00020000;
pub const IMAGE_GUARD_RF_ENABLE: u32                          = 0x00040000;
pub const IMAGE_GUARD_RF_STRICT: u32                          = 0x00080000;
pub const IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_MASK: u32        = 0xF0000000;
pub const IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_SHIFT: u32       = 28;

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct IMAGE_GUARDCF32 {
	pub GuardCFCheckFunctionPointer: u32,
	pub GuardCFDispatchFunctionPointer: u32,
	pub GuardCFFunctionTable: u32,
	pub GuardCFFunctionCount: u32,
	pub GuardFlags: u32,
	pub CodeIntegrity: IMAGE_LOAD_CONFIG_CODE_INTEGRITY,
	pub GuardAddressTakenIatEntryTable: u32,
	pub GuardAddressTakenIatEntryCount: u32,
	pub GuardLongJumpTargetTable: u32,
	pub GuardLongJumpTargetCount: u32,
	pub DynamicValueRelocTable: u32,
	pub HybridMetadataPointer: u32,
	pub GuardRFFailureRoutine: u32,
	pub GuardRFFailureRoutineFunctionPointer: u32,
	pub DynamicValueRelocTableOffset: u32,
	pub DynamicValueRelocTableSection: u16,
	pub Reserved2: u16,
	pub GuardRFVerifyStackPointerFunctionPointer: u32,
	pub HotPatchTableOffset: u32,
	pub Reserved3: u32,
	pub EnclaveConfigurationPointer: u32,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct IMAGE_GUARDCF64 {
	pub GuardCFCheckFunctionPointer: u64,
	pub GuardCFDispatchFunctionPointer: u64,
	pub GuardCFFunctionTable: u64,
	pub GuardCFFunctionCount: u64,
	pub GuardFlags: u32,
	pub CodeIntegrity: IMAGE_LOAD_CONFIG_CODE_INTEGRITY,
	pub GuardAddressTakenIatEntryTable: u64,
	pub GuardAddressTakenIatEntryCount: u64,
	pub GuardLongJumpTargetTable: u64,
	pub GuardLongJumpTargetCount: u64,
	pub DynamicValueRelocTable: u64,
	pub HybridMetadataPointer: u64,
	pub GuardRFFailureRoutine: u64,
	pub GuardRFFailureRoutineFunctionPointer: u64,
	pub DynamicValueRelocTableOffset: u32,
	pub DynamicValueRelocTableSection: u16,
	pub Reserved2: u16,
	pub GuardRFVerifyStackPointerFunctionPointer: u64,
	pub HotPatchTableOffset: u32,
	pub Reserved3: u32,
	pub EnclaveConfigurationPointer: u64,
}

//----------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct IMAGE_TLS_DIRECTORY32 {
	pub StartAddressOfRawData: u32,
	pub EndAddressOfRawData: u32,
	pub AddressOfIndex: u32,
	pub AddressOfCallBacks: u32,
	pub SizeOfZeroFill: u32,
	pub Characteristics: u32,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct IMAGE_TLS_DIRECTORY64 {
	pub StartAddressOfRawData: u64,
	pub EndAddressOfRawData: u64,
	pub AddressOfIndex: u64,
	pub AddressOfCallBacks: u64,
	pub SizeOfZeroFill: u32,
	pub Characteristics: u32,
}

//----------------------------------------------------------------

pub const WIN_CERT_REVISION_1_0: u16 = 0x0100;

pub const WIN_CERT_TYPE_X509: u16             = 0x0001;
pub const WIN_CERT_TYPE_PKCS_SIGNED_DATA: u16 = 0x0002;
pub const WIN_CERT_TYPE_RESERVED_1: u16       = 0x0003;
pub const WIN_CERT_TYPE_PKCS1_SIGN: u16       = 0x0009;

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct WIN_CERTIFICATE {
	pub dwLength: u32,
	pub wRevision: u16,
	pub wCertificateType: u16,
	#[cfg_attr(feature = "serde", serde(skip))]
	pub bCertificate: [u8; 0],
}

//----------------------------------------------------------------
// References
// https://docs.microsoft.com/en-gb/cpp/build/unwind-data-definitions-in-c
// https://www.reactos.org/wiki/Techwiki:SEH64
// http://www.osronline.com/article.cfm?article=469

pub const UWOP_PUSH_NONVOL: u8     = 0;  // info == register number
pub const UWOP_ALLOC_LARGE: u8     = 1;  // no info, alloc size in next 2 slots
pub const UWOP_ALLOC_SMALL: u8     = 2;  // info == size of allocation / 8 - 1
pub const UWOP_SET_FPREG: u8       = 3;  // no info, FP = RSP + UNWIND_INFO.FPRegOffset*16
pub const UWOP_SAVE_NONVOL: u8     = 4;  // info == register number, offset in next slot
pub const UWOP_SAVE_NONVOL_FAR: u8 = 5;  // info == register number, offset in next 2 slots
pub const UWOP_SAVE_XMM128: u8     = 8;  // info == XMM reg number, offset in next slot
pub const UWOP_SAVE_XMM128_FAR: u8 = 9;  // info == XMM reg number, offset in next 2 slots
pub const UWOP_PUSH_MACHFRAME: u8  = 10; // info == 0: no error-code, 1: error-code

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct UNWIND_CODE {
	pub CodeOffset: u8,
	pub UnwindOpInfo: u8,
}

pub const UNW_FLAG_NHANDLER: u8  = 0x00;
pub const UNW_FLAG_EHANDLER: u8  = 0x01;
pub const UNW_FLAG_UHANDLER: u8  = 0x02;
pub const UNW_FLAG_FHANDLER: u8  = 0x03; // inofficial
pub const UNW_FLAG_CHAININFO: u8 = 0x04;

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct UNWIND_INFO {
	pub VersionFlags: u8,
	pub SizeOfProlog: u8,
	pub CountOfCodes: u8,
	pub FrameRegisterOffset: u8,
	#[cfg_attr(feature = "serde", serde(skip))]
	pub UnwindCode: [UNWIND_CODE; 0],
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct RUNTIME_FUNCTION {
	pub BeginAddress: u32,
	pub EndAddress: u32,
	pub UnwindData: u32,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY {
	pub BeginAddress: u32,
	pub UnwindData: u32,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct SCOPE_RECORD {
	pub BeginAddress: u32,
	pub EndAddress: u32,
	pub HandlerAddress: u32,
	pub JumpTarget: u32,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct SCOPE_TABLE {
	pub Count: u32,
	#[cfg_attr(feature = "serde", serde(skip))]
	pub ScopeRecord: [SCOPE_RECORD; 0],
}

//----------------------------------------------------------------
// http://www.debuginfo.com/articles/debuginfomatch.html
// https://docs.microsoft.com/en-us/windows/desktop/debug/pe-format#the-debug-section

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct GUID {
	pub Data1: u32,
	pub Data2: u16,
	pub Data3: u16,
	pub Data4: [u8; 8],
}

pub const IMAGE_DEBUG_TYPE_UNKNOWN: u32       = 0;
pub const IMAGE_DEBUG_TYPE_COFF: u32          = 1;
pub const IMAGE_DEBUG_TYPE_CODEVIEW: u32      = 2;
pub const IMAGE_DEBUG_TYPE_FPO: u32           = 3;
pub const IMAGE_DEBUG_TYPE_MISC: u32          = 4;
pub const IMAGE_DEBUG_TYPE_EXCEPTION: u32     = 5;
pub const IMAGE_DEBUG_TYPE_FIXUP: u32         = 6;
pub const IMAGE_DEBUG_TYPE_OMAP_TO_SRC: u32   = 7;
pub const IMAGE_DEBUG_TYPE_OMAP_FROM_SRC: u32 = 8;
pub const IMAGE_DEBUG_TYPE_BORLAND: u32       = 9;
pub const IMAGE_DEBUG_TYPE_RESERVED10: u32    = 10;
pub const IMAGE_DEBUG_TYPE_CLSID: u32         = 11;
pub const IMAGE_DEBUG_TYPE_VC_FEATURE: u32    = 12;
pub const IMAGE_DEBUG_TYPE_POGO: u32          = 13;
pub const IMAGE_DEBUG_TYPE_ILTCG: u32         = 14;
pub const IMAGE_DEBUG_TYPE_MPX: u32           = 15;
pub const IMAGE_DEBUG_TYPE_REPRO: u32         = 16;

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct IMAGE_DEBUG_DIRECTORY {
	pub Characteristics: u32,
	pub TimeDateStamp: u32,
	pub Version: IMAGE_VERSION<u16>,
	pub Type: u32,
	pub SizeOfData: u32,
	pub AddressOfRawData: u32,
	pub PointerToRawData: u32,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct IMAGE_DEBUG_CV_INFO_PDB20 {
	pub CvSignature: u32,
	pub Offset: u32,
	pub TimeDateStamp: u32,
	pub Age: u32,
	#[cfg_attr(feature = "serde", serde(skip))]
	pub PdbFileName: [u8; 0],
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct IMAGE_DEBUG_CV_INFO_PDB70 {
	pub CvSignature: u32,
	pub Signature: GUID,
	pub Age: u32,
	#[cfg_attr(feature = "serde", serde(skip))]
	pub PdbFileName: [u8; 0],
}

pub const IMAGE_DEBUG_MISC_EXENAME: u32 = 1;

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct IMAGE_DEBUG_MISC {
	pub DataType: u32,
	pub Length: u32,
	pub Unicode: u8,
	#[cfg_attr(feature = "serde", serde(skip))]
	pub Reserved: [u8; 3],
	#[cfg_attr(feature = "serde", serde(skip))]
	pub Name: [u8; 0],
}

pub const FRAME_FPO: u16 = 0;
pub const FRAME_TRAP: u16 = 1;
pub const FRAME_TSS: u16 = 2;

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct FPO_DATA {
	// offset 1st byte of function code
	pub ulOffStart: u32,
	// # bytes in function
	pub cbProcSize: u32,
	// # bytes in locals/4
	pub cdwLocals: u16,
	// # bytes in params/4
	pub cdwParams: u16,
	// # bytes in prolog
	pub cbProlog: u8,
	// cbRegs   : 3  // # regs saved
	// fHasSEH  : 1  // TRUE if SEH in func
	// fUseBP   : 1  // TRUE if EBP has been allocated
	// reserved : 1  // reserved for future use
	// cbFrame  : 2  // frame type
	pub fFlags: u8,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct IMAGE_DEBUG_VC_FEATURE {
	pub PreVCPP: u32, // Pre-VC++ 11.00
	pub CCPP: u32,    // C/C++
	pub GS: u32,      // /GS
	pub SDL: u32,     // /sdl
	pub GuardN: u32,  // guardN
}

//----------------------------------------------------------------

unsafe impl Pod for IMAGE_DOS_HEADER {}
unsafe impl Pod for IMAGE_FILE_HEADER {}
unsafe impl Pod for IMAGE_DATA_DIRECTORY {}
unsafe impl<T: Pod> Pod for IMAGE_VERSION<T> {}
unsafe impl Pod for IMAGE_OPTIONAL_HEADER32 {}
unsafe impl Pod for IMAGE_OPTIONAL_HEADER64 {}
unsafe impl Pod for IMAGE_NT_HEADERS32 {}
unsafe impl Pod for IMAGE_NT_HEADERS64 {}
unsafe impl Pod for IMAGE_SECTION_HEADER {}
unsafe impl Pod for IMAGE_EXPORT_DIRECTORY {}
unsafe impl Pod for IMAGE_IMPORT_DESCRIPTOR {}
unsafe impl Pod for IMAGE_RESOURCE_DIRECTORY {}
unsafe impl Pod for IMAGE_RESOURCE_DIRECTORY_ENTRY {}
unsafe impl Pod for IMAGE_RESOURCE_DATA_ENTRY {}
unsafe impl Pod for VS_VERSION {}
unsafe impl Pod for VS_FIXEDFILEINFO {}
unsafe impl Pod for IMAGE_BASE_RELOCATION {}
unsafe impl Pod for IMAGE_LOAD_CONFIG_DIRECTORY32 {}
unsafe impl Pod for IMAGE_LOAD_CONFIG_DIRECTORY64 {}
unsafe impl Pod for IMAGE_LOAD_CONFIG_CODE_INTEGRITY {}
unsafe impl Pod for IMAGE_DYNAMIC_RELOCATION_TABLE {}
unsafe impl Pod for IMAGE_DYNAMIC_RELOCATION32 {}
unsafe impl Pod for IMAGE_DYNAMIC_RELOCATION64 {}
unsafe impl Pod for IMAGE_GUARDCF32 {}
unsafe impl Pod for IMAGE_GUARDCF64 {}
unsafe impl Pod for IMAGE_TLS_DIRECTORY32 {}
unsafe impl Pod for IMAGE_TLS_DIRECTORY64 {}
unsafe impl Pod for WIN_CERTIFICATE {}
unsafe impl Pod for UNWIND_CODE {}
unsafe impl Pod for UNWIND_INFO {}
unsafe impl Pod for RUNTIME_FUNCTION {}
unsafe impl Pod for IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY {}
unsafe impl Pod for SCOPE_RECORD {}
unsafe impl Pod for SCOPE_TABLE {}
unsafe impl Pod for GUID {}
unsafe impl Pod for IMAGE_DEBUG_DIRECTORY {}
unsafe impl Pod for IMAGE_DEBUG_CV_INFO_PDB20 {}
unsafe impl Pod for IMAGE_DEBUG_CV_INFO_PDB70 {}
unsafe impl Pod for IMAGE_DEBUG_MISC {}
unsafe impl Pod for FPO_DATA {}
unsafe impl Pod for IMAGE_DEBUG_VC_FEATURE {}

//----------------------------------------------------------------

const _: [(); 64] = [(); mem::size_of::<IMAGE_DOS_HEADER>()];
const _: [(); 20] = [(); mem::size_of::<IMAGE_FILE_HEADER>()];
const _: [(); 8] = [(); mem::size_of::<IMAGE_DATA_DIRECTORY>()];
const _: [(); 96] = [(); mem::size_of::<IMAGE_OPTIONAL_HEADER32>()]; // Unsized
const _: [(); 112] = [(); mem::size_of::<IMAGE_OPTIONAL_HEADER64>()]; // Unsized
const _: [(); 120] = [(); mem::size_of::<IMAGE_NT_HEADERS32>()]; // Unsized
const _: [(); 136] = [(); mem::size_of::<IMAGE_NT_HEADERS64>()]; // Unsized
const _: [(); 40] = [(); mem::size_of::<IMAGE_SECTION_HEADER>()];
const _: [(); 40] = [(); mem::size_of::<IMAGE_EXPORT_DIRECTORY>()];
const _: [(); 20] = [(); mem::size_of::<IMAGE_IMPORT_DESCRIPTOR>()];
const _: [(); 16] = [(); mem::size_of::<IMAGE_RESOURCE_DIRECTORY>()];
const _: [(); 8] = [(); mem::size_of::<IMAGE_RESOURCE_DIRECTORY_ENTRY>()];
const _: [(); 16] = [(); mem::size_of::<IMAGE_RESOURCE_DATA_ENTRY>()];
const _: [(); 8] = [(); mem::size_of::<IMAGE_BASE_RELOCATION>()];
const _: [(); 18 * 4] = [(); mem::size_of::<IMAGE_LOAD_CONFIG_DIRECTORY32>()];
const _: [(); 28 * 4] = [(); mem::size_of::<IMAGE_LOAD_CONFIG_DIRECTORY64>()];
const _: [(); 3 * 4] = [(); mem::size_of::<IMAGE_LOAD_CONFIG_CODE_INTEGRITY>()];
const _: [(); 2 * 4] = [(); mem::size_of::<IMAGE_DYNAMIC_RELOCATION_TABLE>()]; // Unsized
const _: [(); 2 * 4] = [(); mem::size_of::<IMAGE_DYNAMIC_RELOCATION32>()]; // Unsized
const _: [(); 3 * 4] = [(); mem::size_of::<IMAGE_DYNAMIC_RELOCATION64>()]; // Unsized
const _: [(); 22 * 4] = [(); mem::size_of::<IMAGE_GUARDCF32>()];
const _: [(); 36 * 4] = [(); mem::size_of::<IMAGE_GUARDCF64>()];
const _: [(); 24] = [(); mem::size_of::<IMAGE_TLS_DIRECTORY32>()];
const _: [(); 40] = [(); mem::size_of::<IMAGE_TLS_DIRECTORY64>()];
const _: [(); 2] = [(); mem::size_of::<UNWIND_CODE>()];
const _: [(); 4] = [(); mem::size_of::<UNWIND_INFO>()]; // Unsized
const _: [(); 12] = [(); mem::size_of::<RUNTIME_FUNCTION>()];
const _: [(); 8] = [(); mem::size_of::<IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY>()];
const _: [(); 4] = [(); mem::size_of::<SCOPE_TABLE>()]; // Unsized
const _: [(); 16] = [(); mem::size_of::<SCOPE_RECORD>()];
const _: [(); 16] = [(); mem::size_of::<GUID>()];
const _: [(); 28] = [(); mem::size_of::<IMAGE_DEBUG_DIRECTORY>()];
const _: [(); 16] = [(); mem::size_of::<IMAGE_DEBUG_CV_INFO_PDB20>()]; // Unsized
const _: [(); 24] = [(); mem::size_of::<IMAGE_DEBUG_CV_INFO_PDB70>()]; // Unsized
const _: [(); 12] = [(); mem::size_of::<IMAGE_DEBUG_MISC>()]; // Unsized
