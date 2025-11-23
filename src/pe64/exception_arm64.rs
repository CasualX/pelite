/*!
ARM64 Exception Directory.

Reference: https://learn.microsoft.com/en-us/cpp/build/arm64-exception-handling?view=msvc-170
*/

use std::{fmt, iter, mem, slice};

use crate::{Error, Result};

use super::image::*;
use super::Pe;

//----------------------------------------------------------------

/// Exception Directory for ARM64 images.
#[derive(Copy, Clone)]
pub struct Exception<'a, P> {
	pe: P,
	image: &'a [IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY],
}
impl<'a, P: Pe<'a>> Exception<'a, P> {
	/// Parses the ARM64 exception directory for the given PE.
	pub fn try_from(pe: P) -> Result<Exception<'a, P>> {
		if pe.file_header().Machine != IMAGE_FILE_MACHINE_ARM64 {
			return Err(Error::Invalid);
		}
		let datadir = pe.data_directory().get(IMAGE_DIRECTORY_ENTRY_EXCEPTION).ok_or(Error::Bounds)?;
		let len = datadir.Size as usize / mem::size_of::<IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY>();
		let rem = datadir.Size as usize % mem::size_of::<IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY>();
		if rem != 0 {
			return Err(Error::Invalid);
		}
		let image = pe.derva_slice(datadir.VirtualAddress, len)?;
		Ok(Exception { pe, image })
	}
	/// Gets the PE instance.
	pub fn pe(&self) -> P {
		self.pe
	}
	/// Returns the functions slice.
	pub fn image(&self) -> &'a [IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY] {
		self.image
	}
	/// Checks if the function table is sorted by begin address.
	pub fn check_sorted(&self) -> bool {
		self.image.windows(2).all(|window| window[0].BeginAddress <= window[1].BeginAddress)
	}
	/// Gets an iterator over the function records.
	pub fn functions(&self) -> iter::Map<slice::Iter<'a, IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY>, impl Clone + FnMut(&'a IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY) -> Function<'a, P>> {
		let pe = self.pe;
		self.image.iter().map(move |image| Function { pe, image })
	}
}
impl<'a, P: Pe<'a>> fmt::Debug for Exception<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f, "ExceptionArm64 {{")?;
		writeln!(f, "    functions.len: {},", self.image.len())?;
		writeln!(f, "    functions: [")?;
		for (index, function) in self.functions().enumerate() {
			let begin = function.begin_address();
			let unwind = function.raw_unwind_data();
			write!(
				f,
				"        [{:04}] begin=0x{:08x} unwind=0x{:08x}",
				index,
				begin,
				unwind
			)?;
			match function.end_address() {
				Ok(Some(end)) => {
					let size = end.saturating_sub(begin);
					write!(f, " end=0x{:08x} size=0x{:04x}", end, size)?;
				}
				Ok(None) => {}
				Err(err) => {
					write!(f, " <end err: {:?}>", err)?;
				}
			}
			writeln!(f, ",")?;
			match function.unwind_data() {
				Ok(data) => writeln!(f, "            {:?}", data)?,
				Err(err) => writeln!(f, "            <unwind decode error: {:?}>", err)?,
			}
		}
		writeln!(f, "    ]")?;
		write!(f, "}}")
	}
}

//----------------------------------------------------------------

/// ARM64 runtime function.
#[derive(Copy, Clone)]
pub struct Function<'a, P> {
	pe: P,
	image: &'a IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY,
}
impl<'a, P: Pe<'a>> Function<'a, P> {
	/// Gets the PE instance.
	pub fn pe(&self) -> P {
		self.pe
	}
	/// Returns the underlying runtime function image.
	pub fn image(&self) -> &'a IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY {
		self.image
	}
	/// Returns the function begin address.
	pub fn begin_address(&self) -> Rva {
		self.image.BeginAddress
	}
	/// Returns the raw unwind data value.
	pub fn raw_unwind_data(&self) -> u32 {
		self.image.UnwindData
	}
	/// Interprets the unwind data.
	pub fn unwind_data(&self) -> Result<Arm64UnwindData> {
		Arm64UnwindData::decode(self.image.UnwindData)
	}
	/// Computes the optional end address of the function.
	pub fn end_address(&self) -> Result<Option<Rva>> {
		let begin = self.image.BeginAddress;
		match self.unwind_data()? {
			Arm64UnwindData::XData { address } => {
				if let Some(len) = self.xdata_function_length(address)? {
					let end = begin.checked_add(len).ok_or(Error::Overflow)?;
					Ok(Some(end))
				}
				else {
					Ok(None)
				}
			}
			Arm64UnwindData::PackedFunction(info) | Arm64UnwindData::PackedFragment(info) => info.end_address(begin).map(Some),
		}
	}
	/// Attempts to fetch the function bytes.
	///
	/// Only available when the function length can be determined from packed data.
	pub fn bytes(&self) -> Result<&'a [u8]> {
		let end = self.end_address()?.ok_or(Error::Invalid)?;
		if self.image.BeginAddress > end {
			return Err(Error::Overflow);
		}
		let len = end - self.image.BeginAddress;
		self.pe.derva_slice(self.image.BeginAddress, len as usize)
	}

	fn xdata_function_length(&self, address: Rva) -> Result<Option<Rva>> {
		let header = self.pe.derva_copy::<u32>(address)?;
		Ok(decode_xdata_function_length(header))
	}
}

//----------------------------------------------------------------

/// Extension trait to access the ARM64 exception directory on any `Pe` implementor.
pub trait Arm64ExceptionExt<'a>: Pe<'a> + Sized {
	/// Returns the ARM64 exception directory.
	///
	/// This is equivalent to calling [`Exception::try_from`].
	fn exception_arm64(self) -> Result<Exception<'a, Self>> {
		Exception::try_from(self)
	}
}

