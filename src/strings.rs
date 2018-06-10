/*!
Formatting as far as the eye can see...
*/

use std::{fmt, mem};

use image::*;
use util::*;

//----------------------------------------------------------------

pub fn stringify<T>(data: &T) -> Option<&str> {
	let bytes = unsafe {
		::std::slice::from_raw_parts(
			data as *const T as *const u8,
			mem::size_of_val(data),
		)
	};
	::std::str::from_utf8(strn(bytes)).ok()
}

//----------------------------------------------------------------

// Intersperse formatting strings with their arguments
macro_rules! pretty {
	(@ [$($istr:expr,)*] $exprs:tt [# $s:expr, $($tail:tt)*]) => {
		pretty!(@ [$($istr,)* $s,] $exprs [$($tail)*]);
	};
	(@ [$($istr:expr,)*] $exprs:tt [# $s:expr]) => {
		pretty!(@ [$($istr,)* $s,] $exprs []);
	};
	(@ $istrs:tt [$($arg:expr,)*] [$e:expr, $($tail:tt)*]) => {
		pretty!(@ $istrs [$($arg,)* $e,] [$($tail)*]);
	};
	(@ $istrs:tt [$($arg:expr,)*] [$e:expr]) => {
		pretty!(@ $istrs [$($arg,)* $e,] []);
	};
	(@ [$($istr:expr,)*] [$($e:expr,)*] []) => {
		format_args!(concat!($($istr),*), $($e),* )
	};
	(# $istr:expr, $($tail:tt)*) => {
		pretty!(@ [$istr,] [] [$($tail)*]);
	};
}
// Convenience :)
macro_rules! write {
	($f:expr, # $($tail:tt)*) => {
		$f.write_fmt(pretty!(# $($tail)*))
	};
	($f:expr, $($tail:tt)*) => {
		$f.write_fmt(format_args!($($tail)*))
	};
}

pub struct Fmt<F: Fn(&mut fmt::Formatter) -> fmt::Result>(pub F);
impl<F: Fn(&mut fmt::Formatter) -> fmt::Result> fmt::Display for Fmt<F> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		(self.0)(f)
	}
}

//----------------------------------------------------------------

impl fmt::Debug for IMAGE_DOS_HEADER {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
			#"MZ Header\n",
			#"  e_magic:    {:·>4X}{}\n", self.e_magic, Fmt(|f| stringify(&self.e_magic).map(|e_magic| write!(f, ": {}", e_magic)).unwrap_or(Ok(()))),
			#"  e_cblp:     {:·>4X}\n", self.e_cblp,
			#"  e_cp:       {:·>4X}\n", self.e_cp,
			#"  e_crlc:     {:·>4X}\n", self.e_crlc,
			#"  e_cparhdr:  {:·>4X}\n", self.e_cparhdr,
			#"  e_minalloc: {:·>4X}\n", self.e_minalloc,
			#"  e_maxalloc: {:·>4X}\n", self.e_maxalloc,
			#"  e_ss:       {:·>4X}\n", self.e_ss,
			#"  e_sp:       {:·>4X}\n", self.e_sp,
			#"  e_csum:     {:·>4X}\n", self.e_csum,
			#"  e_ip:       {:·>4X}\n", self.e_ip,
			#"  e_cs:       {:·>4X}\n", self.e_cs,
			#"  e_lfarlc:   {:·>4X}\n", self.e_lfarlc,
			#"  e_ovno:     {:·>4X}\n", self.e_ovno,
			#"  e_oemid:    {:·>4X}\n", self.e_oemid,
			#"  e_oeminfo:  {:·>4X}\n", self.e_oeminfo,
			#"  e_lfanew:   {:·>8X}\n", self.e_lfanew,
		)
	}
}

//----------------------------------------------------------------

