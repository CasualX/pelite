use pelite::Pod;

#[derive(Debug, Pod)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct ApiSetMap {
	pub Version: u32,
	pub NumberOfHosts: u32,
	#[cfg_attr(feature = "serde", serde(skip))]
	pub Descriptors: [DllHostDescriptor; 0],
}

#[derive(Copy, Clone, Debug, Pod)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct DllHostDescriptor {
	pub OffsetDllString: u32,
	pub StringLength: u32,
	pub OffsetDllRedirector: u32,
}

#[derive(Debug, Pod)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct DllRedirector {
	pub NumberOfRedirections: u32,
	#[cfg_attr(feature = "serde", serde(skip))]
	pub Redirection: [Redirection; 0],
}

#[derive(Copy, Clone, Debug, Pod)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[repr(C)]
pub struct Redirection {
	pub OffsetRedirection1: u32,
	pub RedirectionLength1: u16,
	#[cfg_attr(feature = "serde", serde(skip))]
	pub _pad1: u16,
	pub OffsetRedirection2: u32,
	pub RedirectionLength2: u16,
	#[cfg_attr(feature = "serde", serde(skip))]
	pub _pad2: u16,
}
