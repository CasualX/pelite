/*!
PE32 image structures.
*/

#![allow(non_camel_case_types)]

pub use crate::image::*;

pub const IMAGE_NT_OPTIONAL_HDR_MAGIC: u16 = IMAGE_NT_OPTIONAL_HDR32_MAGIC;

pub type IMAGE_OPTIONAL_HEADER = IMAGE_OPTIONAL_HEADER32;
pub type IMAGE_NT_HEADERS = IMAGE_NT_HEADERS32;
pub type IMAGE_LOAD_CONFIG_DIRECTORY = IMAGE_LOAD_CONFIG_DIRECTORY32;
pub type IMAGE_DYNAMIC_RELOCATION = IMAGE_DYNAMIC_RELOCATION32;
pub type IMAGE_GUARDCF = IMAGE_GUARDCF32;
pub type IMAGE_TLS_DIRECTORY = IMAGE_TLS_DIRECTORY32;

/// Relative virtual address type, these are all offsets from the base of the mapped image in memory.
pub type Rva = u32;
/// Virtual address type, absolute address as known by the image. Not always the same as a pointer.
pub type Va = u32;
/// Signed address offset type used in pointer arithmetic.
pub(super) type SignedVa = i32;

/// Invalid Rva value.
pub const BADRVA: Rva = !0;
/// Invalid Va value.
pub const BADVA: Va = !0;

/// Ordinal type for imports and exports.
pub type Ordinal = u16;
pub const IMAGE_ORDINAL_FLAG: Va = IMAGE_ORDINAL_FLAG32;