fn stringify_machine(machine: u16) -> Option<&'static str> {
	match machine {
		IMAGE_FILE_MACHINE_I386 => Some("i386"),
		IMAGE_FILE_MACHINE_AMD64 => Some("AMD64"),
		IMAGE_FILE_MACHINE_IA64 => Some("IA64"),
		_ => None,
	}
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
impl fmt::Debug for IMAGE_FILE_HEADER {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
			#"File Header\n",
			#"  Machine:                 {:·>4X}{}\n", self.Machine, Fmt(|f| stringify_machine(self.Machine).map(|machine| write!(f, ": {}", machine)).unwrap_or(Ok(()))),
			#"  NumberOfSections:        {}\n", self.NumberOfSections,
			#"  TimeDateStamp:           {}\n", self.TimeDateStamp,
			#"  PointerToSymbolTable:    {:·>8X}\n", self.PointerToSymbolTable,
			#"  NumberOfSymbols:         {}\n", self.NumberOfSymbols,
			#"  SizeOfOptionalHeader:    {:·>4X}\n", self.SizeOfOptionalHeader,
			#"  Characteristics:         {:·>4X}", self.Characteristics,
			#"{}\n", Fmt(|f| {
				for i in 0u16..16u16 {
					if (1 << i) & self.Characteristics != 0 {
						write!(f, "\n    {:·>4X}", 1 << i)?;
						if let Some(chars) = IMAGE_FILE_CHARS_STRINGS[i as usize] {
							write!(f, ": {}", chars)?;
						}
					}
				}
				Ok(())
			}),
		)
	}
}

//----------------------------------------------------------------

