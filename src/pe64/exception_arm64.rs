use super::*;

//----------------------------------------------------------------

/// Exception directory for ARM64 images.
///
/// See Microsoft's [ARM64 exception handling documentation][docs].
///
/// [docs]: https://learn.microsoft.com/en-us/cpp/build/arm64-exception-handling?view=msvc-170
#[derive(Copy, Clone)]
pub struct Arm64ExceptionDirectory<'a, P> {
	pe: P,
	image: &'a [IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY],
}
impl<'a, P: Pe<'a>> Arm64ExceptionDirectory<'a, P> {
	/// Parses the ARM64 exception directory for the given PE.
	pub(crate) fn try_from(pe: P) -> Result<Arm64ExceptionDirectory<'a, P>> {
		let datadir = pe.data_directory().get(IMAGE_DIRECTORY_ENTRY_EXCEPTION).ok_or(Error::Bounds)?;
		if datadir.VirtualAddress == 0 {
			return Err(Error::Null);
		}
		if pe.file_header().Machine != IMAGE_FILE_MACHINE_ARM64 {
			return Err(Error::Invalid);
		}
		let len = datadir.Size as usize / mem::size_of::<IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY>();
		let rem = datadir.Size as usize % mem::size_of::<IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY>();
		if rem != 0 {
			return Err(Error::Invalid);
		}
		let image = pe.derva_slice(datadir.VirtualAddress, len)?;
		Ok(Arm64ExceptionDirectory { pe, image })
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
	pub fn functions(&self) -> iter::Map<slice::Iter<'a, IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY>, impl Clone + FnMut(&'a IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY) -> Arm64RuntimeFunction<'a, P>> {
		let pe = self.pe;
		self.image.iter().map(move |image| Arm64RuntimeFunction { pe, image })
	}
}
impl<'a, P: Pe<'a>> fmt::Debug for Arm64ExceptionDirectory<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Arm64ExceptionDirectory")
			.field("functions", &crate::util::DebugList(self.functions()))
			.finish()
	}
}

//----------------------------------------------------------------

/// ARM64 runtime function.
#[derive(Copy, Clone)]
pub struct Arm64RuntimeFunction<'a, P> {
	pe: P,
	image: &'a IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY,
}
impl<'a, P: Pe<'a>> Arm64RuntimeFunction<'a, P> {
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
			},
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
		decode_xdata_function_length(header)
	}
}
impl<'a, P: Pe<'a>> fmt::Debug for Arm64RuntimeFunction<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let begin = self.begin_address();
		let mut debug = f.debug_struct("Arm64RuntimeFunction");
		debug.field("begin", &format_args!("{:#010x}", begin));
		debug.field("unwind", &format_args!("{:#010x}", self.raw_unwind_data()));
		match self.end_address() {
			Ok(Some(end)) => {
				debug.field("end", &format_args!("{:#010x}", end));
				debug.field("size", &format_args!("{:#06x}", end.saturating_sub(begin)));
			},
			Ok(None) => {},
			Err(err) => {
				debug.field("end", &err);
			},
		}
		debug.field("unwind_data", &self.unwind_data()).finish()
	}
}

//----------------------------------------------------------------

/// Pre-decodes the unwind data.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Arm64UnwindData {
	/// Entry references an `.xdata` record.
	XData {
		/// Address of the full unwind record.
		address: Rva,
	},
	/// Entry encodes unwind information inline.
	PackedFunction(Arm64PackedUnwindInfo),
	/// Entry encodes a fragment of a large function inline.
	PackedFragment(Arm64PackedUnwindInfo),
}
impl Arm64UnwindData {
	fn decode(raw: u32) -> Result<Arm64UnwindData> {
		let flag_bits = (raw & 0b11) as u32;
		let flag = Arm64PdataKind::from_bits(flag_bits).ok_or(Error::Invalid)?;
		match flag {
			Arm64PdataKind::RefToFullXdata => Ok(Arm64UnwindData::XData { address: raw & !0b11 }),
			Arm64PdataKind::PackedUnwindFunction => {
				let info = Arm64PackedUnwindInfo::from_raw(raw)?;
				Ok(Arm64UnwindData::PackedFunction(info))
			},
			Arm64PdataKind::PackedUnwindFragment => {
				let info = Arm64PackedUnwindInfo::from_raw(raw)?;
				Ok(Arm64UnwindData::PackedFragment(info))
			},
		}
	}
}

