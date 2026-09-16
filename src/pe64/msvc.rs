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
	pub type_descriptor: u32,  //Ptr<TypeDescriptor>
	/// Image-relative offset of the [`RTTIClassHierarchyDescriptor`].
	pub class_descriptor: u32, //Ptr<RTTIClassHierarchyDescriptor>
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
