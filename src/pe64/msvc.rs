use crate::{util::CStr, Pod};

use super::Ptr;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct TypeDescriptor {
    pub vftable: Ptr,
    pub spare: Ptr<CStr>,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub name: [u8; 0],
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct PMD {
    pub mdisp: i32,
    pub pdisp: i32,
    pub vdisp: i32,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct RTTICompleteObjectLocator {
    pub signature: u32,
    pub offset: u32,
    pub cd_offset: u32,
    pub type_descriptor: u32,  //Ptr<TypeDescriptor>
    pub class_descriptor: u32, //Ptr<RTTIClassHierarchyDescriptor>
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct RTTIClassHierarchyDescriptor {
    pub signature: u32,
    pub attributes: u32,
    pub num_base_classes: u32,
    pub base_class_array: u32, //Ptr<[Ptr<RTTIBaseClassDescriptor>]>,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct RTTIBaseClassDescriptor {
    pub type_descriptor: u32, //Ptr<TypeDescriptor>,
    pub num_contained_bases: u32,
    pub pmd: PMD,
    pub attributes: u32,
}

unsafe impl Pod for TypeDescriptor {}
unsafe impl Pod for PMD {}
unsafe impl Pod for RTTICompleteObjectLocator {}
unsafe impl Pod for RTTIClassHierarchyDescriptor {}
unsafe impl Pod for RTTIBaseClassDescriptor {}