const XDATA_FUNCTION_LENGTH_MASK: u32 = (1 << 18) - 1;

fn decode_xdata_function_length(header: u32) -> Result<Option<Rva>> {
	let version = (header >> 18) & 0b11;
	if version != 0 {
		return Err(Error::Invalid);
	}
	let len = header & XDATA_FUNCTION_LENGTH_MASK;
	if len == 0 {
		return Ok(None);
	}
	len.checked_mul(4).map(Some).ok_or(Error::Overflow)
}

/// Encoding kind stored in the low bits of an ARM64 `.pdata` entry.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Arm64PdataKind {
	/// The remaining bits reference a full `.xdata` record.
	RefToFullXdata,
	/// The entry contains packed unwind data for a complete function.
	PackedUnwindFunction,
	/// The entry contains packed unwind data for a function fragment.
	PackedUnwindFragment,
}
impl Arm64PdataKind {
	fn from_bits(bits: u32) -> Option<Arm64PdataKind> {
		match bits {
			0 => Some(Arm64PdataKind::RefToFullXdata),
			1 => Some(Arm64PdataKind::PackedUnwindFunction),
			2 => Some(Arm64PdataKind::PackedUnwindFragment),
			_ => None,
		}
	}
}

/// Return-address handling encoded in packed ARM64 unwind data.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Arm64ChainedReturn {
	/// The function is unchained and does not save the link register.
	Unchained,
	/// The function is unchained and saves the link register.
	UnchainedSavedLr,
	/// The function is chained and uses pointer authentication.
	ChainedWithPac,
	/// The function is chained without pointer authentication.
	Chained,
}
impl Arm64ChainedReturn {
	fn from_bits(bits: u32) -> Option<Arm64ChainedReturn> {
		match bits {
			0 => Some(Arm64ChainedReturn::Unchained),
			1 => Some(Arm64ChainedReturn::UnchainedSavedLr),
			2 => Some(Arm64ChainedReturn::ChainedWithPac),
			3 => Some(Arm64ChainedReturn::Chained),
			_ => None,
		}
	}
}

/// Extracted packed unwind data fields.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Arm64PackedUnwindInfo {
	/// Function length in four-byte instruction units.
	pub function_length: u16,
	/// Encoded count of saved floating-point registers.
	pub reg_f: u8,
	/// Encoded count of saved integer registers.
	pub reg_i: u8,
	/// Whether the integer parameter registers are homed.
	pub homed_parameter_registers: bool,
	/// Return-address handling mode.
	pub chained_return: Arm64ChainedReturn,
	/// Encoded stack-frame size in 16-byte units.
	pub frame_size: u16,
}
impl Arm64PackedUnwindInfo {
	fn from_raw(raw: u32) -> Result<Arm64PackedUnwindInfo> {
		let function_length = ((raw >> 2) & 0x7ff) as u16;
		let reg_f = ((raw >> 13) & 0x7) as u8;
		let reg_i = ((raw >> 16) & 0xf) as u8;
		let homed = ((raw >> 20) & 0x1) != 0;
		let chained_return_bits = (raw >> 21) & 0x3;
		let chained_return = Arm64ChainedReturn::from_bits(chained_return_bits).ok_or(Error::Invalid)?;
		let frame_size = ((raw >> 23) & 0x1ff) as u16;
		Ok(Arm64PackedUnwindInfo {
			function_length,
			reg_f,
			reg_i,
			homed_parameter_registers: homed,
			chained_return,
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
				assert_eq!(info.chained_return, Arm64ChainedReturn::ChainedWithPac);
				assert_eq!(info.frame_size, 0x100 & 0x1ff);
				assert_eq!(info.end_address(0x1000).unwrap(), 0x1000 + (0x3f * 4));
			},
			_ => panic!("unexpected variant"),
		}
	}

	#[test]
	fn decode_xdata_len() {
		let header = 0x3f | (1 << 20); // Length=0x3f, other valid header bits set
		assert_eq!(super::decode_xdata_function_length(header), Ok(Some(0xFC)));
		assert_eq!(super::decode_xdata_function_length(0), Ok(None));
	}

	#[test]
	fn reject_unknown_xdata_version() {
		for version in 1..=3 {
			let header = 0x3f | (version << 18);
			assert_eq!(super::decode_xdata_function_length(header), Err(Error::Invalid));
		}
	}
}
