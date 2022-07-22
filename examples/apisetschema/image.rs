#![allow(non_snake_case, non_camel_case_types)]

use pelite::Pod;

#[derive(Copy, Clone, Debug, Pod)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct API_SET_NAMESPACE_V6 {
    /// v2 on Windows 7, v4 on Windows 8.1  and v6 on Windows 10
    pub Version: u32,
    /// apiset map size (usually the .apiset section virtual size)
    pub Size: u32,
    /// according to Geoff Chappell,  tells if the map is sealed or not.
    pub Flags: u32,
    /// hash table entry count
    pub Count: u32,
    /// Offset to the api set entries values
    pub EntryOffset: u32,
    /// Offset to the api set entries hash indexes
    pub HashOffset: u32,
    /// multiplier to use when computing hash
    pub HashFactor: u32,
}

pub const API_SET_SCHEMA_ENTRY_FLAGS_SEALED: u32 = 1;

#[derive(Copy, Clone, Debug, Pod)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct API_SET_HASH_ENTRY {
    pub Hash: u32,
    pub Index: u32,
}

#[derive(Copy, Clone, Debug, Pod)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct API_SET_NAMESPACE_ENTRY {
    /// sealed flag in bit 0
    pub Flags: u32,
    /// Offset to the ApiSet library name PWCHAR (e.g. "api-ms-win-core-job-l2-1-1")
    pub NameOffset: u32,
    /// Ignored
    pub NameLength: u32,
    /// Apiset library name length
    pub HashedLength: u32,
    /// Offset the list of hosts library implement the apiset contract (points to API_SET_VALUE_ENTRY array)
    pub ValueOffset: u32,
    /// Number of hosts libraries
    pub ValueCount: u32,
}

#[derive(Copy, Clone, Debug, Pod)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct API_SET_VALUE_ENTRY {
    /// sealed flag in bit 0
    pub Flags: u32,
    /// Offset to the ApiSet library name PWCHAR (e.g. "api-ms-win-core-job-l2-1-1")
    pub NameOffset: u32,
    /// Apiset library name length
    pub NameLength: u32,
    /// Offset to the Host library name PWCHAR (e.g. "ucrtbase.dll")
    pub ValueOffset: u32,
    /// Host library name length
    pub ValueLength: u32,
}