fn stringify_optional_magic(magic: u16) -> Option<&'static str> {
	match magic {
		IMAGE_NT_OPTIONAL_HDR32_MAGIC => Some("PE32"),
		IMAGE_NT_OPTIONAL_HDR64_MAGIC => Some("PE32+"),
		IMAGE_ROM_OPTIONAL_HDR_MAGIC => Some("ROM"),
		_ => None,
	}
}
fn stringify_subsystem(subsystem: u16) -> Option<&'static str> {
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
fn stringify_datadir_entry(entry: usize) -> Option<&'static str> {
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
impl fmt::Debug for IMAGE_OPTIONAL_HEADER32 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
			#"Optional Header\n",
			#"  Magic:                   {:·>4X}{}\n", self.Magic, Fmt(|f| {
				if let Some(magic) = stringify_optional_magic(self.Magic) {
					write!(f, ": {}", magic)?;
				}
				Ok(())
			}),
			#"  LinkerVersion:           {}.{}\n", self.MajorLinkerVersion, self.MinorLinkerVersion,
			#"  SizeOfCode:              {:·>8X}\n", self.SizeOfCode,
			#"  SizeOfInitializedData:   {:·>8X}\n", self.SizeOfInitializedData,
			#"  SizeOfUninitializedData: {:·>8X}\n", self.SizeOfUninitializedData,
			#"  AddressOfEntryPoint:     {:·>8X}\n", self.AddressOfEntryPoint,
			#"  BaseOfCode:              {:·>8X}\n", self.BaseOfCode,
			#"  BaseOfData:              {:·>8X}\n", self.BaseOfData,
			#"  ImageBase:               {:·>8X}\n", self.ImageBase,
			#"  SectionAlignment:        {:·>8X}\n", self.SectionAlignment,
			#"  FileAlignment:           {:·>8X}\n", self.FileAlignment,
			#"  OperatingSystemVersion:  {}.{}\n", self.MajorOperatingSystemVersion, self.MinorOperatingSystemVersion,
			#"  ImageVersion:            {}.{}\n", self.MajorImageVersion, self.MinorImageVersion,
			#"  SubsystemVersion:        {}.{}\n", self.MajorSubsystemVersion, self.MinorSubsystemVersion,
			#"  Win32VersionValue:       {:·>8X}\n", self.Win32VersionValue,
			#"  SizeOfImage:             {:·>8X}\n", self.SizeOfImage,
			#"  SizeOfHeaders:           {:·>8X}\n", self.SizeOfHeaders,
			#"  CheckSum:                {:·>8X}\n", self.CheckSum,
			#"  Subsystem:               {:·>4X}{}\n", self.Subsystem, Fmt(|f| stringify_subsystem(self.Subsystem).map(|subsystem| write!(f, ": {}", subsystem)).unwrap_or(Ok(()))),
			#"  DllCharacteristics:      {:·>4X}", self.DllCharacteristics,
			#"{}\n", Fmt(|f| {
				for i in 0u16..16u16 {
					if (1 << i) & self.DllCharacteristics != 0 {
						write!(f, "\n    {:·>4X}", 1 << i)?;
						if let Some(dllchars) = IMAGE_DLLCHARS_STRINGS[i as usize] {
							write!(f, ": {}", dllchars)?;
						}
					}
				}
				Ok(())
			}),
			#"  SizeOfStackReserve:      {:·>8X}\n", self.SizeOfStackReserve,
			#"  SizeOfStackCommit:       {:·>8X}\n", self.SizeOfStackCommit,
			#"  SizeOfHeapReserve:       {:·>8X}\n", self.SizeOfHeapReserve,
			#"  SizeOfHeapCommit:        {:·>8X}\n", self.SizeOfHeapCommit,
			#"  LoaderFlags:             {:·>8X}\n", self.LoaderFlags,
			#"  NumberOfRvaAndSizes:     {:·>8X}", self.NumberOfRvaAndSizes,
			#"{}\n", Fmt(|f| {
				for i in 0u32..::std::cmp::min(self.NumberOfRvaAndSizes, IMAGE_NUMBEROF_DIRECTORY_ENTRIES as u32) {
					let datadir = &self.DataDirectory[i as usize];
					if datadir.VirtualAddress != 0 && datadir.Size != 0 {
						write!(f, "\n    {:>2} [{:·>8X} +{:·>8X}]", i, datadir.VirtualAddress, datadir.Size)?;
						if let Some(entry) = stringify_datadir_entry(i as usize) {
							write!(f, ": {}", entry)?;
						}
					}
				}
				Ok(())
			}),
		)
	}
}
impl fmt::Debug for IMAGE_OPTIONAL_HEADER64 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
			#"Optional Header\n",
			#"  Magic:                   {:·>4X}{}\n", self.Magic, Fmt(|f| stringify_optional_magic(self.Magic).map(|magic| write!(f, ": {}", magic)).unwrap_or(Ok(()))),
			#"  LinkerVersion:           {}.{}\n", self.MajorLinkerVersion, self.MinorLinkerVersion,
			#"  SizeOfCode:              {:·>8X}\n", self.SizeOfCode,
			#"  SizeOfInitializedData:   {:·>8X}\n", self.SizeOfInitializedData,
			#"  SizeOfUninitializedData: {:·>8X}\n", self.SizeOfUninitializedData,
			#"  AddressOfEntryPoint:     {:·>8X}\n", self.AddressOfEntryPoint,
			#"  BaseOfCode:              {:·>8X}\n", self.BaseOfCode,
			#"  ImageBase:               {:·>16X}\n", self.ImageBase,
			#"  SectionAlignment:        {:·>8X}\n", self.SectionAlignment,
			#"  FileAlignment:           {:·>8X}\n", self.FileAlignment,
			#"  OperatingSystemVersion:  {}.{}\n", self.MajorOperatingSystemVersion, self.MinorOperatingSystemVersion,
			#"  ImageVersion:            {}.{}\n", self.MajorImageVersion, self.MinorImageVersion,
			#"  SubsystemVersion:        {}.{}\n", self.MajorSubsystemVersion, self.MinorSubsystemVersion,
			#"  Win32VersionValue:       {:·>8X}\n", self.Win32VersionValue,
			#"  SizeOfImage:             {:·>8X}\n", self.SizeOfImage,
			#"  SizeOfHeaders:           {:·>8X}\n", self.SizeOfHeaders,
			#"  CheckSum:                {:·>8X}\n", self.CheckSum,
			#"  Subsystem:               {:·>4X}{}\n", self.Subsystem, Fmt(|f| stringify_subsystem(self.Subsystem).map(|subsystem| write!(f, ": {}", subsystem)).unwrap_or(Ok(()))),
			#"  DllCharacteristics:      {:·>4X}", self.DllCharacteristics,
			#"{}\n", Fmt(|f| {
				for i in 0u16..16u16 {
					if (1 << i) & self.DllCharacteristics != 0 {
						write!(f, "\n    {:·>4X}", 1 << i)?;
						if let Some(dllchars) = IMAGE_DLLCHARS_STRINGS[i as usize] {
							write!(f, ": {}", dllchars)?;
						}
					}
				}
				Ok(())
			}),
			#"  SizeOfStackReserve:      {:·>16X}\n", self.SizeOfStackReserve,
			#"  SizeOfStackCommit:       {:·>16X}\n", self.SizeOfStackCommit,
			#"  SizeOfHeapReserve:       {:·>16X}\n", self.SizeOfHeapReserve,
			#"  SizeOfHeapCommit:        {:·>16X}\n", self.SizeOfHeapCommit,
			#"  LoaderFlags:             {:·>8X}\n", self.LoaderFlags,
			#"  NumberOfRvaAndSizes:     {:·>8X}", self.NumberOfRvaAndSizes,
			#"{}\n", Fmt(|f| {
				for i in 0u32..::std::cmp::min(self.NumberOfRvaAndSizes, IMAGE_NUMBEROF_DIRECTORY_ENTRIES as u32) {
					let datadir = &self.DataDirectory[i as usize];
					if datadir.VirtualAddress != 0 && datadir.Size != 0 {
						write!(f, "\n    {:>2} [RVA:{:·>8X} Size:{:·>8X}]", i, datadir.VirtualAddress, datadir.Size)?;
						if let Some(entry) = stringify_datadir_entry(i as usize) {
							write!(f, " of {} Directory", entry)?;
						}
					}
				}
				Ok(())
			}),
		)
	}
}

