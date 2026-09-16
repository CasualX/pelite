use super::*;

//----------------------------------------------------------------

/// x64 exception directory containing the image's runtime function table.
///
/// See Microsoft's [x64 exception handling documentation][docs].
///
/// [docs]: https://learn.microsoft.com/en-us/cpp/build/exception-handling-x64?view=msvc-170
#[derive(Copy, Clone)]
pub struct X64ExceptionDirectory<'a, P> {
	pe: P,
	image: &'a [RUNTIME_FUNCTION],
}
impl<'a, P: Pe<'a>> X64ExceptionDirectory<'a, P> {
	/// Parses the X64 exception directory for the given PE.
	pub(crate) fn try_from(pe: P) -> Result<X64ExceptionDirectory<'a, P>> {
		let datadir = pe.data_directory().get(IMAGE_DIRECTORY_ENTRY_EXCEPTION).ok_or(Error::Bounds)?;
		if datadir.VirtualAddress == 0 {
			return Err(Error::Null);
		}
		if pe.file_header().Machine != IMAGE_FILE_MACHINE_AMD64 {
			return Err(Error::Invalid);
		}
		let len = datadir.Size as usize / mem::size_of::<RUNTIME_FUNCTION>();
		let rem = datadir.Size as usize % mem::size_of::<RUNTIME_FUNCTION>();
		if rem != 0 {
			return Err(Error::Invalid);
		}
		let image = pe.derva_slice(datadir.VirtualAddress, len)?;
		Ok(X64ExceptionDirectory { pe, image })
	}
	/// Gets the PE instance.
	pub fn pe(&self) -> P {
		self.pe
	}
	/// Returns the functions slice.
	pub fn image(&self) -> &'a [RUNTIME_FUNCTION] {
		self.image
	}
	/// Checks if the function table is sorted.
	///
	/// The PE specification says that the list of runtime functions should be sorted to allow binary search.
	/// This function checks if the runtime functions are actually sorted, if not then lookups may fail unexpectedly.
	pub fn check_sorted(&self) -> bool {
		#[rustfmt::skip]
		fn check_sorted(window: &[RUNTIME_FUNCTION]) -> bool {
			return
				window[0].BeginAddress <= window[0].EndAddress &&
				window[0].EndAddress <= window[1].BeginAddress &&
				window[1].BeginAddress <= window[1].EndAddress;
		}
		self.image.windows(2).all(check_sorted)
	}
	/// Gets an iterator over the function records.
	pub fn functions(&self) -> iter::Map<slice::Iter<'a, RUNTIME_FUNCTION>, impl Clone + FnMut(&'a RUNTIME_FUNCTION) -> X64RuntimeFunction<'a, P>> {
		let pe = self.pe;
		self.image.iter().map(move |image| X64RuntimeFunction { pe, image })
	}
	/// Finds the index of the function for the given program counter.
	pub fn index_of(&self, pc: Rva) -> core::result::Result<usize, usize> {
		self.image.binary_search_by(|rf| {
			if pc < rf.BeginAddress {
				cmp::Ordering::Less
			}
			else if pc > rf.EndAddress {
				cmp::Ordering::Greater
			}
			else {
				cmp::Ordering::Equal
			}
		})
	}
	/// Finds the function for the given 'program counter' address.
	///
	/// The function records are sorted by their address allowing binary search for the record.
	pub fn lookup_function_entry(&self, pc: Rva) -> Option<X64RuntimeFunction<'a, P>> {
		self.index_of(pc)
			.map(|index| X64RuntimeFunction {
				pe: self.pe,
				image: &self.image[index],
			})
			.ok()
	}
}
impl<'a, P: Pe<'a>> fmt::Debug for X64ExceptionDirectory<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("X64ExceptionDirectory")
			.field("functions", &crate::util::DebugList(self.functions()))
			.finish()
	}
}

//----------------------------------------------------------------

/// Runtime function.
#[derive(Copy, Clone)]
pub struct X64RuntimeFunction<'a, P> {
	pe: P,
	image: &'a RUNTIME_FUNCTION,
}
impl<'a, P: Pe<'a>> X64RuntimeFunction<'a, P> {
	/// Gets the PE instance.
	pub fn pe(&self) -> P {
		self.pe
	}
	/// Returns the underlying runtime function image.
	pub fn image(&self) -> &'a RUNTIME_FUNCTION {
		self.image
	}
	/// Gets the function bytes.
	pub fn bytes(&self) -> Result<&'a [u8]> {
		let len = if self.image.BeginAddress > self.image.EndAddress {
			return Err(Error::Overflow);
		}
		else {
			(self.image.EndAddress - self.image.BeginAddress) as usize
		};
		self.pe.derva_slice(self.image.BeginAddress, len)
	}
	/// Gets the unwind info.
	pub fn unwind_info(&self) -> Result<X64UnwindInfo<'a, P>> {
		// Read as many bytes as we can for interpretation
		let bytes = self.pe.slice(
			self.image.UnwindData,
			mem::size_of::<UNWIND_INFO>(),
			mem::align_of::<UNWIND_INFO>(),
		)?;
		let image = unsafe { &*(bytes.as_ptr() as *const UNWIND_INFO) };
		// Calculate actual size including size of unwind codes
		let min_size_of = mem::size_of::<UNWIND_INFO>() + mem::size_of::<UNWIND_CODE>() * image.CountOfCodes as usize;
		if bytes.len() < min_size_of {
			return Err(Error::Bounds);
		}
		// Ok
		Ok(X64UnwindInfo { pe: self.pe, image })
	}
}
#[rustfmt::skip]
impl<'a, P: Pe<'a>> fmt::Debug for X64RuntimeFunction<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let image = self.image();
		f.debug_struct("X64RuntimeFunction")
			.field("begin", &format_args!("{:#010x}", image.BeginAddress))
			.field("end", &format_args!("{:#010x}", image.EndAddress))
			.field("size", &format_args!("{:#06x}", image.EndAddress.saturating_sub(image.BeginAddress)))
			.field("unwind", &format_args!("{:#010x}", image.UnwindData))
			.field("bytes.len", &self.bytes().map(<[_]>::len))
			.finish()
	}
}