impl<'a, P: Pe<'a>> Arm64ExceptionExt<'a> for P {}
impl<'a, P: Pe<'a>> fmt::Debug for Function<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("FunctionArm64").field("begin", &self.begin_address()).field("unwind_data", &self.unwind_data()).finish()
	}
}

//----------------------------------------------------------------

/// Pre-decodes the unwind data.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Arm64UnwindData {
	/// Entry references an `.xdata` record.
	XData { address: Rva },
	/// Entry encodes unwind information inline.
	PackedFunction(PackedUnwindInfo),
	/// Entry encodes a fragment of a large function inline.
	PackedFragment(PackedUnwindInfo),
}
impl Arm64UnwindData {
	fn decode(raw: u32) -> Result<Arm64UnwindData> {
		let flag_bits = (raw & 0b11) as u32;
		let flag = Arm64FnPdataFlags::from_bits(flag_bits).ok_or(Error::Invalid)?;
		match flag {
			Arm64FnPdataFlags::RefToFullXdata => Ok(Arm64UnwindData::XData { address: raw & !0b11 }),
			Arm64FnPdataFlags::PackedUnwindFunction => {
				let info = PackedUnwindInfo::from_raw(raw)?;
				Ok(Arm64UnwindData::PackedFunction(info))
			},
			Arm64FnPdataFlags::PackedUnwindFragment => {
				let info = PackedUnwindInfo::from_raw(raw)?;
				Ok(Arm64UnwindData::PackedFragment(info))
			},
		}
	}
}

const XDATA_FUNCTION_LENGTH_MASK: u32 = (1 << 18) - 1;

fn decode_xdata_function_length(header: u32) -> Option<Rva> {
	let len = header & XDATA_FUNCTION_LENGTH_MASK;
	if len == 0 {
		return None;
	}
	len.checked_mul(4)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Arm64FnPdataFlags {
	RefToFullXdata,
	PackedUnwindFunction,
	PackedUnwindFragment,
}
impl Arm64FnPdataFlags {
	fn from_bits(bits: u32) -> Option<Arm64FnPdataFlags> {
		match bits {
			0 => Some(Arm64FnPdataFlags::RefToFullXdata),
			1 => Some(Arm64FnPdataFlags::PackedUnwindFunction),
			2 => Some(Arm64FnPdataFlags::PackedUnwindFragment),
			_ => None,
		}
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Arm64FnPdataCr {
	Unchained,
	UnchainedSavedLr,
	ChainedWithPac,
	Chained,
}
impl Arm64FnPdataCr {
	fn from_bits(bits: u32) -> Option<Arm64FnPdataCr> {
		match bits {
			0 => Some(Arm64FnPdataCr::Unchained),
			1 => Some(Arm64FnPdataCr::UnchainedSavedLr),
			2 => Some(Arm64FnPdataCr::ChainedWithPac),
			3 => Some(Arm64FnPdataCr::Chained),
			_ => None,
		}
	}
}

/// Extracted packed unwind data fields.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PackedUnwindInfo {
	pub function_length: u16,
	pub reg_f: u8,
	pub reg_i: u8,
	pub homed_parameter_registers: bool,
	pub cr: Arm64FnPdataCr,
	pub frame_size: u16,
}
impl PackedUnwindInfo {
	fn from_raw(raw: u32) -> Result<PackedUnwindInfo> {
		let function_length = ((raw >> 2) & 0x7ff) as u16;
		let reg_f = ((raw >> 13) & 0x7) as u8;
		let reg_i = ((raw >> 16) & 0xf) as u8;
		let homed = ((raw >> 20) & 0x1) != 0;
		let cr_bits = (raw >> 21) & 0x3;
		let cr = Arm64FnPdataCr::from_bits(cr_bits).ok_or(Error::Invalid)?;
		let frame_size = ((raw >> 23) & 0x1ff) as u16;
		Ok(PackedUnwindInfo {
			function_length,
			reg_f,
			reg_i,
			homed_parameter_registers: homed,
			cr,
			frame_size,
		})
	}
	fn function_length_bytes(&self) -> Result<Rva> {
		(self.function_length as Rva).checked_mul(4).ok_or(Error::Overflow)
	}
	fn end_address(&self, begin: Rva) -> Result<Rva> {
		let span = self.function_length_bytes()?;
		begin.checked_add(span).ok_or(Error::Overflow)
	}
}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn decode_packed_unwind_info() {
		// Flag = 1, function_length = 0x3f, reg_f = 2, reg_i = 4, H = 1, CR = 2, frame_size = 0x100 >> 4
		let raw = (1u32) | (0x3fu32 << 2) | (2u32 << 13) | (4u32 << 16) | (1u32 << 20) | (2u32 << 21) | (0x100u32 << 23);
		let data = Arm64UnwindData::decode(raw).unwrap();
		match data {
			Arm64UnwindData::PackedFunction(info) => {
				assert_eq!(info.function_length, 0x3f);
				assert_eq!(info.reg_f, 2);
				assert_eq!(info.reg_i, 4);
				assert!(info.homed_parameter_registers);
				assert_eq!(info.cr, Arm64FnPdataCr::ChainedWithPac);
				assert_eq!(info.frame_size, 0x100 & 0x1ff);
				assert_eq!(info.end_address(0x1000).unwrap(), 0x1000 + (0x3f * 4));
			},
			_ => panic!("unexpected variant"),
		}
	}

	#[test]
	fn decode_xdata_len() {
		let header = 0x3f | (1 << 18); // Length=0x3f, other bits set
		assert_eq!(super::decode_xdata_function_length(header), Some(0xFC));
		assert_eq!(super::decode_xdata_function_length(0), None);
	}
}