//----------------------------------------------------------------

impl fmt::Debug for IMAGE_NT_HEADERS32 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
			#"NT Headers\n",
			#"  Signature:               {:·>8X}{}\n", self.Signature, Fmt(|f| stringify(&self.Signature).map(|sig| write!(f, ": {}", sig)).unwrap_or(Ok(()))),
			#"{:?}", self.FileHeader,
			#"{:?}", self.OptionalHeader,
		)
	}
}
impl fmt::Debug for IMAGE_NT_HEADERS64 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
			#"NT Headers\n",
			#"  Signature:               {:·>8X}{}\n", self.Signature, Fmt(|f| stringify(&self.Signature).map(|sig| write!(f, ": {}", sig)).unwrap_or(Ok(()))),
			#"{:?}", self.FileHeader,
			#"{:?}", self.OptionalHeader,
		)
	}
}

//----------------------------------------------------------------

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
impl fmt::Debug for IMAGE_SECTION_HEADER {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
			#"Section Header\n",
			#"  Name:                 {}\n", Fmt(|f|
				match stringify(&self.Name) {
					Some(name) => write!(f, "{:·<8}", name),
					None => write!(f, "{:?}", self.Name),
				}
			),
			#"  VirtualSize:          {:·>8X}\n", self.VirtualSize,
			#"  VirtualAddress:       {:·>8X}\n", self.VirtualAddress,
			#"  SizeOfRawData:        {:·>8X}\n", self.SizeOfRawData,
			#"  PointerToRawData:     {:·>8X}\n", self.PointerToRawData,
			#"  PointerToRelocations: {:·>8X}\n", self.PointerToRelocations,
			#"  PointerToLinenumbers: {:·>8X}\n", self.PointerToLinenumbers,
			#"  NumberOfRelocations:  {:·>8X}\n", self.NumberOfRelocations,
			#"  NumberOfLinenumbers:  {:·>8X}\n", self.NumberOfLinenumbers,
			#"  Characteristics:      {:·>8X}", self.Characteristics,
			#"{}\n", Fmt(|f| {
				let mut it = 0u32..32u32;
				while let Some(i) = it.next() {
					if i == 20 {
						let align = self.Characteristics & 0x00F00000;
						if align != 0 {
							write!(f, "\n    {:·>8X}: ALIGN_{}BYTES", align, 1 << (align >> 20))?;
						}
						it.nth(4);
					}
					else if (1 << i) & self.Characteristics != 0 {
						write!(f, "\n    {:·>8X}", 1 << i)?;
						if let Some(scn) = IMAGE_SCN_STRINGS[i as usize] {
							write!(f, ": {}", scn)?;
						}
					}
				}
				Ok(())
			}),
		)
	}
}

//----------------------------------------------------------------

impl fmt::Debug for IMAGE_EXPORT_DIRECTORY {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
			#"Export Directory",
			#"\n  Characteristics:       {:·>8X}", self.Characteristics,
			#"\n  TimeDateStamp:         {}", self.TimeDateStamp,
			#"\n  Version:               {}.{}", self.MajorVersion, self.MinorVersion,
			#"\n  Name:                  {:·>8X}", self.Name,
			#"\n  OrdinalBase:           {}", self.Base,
			#"\n  NumberOfFunctions:     {}", self.NumberOfFunctions,
			#"\n  NumberOfNames:         {}", self.NumberOfNames,
			#"\n  AddressOfFunctions:    {:·>8X}", self.AddressOfFunctions,
			#"\n  AddressOfNames:        {:·>8X}", self.AddressOfNames,
			#"\n  AddressOfNameOrdinals: {:·>8X}", self.AddressOfNameOrdinals,
			#"\n",
		)
	}
}

