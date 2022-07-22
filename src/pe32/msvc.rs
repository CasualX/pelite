/*!
Some MSVC structs for RTTI and exception handling.

References:

[1]: [Reversing Microsoft Visual C++ Part I: Exception Handling](http://www.openrce.org/articles/full_view/21)
[2]: [Reversing Microsoft Visual C++ Part II: Classes, Methods and RTTI](http://www.openrce.org/articles/full_view/23)
*/

use std::mem;

use crate::{util::CStr, Pod};

use super::Ptr;

//----------------------------------------------------------------

/// Represents the C++ `std::type_info` class returned by the `typeid` operator.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct TypeDescriptor {
    /// Vtable of the `type_info` class.
    pub vftable: Ptr,
    /// Used to keep the demangled name returned by `type_info::name()`.
    pub spare: Ptr<CStr>,
    /// Inlined mangled type name, nul terminated.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub name: [u8; 0],
}

/// Pointer-to-member displacement info.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct PMD {
    /// Member displacement.
    pub mdisp: i32,
    /// Vbtable (virtual base class table) displacement.
    pub pdisp: i32,
    /// Displacement inside the vbtable.
    pub vdisp: i32,
}

//----------------------------------------------------------------

/// Fully describes all try/catch blocks and unwindable objects in the function.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct FuncInfo {
    /// Compiler version.
    ///
    /// * `0x19930520`: up to VC6
    /// * `0x19930521`: VC7.x (2002-2003)
    /// * `0x19930522`: VC8 (2005)
    pub magic_number: u32,
    /// Number of entries in the unwind table.
    pub max_state: i32,
    /// Table of unwind destructors.
    pub unwind_map: Ptr,
    /// Number of try blocks in the function.
    pub try_blocks: u32,
    /// Mapping of catch blocks to try blocks.
    pub try_block_map: Ptr<UnwindMapEntry>,
    pub ip_map_entries: u32,
    pub ip_to_state_map: Ptr,
    /// VC7+ only, expected exceptions list (function "throw" specifier).
    pub es_type_list: Ptr<ESTypeList>,
    /// VC8+ only, bit `0` set if function was compiled with `/EHs`.
    pub eh_flags: i32,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct UnwindMapEntry {
    /// Target state.
    pub to_state: i32,
    /// Action to perform (unwind funclet address).
    ///
    /// Pointer to function with signature `fn()`.
    pub action: Ptr,
}

/// Try block descriptor.
///
/// Describes a try block with associated catches.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct TryBlockMapEntry {
    /// This `try {}` covers states ranging from `try_low` to `try_high`.
    pub try_low: i32,
    pub try_high: i32,
    /// Highest state inside catch handlers of this try.
    pub catch_high: i32,
    /// Number of catch handlers.
    pub catches: i32,
    /// Catch handlers table.
    pub handler_array: Ptr<[HandlerType]>,
}

/// Catch block descriptor.
///
/// Describes a single catch of a try block.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct HandlerType {
    /// * `0x01`: const
    /// * `0x02`: volatile
    /// * `0x08`: reference
    pub adjectives: u32,
    /// RTTI descriptor of the exception type. `0` = any (ellipsis).
    pub ty: Ptr<TypeDescriptor>,
    /// EBP-based offset of the exception object in the function stack. `0` = no object (catch by type).
    pub disp_catch_obj: i32,
    /// Address of the catch handler Code.
    ///
    /// Returns address where to continues execution (i.e. code after the try block).
    pub address_of_handler: Ptr,
}

/// List of expected exceptions.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct ESTypeList {
    /// Number of entries in the list.
    pub count: i32,
    /// List of exceptions; it seems only pType field in HandlerType is used.
    pub type_array: Ptr<[HandlerType]>,
}

//----------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct ThrowInfo {
    /// * `0x01`: const
    /// * `0x02`: volatile
    pub attributes: u32,
    /// Exception destructor.
    ///
    /// Pointer to function with signature `fn()`.
    pub unwind: Ptr,
    /// Forward compatibility handler.
    ///
    /// Pointer to function with signature `fn() -> i32`.
    pub forward_compat: Ptr,
    /// List of types that can catch this exception; i.e. the actual type and all its ancestors.
    pub catchable_type_array: Ptr<CatchableTypeArray>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct CatchableTypeArray {
    /// Number of entries in the following array.
    pub catchable_types: i32,
    /// Array of pointers to catchable types.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub array: [Ptr<CatchableType>; 0],
}

