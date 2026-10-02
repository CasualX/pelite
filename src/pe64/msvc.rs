use super::*;

/// MSVC run-time type information for a C++ type.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[repr(C)]
pub struct TypeDescriptor {
	/// Vtable of the C++ `type_info` class.
	pub vftable: Ptr,
	/// Storage used by MSVC for the decorated type name.
	pub spare: Ptr<CStr>,
	/// Inlined nul-terminated decorated type name.
	#[cfg_attr(feature = "serde", serde(skip))]
	pub name: [u8; 0],
}

/// Pointer-to-member displacement information.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[repr(C)]
pub struct PMD {
	/// Member displacement within the class.
	pub mdisp: i32,
	/// Displacement to the virtual base table.
	pub pdisp: i32,
	/// Displacement within the virtual base table.
	pub vdisp: i32,
}

/// Locates the complete C++ object and its RTTI descriptors.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[repr(C)]
pub struct RTTICompleteObjectLocator {
	/// RTTI format signature.
	pub signature: u32,
	/// Offset of this subobject within the complete object.
	pub offset: u32,
	/// Constructor displacement offset.
	pub cd_offset: u32,
	/// Image-relative offset of the [`TypeDescriptor`].
	pub type_descriptor: u32, //Ptr<TypeDescriptor>
	/// Image-relative offset of the [`RTTIClassHierarchyDescriptor`].
	pub class_descriptor: u32, //Ptr<RTTIClassHierarchyDescriptor>
	/// Image-relative offset of this locator (version 1 format).
	pub self_rva: u32,
}

/// Describes the inheritance hierarchy of an MSVC C++ class.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[repr(C)]
pub struct RTTIClassHierarchyDescriptor {
	/// RTTI format signature.
	pub signature: u32,
	/// Inheritance hierarchy attribute flags.
	pub attributes: u32,
	/// Number of entries in the base-class array.
	pub num_base_classes: u32,
	/// Image-relative offset of the base-class descriptor array.
	pub base_class_array: u32, //Ptr<[Ptr<RTTIBaseClassDescriptor>]>,
}

/// Describes one base class in an MSVC RTTI hierarchy.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[repr(C)]
pub struct RTTIBaseClassDescriptor {
	/// Image-relative offset of the base class's [`TypeDescriptor`].
	pub type_descriptor: u32, //Ptr<TypeDescriptor>,
	/// Number of base-class descriptors contained below this entry.
	pub num_contained_bases: u32,
	/// Displacements used to locate this base-class subobject.
	pub pmd: PMD,
	/// Base-class attribute flags.
	pub attributes: u32,
}

unsafe impl Pod for TypeDescriptor {}
unsafe impl Pod for PMD {}
unsafe impl Pod for RTTICompleteObjectLocator {}
unsafe impl Pod for RTTIClassHierarchyDescriptor {}
unsafe impl Pod for RTTIBaseClassDescriptor {}

assert_sizeof!(24, RTTICompleteObjectLocator);

/// Describes a thrown C++ exception. All address fields are image-relative
/// RVAs, even though native pointers in a PE32+ image are 64 bits wide.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[repr(C)]
pub struct ThrowInfo {
	/// Qualifiers: bit 0 = const, bit 1 = volatile.
	pub attributes: u32,
	/// Exception destructor RVA, or zero.
	pub unwind: u32,
	/// Forward compatibility handler RVA, or zero.
	pub forward_compat: u32,
	/// RVA of the [`CatchableTypeArray`].
	pub catchable_type_array: u32,
}

/// Variable-length list of types capable of catching a thrown exception.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[repr(C)]
pub struct CatchableTypeArray {
	/// Number of RVAs following this header.
	pub catchable_types: i32,
	/// RVAs of [`CatchableType`] records.
	#[cfg_attr(feature = "serde", serde(skip))]
	pub array: [u32; 0],
}

/// Describes one permitted conversion of a thrown exception.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[repr(C)]
pub struct CatchableType {
	/// Bit 0 = simple type, bit 1 = reference only, bit 2 = virtual bases.
	pub properties: u32,
	/// RVA of the [`TypeDescriptor`].
	pub type_descriptor: u32,
	/// Displacements used to convert the thrown object to this type.
	pub pmd: PMD,
	/// Copy size, or an offset for special runtime representations.
	pub size_or_offset: i32,
	/// Copy helper RVA, or zero for a trivial copy.
	pub copy_function: u32,
}

unsafe impl Pod for ThrowInfo {}
unsafe impl Pod for CatchableTypeArray {}
unsafe impl Pod for CatchableType {}

assert_sizeof!(16, ThrowInfo);
assert_sizeof!(4, CatchableTypeArray);
assert_sizeof!(28, CatchableType);