//----------------------------------------------------------------

/// Unwind info.
#[derive(Copy, Clone)]
pub struct X64UnwindInfo<'a, P> {
	pe: P,
	image: &'a UNWIND_INFO,
}
impl<'a, P: Pe<'a>> X64UnwindInfo<'a, P> {
	/// Gets the PE instance.
	pub fn pe(&self) -> P {
		self.pe
	}
	/// Returns the underlying unwind info image.
	pub fn image(&self) -> &'a UNWIND_INFO {
		self.image
	}
	/// Returns the unwind information format version.
	pub fn version(&self) -> u8 {
		self.image.VersionFlags & 0b00000111
	}
	/// Returns the unwind flags.
	pub fn flags(&self) -> u8 {
		self.image.VersionFlags >> 3
	}
	/// Returns the prolog size in bytes.
	pub fn size_of_prolog(&self) -> usize {
		self.image.SizeOfProlog as usize
	}
	/// Returns the frame register number, or zero if none is used.
	pub fn frame_register(&self) -> u8 {
		self.image.FrameRegisterOffset & 0b00001111
	}
	/// Returns the scaled frame-register offset.
	pub fn frame_offset(&self) -> u8 {
		self.image.FrameRegisterOffset >> 4
	}
	/// Returns the unwind operation codes.
	pub fn unwind_codes(&self) -> &'a [UNWIND_CODE] {
		let len = self.image.CountOfCodes as usize;
		unsafe { slice::from_raw_parts(self.image.UnwindCode.as_ptr(), len) }
	}
}
impl<'a, P: Pe<'a>> fmt::Debug for X64UnwindInfo<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("X64UnwindInfo")
			.field("version", &self.version())
			.field("flags", &self.flags())
			.field("size_of_prolog", &self.size_of_prolog())
			.field("frame_register", &self.frame_register())
			.field("frame_offset", &self.frame_offset())
			.field("unwind_codes.len", &self.unwind_codes().len())
			.finish()
	}
}

//----------------------------------------------------------------

serde_impl! {
	impl<'a, P: Pe<'a>> Serialize for X64ExceptionDirectory<'a, P> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			serializer.collect_seq(self.functions())
		}
	}

	impl<'a, P: Pe<'a>> Serialize for X64RuntimeFunction<'a, P> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let mut state = serializer.serialize_struct("X64RuntimeFunction", 2)?;
			state.serialize_field("image", self.image())?;
			state.serialize_field("unwind_info", &self.unwind_info().ok())?;
			state.end()
		}
	}

	impl<'a, P: Pe<'a>> Serialize for X64UnwindInfo<'a, P> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let mut state = serializer.serialize_struct("X64UnwindInfo", 6)?;
			state.serialize_field("version", &self.version())?;
			state.serialize_field("flags", &self.flags())?;
			state.serialize_field("size_of_prolog", &self.size_of_prolog())?;
			state.serialize_field("frame_register", &self.frame_register())?;
			state.serialize_field("frame_offset", &self.frame_offset())?;
			state.serialize_field("unwind_codes", &self.unwind_codes())?;
			state.end()
		}
	}
}

//----------------------------------------------------------------

#[cfg(test)]
pub(crate) fn test_exception_x64<'a, P: Pe<'a>>(pe: P) -> Result<()> {
	let exception = pe.exception_x64()?;
	let _ = format!("{:?}", exception);

	let sorted = exception.check_sorted();

	for (index, function) in exception.functions().enumerate() {
		let _ = format!("{:?}", function);
		let _bytes = function.bytes();

		if sorted {
			for pc in function.image().BeginAddress..function.image().EndAddress {
				assert_eq!(exception.index_of(pc), Ok(index));
			}
		}

		if let Ok(unwind_info) = function.unwind_info() {
			let _ = format!("{:?}", unwind_info);
			let _version = unwind_info.version();
			let _flags = unwind_info.flags();
			let _size_of_prolog = unwind_info.size_of_prolog();
			let _frame_register = unwind_info.frame_register();
			let _frame_offset = unwind_info.frame_offset();
			let _unwind_codes = unwind_info.unwind_codes();
		}
	}

	Ok(())
}