//----------------------------------------------------------------

impl fmt::Debug for IMAGE_IMPORT_DESCRIPTOR {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
			#"Import Descriptor",
			#"\n  OriginalFirstThunk: {:·>8X}", self.OriginalFirstThunk,
			#"\n  TimeDateStamp:      {}", self.TimeDateStamp,
			#"\n  ForwarderChain:     {:·>8X}", self.ForwarderChain,
			#"\n  Name:               {:·>8X}", self.Name,
			#"\n  FirstThunk:         {:·>8X}", self.FirstThunk,
			#"\n",
		)
	}
}

//----------------------------------------------------------------

pub static RSRC_TYPES: [Option<&str>; 25] = [
	/* 0*/ None, Some("Cursors"), Some("Bitmaps"), Some("Icons"), Some("Menus"),
	/* 5*/ Some("Dialogs"), Some("Strings"), Some("Font Directory"), Some("Fonts"), Some("Accelerators"),
	/*10*/ Some("Raw Data"), Some("Message Tables"), Some("Group Cursors"), None, Some("Group Icons"),
	/*15*/ None, Some("Version"), Some("DlgInclude"), None, Some("Plug and Play"),
	/*20*/ Some("VXD"), Some("Animated Cursors"), Some("Animated Icons"), Some("HTML"), Some("Manifest"),
];
impl fmt::Debug for IMAGE_RESOURCE_DIRECTORY {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
			#"Resource Directory",
			#"\n  Characteristics:      {:·>8X}", self.Characteristics,
			#"\n  TimeDateStamp:        {}", self.TimeDateStamp,
			#"\n  Version:              {}.{}", self.MajorVersion, self.MinorVersion,
			#"\n  NumberOfNamedEntries: {}", self.NumberOfNamedEntries,
			#"\n  NumberOfIdEntries:    {}", self.NumberOfIdEntries,
			#"\n",
		)
	}
}
impl fmt::Debug for IMAGE_RESOURCE_DIRECTORY_ENTRY {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
			#"Resource Directory Entry",
			#"\n  Name:       {:·>8X}", self.Name,
			#"\n  Offset:     {:·>8X}", self.Offset,
			#"\n",
		)
	}
}
impl fmt::Debug for IMAGE_RESOURCE_DATA_ENTRY {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
			#"Resource Data Entry",
			#"\n  OffsetToData:   {:·>8X}", self.OffsetToData,
			#"\n  Size:           {:·>8X}", self.Size,
			#"\n  CodePage:       {}", self.CodePage,
			#"\n  Reserved:       {:·>8X}", self.Reserved,
			#"\n",
		)
	}
}

//----------------------------------------------------------------

pub fn stringify_reloc_type(&tyoff: &IMAGE_BASE_RELOC_TYPEOFFSET) -> Option<&'static str> {
	match (tyoff >> 12) as u8 {
		IMAGE_REL_BASED_ABSOLUTE => Some("ABSOLUTE"),
		IMAGE_REL_BASED_HIGH => Some("HIGH"),
		IMAGE_REL_BASED_LOW => Some("LOW"),
		IMAGE_REL_BASED_HIGHLOW => Some("HIGHLOW"),
		IMAGE_REL_BASED_HIGHADJ => Some("HIGHADJ"),
		IMAGE_REL_BASED_MACHINE_SPECIFIC_5 => Some("MIPSJMPADDR | ARM_MOV32"),
		IMAGE_REL_BASED_MACHINE_SPECIFIC_7 => Some("THUMB_MOV32"),
		IMAGE_REL_BASED_MACHINE_SPECIFIC_9 => Some("MIPSJMPADDR16 | IA64IMM64"),
		IMAGE_REL_BASED_DIR64 => Some("DIR64"),
		_ => None,
	}
}
impl fmt::Debug for IMAGE_BASE_RELOCATION {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
			#"Base Relocation",
			#"\n  VirtualAddress: {:·>8X}", self.VirtualAddress,
			#"\n  SizeOfBlock:    {:·>8X}", self.SizeOfBlock,
			#"\n",
		)
	}
}

