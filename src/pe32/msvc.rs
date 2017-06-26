/*!
Some MSVC structs for RTTI and exception handling.

References:

[1]: [Reversing Microsoft Visual C++ Part I: Exception Handling](http://www.openrce.org/articles/full_view/21)
[2]: [Reversing Microsoft Visual C++ Part II: Classes, Methods and RTTI](http://www.openrce.org/articles/full_view/23)
*/

use super::image::{Va};
use super::ptr::Ptr;
use ::util::Pod;

//----------------------------------------------------------------

#[derive(Debug)]
#[repr(C)]
pub struct TypeDescriptor {
	pub vftable: Va,
	pub spare: Va,
	pub name: [u8; 0],
}

/// Pointer-to-member displacement info.
#[derive(Debug)]
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

#[derive(Debug)]
#[repr(C)]
pub struct FuncInfo {
	/// Compiler version.
	///
	/// 0x19930520: up to VC6, 0x19930521: VC7.x(2002-2003), 0x19930522: VC8 (2005)
	pub magic_number: u32,
	/// Number of entries in the unwind table.
	pub max_state: i32,
	/// Table of unwind destructors.
	pub unwind_map: Va,
	/// Number of try blocks in the function.
	pub try_blocks: u32,
	/// Mapping of catch blocks to try blocks.
	pub try_block_map: Ptr<UnwindMapEntry>,
	pub ip_map_entries: u32,
	pub ip_to_state_map: Va,
	/// VC7+ only, expected exceptions list (function "throw" specifier).
	pub es_type_list: Ptr<ESTypeList>,
	/// VC8+ only, bit 0 set if function was compiled with `/EHs`.
	pub eh_flags: i32,
}

#[derive(Debug)]
#[repr(C)]
pub struct UnwindMapEntry {
	/// Target state.
	pub to_state: i32,
	/// Action to perform (unwind funclet address).
	///
	/// Pointer to function with signature `fn()`.
	pub action: Va,
}

#[derive(Debug)]
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

#[derive(Debug)]
#[repr(C)]
pub struct HandlerType {
	/// 0x01: const, 0x02: volatile, 0x08: reference.
	pub adjectives: u32,
	/// RTTI descriptor of the exception type. 0 = any (ellipsis).
	pub ty: Ptr<TypeDescriptor>,
	/// EBP-based offset of the exception object in the function stack. 0 = no object (catch by type).
	pub disp_catch_obj: i32,
	/// Address of the catch handler Code.
	///
	/// Returns address where to continues execution (i.e. code after the try block).
	pub address_of_handler: Va,
}

#[derive(Debug)]
#[repr(C)]
pub struct ESTypeList {
	/// Number of entries in the list.
	pub count: i32,
	/// list of exceptions; it seems only pType field in HandlerType is used.
	pub type_array: Ptr<[HandlerType]>,
}

//----------------------------------------------------------------

#[derive(Debug)]
#[repr(C)]
pub struct ThrowInfo {
	/// 0x01: const, 0x02: volatile
	pub attributes: u32,
	/// Exception destructor.
	///
	/// Pointer to function with signature `fn()`.
	pub unwind: Va,
	/// Forward compatibility handler.
	///
	/// Pointer to function with signature `fn() -> i32`.
	pub forward_compat: Va,
	/// List of types that can catch this exception; i.e. the actual type and all its ancestors.
	pub catchable_type_array: Ptr<CatchableTypeArray>,
}

#[derive(Debug)]
#[repr(C)]
pub struct CatchableTypeArray {
	/// Number of entries in the following array.
	pub catchable_types: i32,
	/// Array of pointers to catchable types.
	pub array: [Ptr<CatchableType>; 0],
}

#[derive(Debug)]
#[repr(C)]
pub struct CatchableType {
	/// 0x01: simple type (can be copied by memmove), 0x02: can be caught by reference only, 0x04: has virtual bases.
	pub properties: u32,
	/// Pointer to its type descriptor.
	pub type_descriptor: Ptr<TypeDescriptor>,
	/// How to cast the thrown object to this type.
	pub pmd: PMD,
	/// Object size.
	pub size_or_offset: i32,
	/// Copy constructor address.
	pub copy_function: Va,
}

//----------------------------------------------------------------

#[derive(Debug)]
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

#[derive(Debug)]
#[repr(C)]
pub struct RTTIClassHierarchyDescriptor {
	/// Always zero?
	pub signature: u32,
	/// bit 0 set = multiple inheritance, bit 1 set = virtual inheritance.
	pub attributes: u32,
	/// Number of classes in `base_class_array`.
	pub num_base_classes: u32,
	/// Pointer to an array of pointers to base class descriptors.
	pub base_class_array: Ptr<[Ptr<RTTIBaseClassDescriptor>]>,
}

#[derive(Debug)]
#[repr(C)]
pub struct RTTIBaseClassDescriptor {
	/// Type descriptor of the class.
	pub type_descriptor: Ptr<TypeDescriptor>,
	/// Number of nested classes following in the `base_class_array`.
	pub num_contained_bases: u32,
	/// Pointer-to-member displacement info.
	pub pmd: PMD,
	/// Flags, usually 0. (?)
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