/// Describes a type that can catch this exception.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct CatchableType {
    /// * `0x01`: simple type (can be copied by memmove)
    /// * `0x02`: can be caught by reference only
    /// * `0x04`: has virtual bases
    pub properties: u32,
    /// Pointer to its type descriptor.
    pub type_descriptor: Ptr<TypeDescriptor>,
    /// How to cast the thrown object to this type.
    pub pmd: PMD,
    /// Object size.
    pub size_or_offset: i32,
    /// Copy constructor address.
    pub copy_function: Ptr,
}

//----------------------------------------------------------------

/// Complete Object Locator.
///
/// MSVC compiler puts a pointer to this structure just before the vftable.
/// The structure is called so because it lets you find the location to the complete object from a specific vftable pointer.
///
/// Every vftable has its own Complete Object Locator.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct RTTICompleteObjectLocator {
    /// Always zero?
    pub signature: u32,
    /// Offset of this vtable in the complete class.
    pub offset: u32,
    /// Constructor displacement offset.
    pub cd_offset: u32,
    /// Pointer to the type descriptor of the complete class.
    pub type_descriptor: Ptr<TypeDescriptor>,
    /// Pointer to the class hierarchy descriptor.
    pub class_descriptor: Ptr<RTTIClassHierarchyDescriptor>,
}

/// Class Hierarchy Descriptor.
///
/// Describes the inheritance hierarchy of the class, it is shared by all [COL](struct.RTTICompleteObjectLocator.html)s.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct RTTIClassHierarchyDescriptor {
    /// Always zero?
    pub signature: u32,
    /// Bit `0` set = multiple inheritance, bit `1` set = virtual inheritance.
    pub attributes: u32,
    /// Number of classes in `base_class_array`.
    pub num_base_classes: u32,
    /// Pointer to an array of pointers to base class descriptors.
    pub base_class_array: Ptr<[Ptr<RTTIBaseClassDescriptor>]>,
}

/// Entry in the [Base Class Array](struct.RTTIClassHierarchyDescriptor.html#base_class_array.v).
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct RTTIBaseClassDescriptor {
    /// Type descriptor of the class.
    pub type_descriptor: Ptr<TypeDescriptor>,
    /// Number of nested classes following in the `base_class_array`.
    pub num_contained_bases: u32,
    /// Pointer-to-member displacement info.
    pub pmd: PMD,
    /// Flags, usually `0`. (?)
    pub attributes: u32,
}

//----------------------------------------------------------------

unsafe impl Pod for TypeDescriptor {}
unsafe impl Pod for PMD {}
unsafe impl Pod for FuncInfo {}
unsafe impl Pod for UnwindMapEntry {}
unsafe impl Pod for TryBlockMapEntry {}
unsafe impl Pod for HandlerType {}
unsafe impl Pod for ESTypeList {}
unsafe impl Pod for ThrowInfo {}
unsafe impl Pod for CatchableTypeArray {}
unsafe impl Pod for CatchableType {}
unsafe impl Pod for RTTICompleteObjectLocator {}
unsafe impl Pod for RTTIClassHierarchyDescriptor {}
unsafe impl Pod for RTTIBaseClassDescriptor {}

//----------------------------------------------------------------

const _: [(); 8] = [(); mem::size_of::<TypeDescriptor>()]; // Unsized
const _: [(); 12] = [(); mem::size_of::<PMD>()];
const _: [(); 36] = [(); mem::size_of::<FuncInfo>()];
const _: [(); 8] = [(); mem::size_of::<UnwindMapEntry>()];
const _: [(); 20] = [(); mem::size_of::<TryBlockMapEntry>()];
const _: [(); 16] = [(); mem::size_of::<HandlerType>()];
const _: [(); 8] = [(); mem::size_of::<ESTypeList>()];
const _: [(); 16] = [(); mem::size_of::<ThrowInfo>()];
const _: [(); 4] = [(); mem::size_of::<CatchableTypeArray>()]; // Unsized
const _: [(); 28] = [(); mem::size_of::<CatchableType>()];
const _: [(); 20] = [(); mem::size_of::<RTTICompleteObjectLocator>()];
const _: [(); 16] = [(); mem::size_of::<RTTIClassHierarchyDescriptor>()];
const _: [(); 24] = [(); mem::size_of::<RTTIBaseClassDescriptor>()];