//----------------------------------------------------------------

impl fmt::Debug for IMAGE_TLS_DIRECTORY32 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
			#"TLS Directory",
			#"\n  StartAddressOfRawData:  {:·>8X}", self.StartAddressOfRawData,
			#"\n  EndAddressOfRawData:    {:·>8X}", self.EndAddressOfRawData,
			#"\n  AddressOfIndex:         {:·>8X}", self.AddressOfIndex,
			#"\n  AddressOfCallBacks:     {:·>8X}", self.AddressOfCallBacks,
			#"\n  SizeOfZeroFill:         {:·>8X}", self.SizeOfZeroFill,
			#"\n  Characteristics:        {:·>8X}", self.Characteristics,
			#"\n",
		)
	}
}
impl fmt::Debug for IMAGE_TLS_DIRECTORY64 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
			#"TLS Directory",
			#"\n  StartAddressOfRawData:  {:·>16X}", self.StartAddressOfRawData,
			#"\n  EndAddressOfRawData:    {:·>16X}", self.EndAddressOfRawData,
			#"\n  AddressOfIndex:         {:·>16X}", self.AddressOfIndex,
			#"\n  AddressOfCallBacks:     {:·>16X}", self.AddressOfCallBacks,
			#"\n  SizeOfZeroFill:         {:·>8X}", self.SizeOfZeroFill,
			#"\n  Characteristics:        {:·>8X}", self.Characteristics,
			#"\n",
		)
	}
}

//----------------------------------------------------------------

impl fmt::Debug for GUID {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self)
	}
}
impl fmt::Display for GUID {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let g: &[u8; 16] = unsafe { mem::transmute(self) };
		write!(f, "{:02X}{:02X}{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
			g[0], g[1], g[2], g[3], g[4], g[5], g[6], g[7], g[8], g[9], g[10], g[11], g[12], g[13], g[14], g[15])
	}
}

//----------------------------------------------------------------

fn stringify_debug_type(ty: u32) -> Option<&'static str> {
	match ty {
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
impl fmt::Debug for IMAGE_DEBUG_DIRECTORY {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
			#"Debug Directory\n",
			#"  Characteristics:  {:·>8X}\n", self.Characteristics,
			#"  TimeDateStamp:    {}\n", self.TimeDateStamp,
			#"  Version:          {}.{}\n", self.MajorVersion, self.MinorVersion,
			#"  Type:             {}{}\n", self.Type, Fmt(|f| stringify_debug_type(self.Type).map(|ty| write!(f, ": {}", ty)).unwrap_or(Ok(()))),
			#"  SizeOfData:       {:·>8X}\n", self.SizeOfData,
			#"  AddressOfRawData: {:·>8X}\n", self.AddressOfRawData,
			#"  PointerToRawData: {:·>8X}\n", self.PointerToRawData,
		)
	}
}
impl fmt::Debug for IMAGE_DEBUG_CV_INFO_PDB20 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
			#"CodeView PDB 2.0\n",
			#"  CvSignature:      {:·>8X}{}\n", self.CvSignature, Fmt(|f| stringify(&self.CvSignature).map(|cv_sig| write!(f, ": {}", cv_sig)).unwrap_or(Ok(()))),
			#"  Offset:           {:·>8X}\n", self.Offset,
			#"  TimeDateStamp:    {}\n", self.TimeDateStamp,
			#"  Age:              {}\n", self.Age,
		)
	}
}
impl fmt::Debug for IMAGE_DEBUG_CV_INFO_PDB70 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
			#"CodeView PDB 7.0\n",
			#"  CvSignature:      {:·>8X}{}\n", self.CvSignature, Fmt(|f| stringify(&self.CvSignature).map(|cv_sig| write!(f, ": {}", cv_sig)).unwrap_or(Ok(()))),
			#"  Signature:        {}\n", self.Signature,
			#"  Age:              {}\n", self.Age,
		)
	}
}
impl fmt::Debug for IMAGE_DEBUG_MISC {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
			#"DBG\n",
			#"  DataType:         {}\n", self.DataType,
			#"  Unicode:          {}\n", self.Unicode != 0,
		)
	}
}
