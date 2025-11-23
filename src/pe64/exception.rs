/*!
Exception Directory.
*/

use std::cmp::Ordering;
use std::{fmt, iter, mem, slice};

use crate::{Error, Result};

use super::image::*;
use super::Pe;

//----------------------------------------------------------------

/// Exception Directory.
///
/// For more information see the [module-level documentation](index.html).
#[derive(Copy, Clone)]
pub struct Exception<'a, P> {
	pe: P,
	image: &'a [RUNTIME_FUNCTION],
}
impl<'a, P: Pe<'a>> Exception<'a, P> {
	pub(crate) fn try_from(pe: P) -> Result<Exception<'a, P>> {
		let datadir = pe.data_directory().get(IMAGE_DIRECTORY_ENTRY_EXCEPTION).ok_or(Error::Bounds)?;
		let len = datadir.Size as usize / mem::size_of::<RUNTIME_FUNCTION>();
		let rem = datadir.Size as usize % mem::size_of::<RUNTIME_FUNCTION>();
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
	pub fn functions(&self) -> iter::Map<slice::Iter<'a, RUNTIME_FUNCTION>, impl Clone + FnMut(&'a RUNTIME_FUNCTION) -> Function<'a, P>> {
		let pe = self.pe;
		self.image.iter().map(move |image| Function { pe, image })
	}
	/// Finds the index of the function for the given program counter.
	pub fn index_of(&self, pc: Rva) -> std::result::Result<usize, usize> {
		self.image.binary_search_by(|rf| {
			if pc < rf.BeginAddress {
				Ordering::Less
			}
			else if pc > rf.EndAddress {
				Ordering::Greater
			}
			else {
				Ordering::Equal
			}
		})
	}
	/// Finds the function for the given 'program counter' address.
	///
	/// The function records are sorted by their address allowing binary search for the record.
	pub fn lookup_function_entry(&self, pc: Rva) -> Option<Function<'a, P>> {
		self.index_of(pc)
			.map(|index| Function {
				pe: self.pe,
				image: &self.image[index],
			})
			.ok()
	}
}
#[rustfmt::skip]
impl<'a, P: Pe<'a>> fmt::Debug for Exception<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f, "Exception {{")?;
		writeln!(f, "    functions.len: {},", self.image.len())?;
		writeln!(f, "    functions: [")?;
		for (index, function) in self.functions().enumerate() {
			let image = function.image();
			let size = image.EndAddress.saturating_sub(image.BeginAddress);
			writeln!(
				f,
				"        [{:04}] begin=0x{:08x} end=0x{:08x} size=0x{:04x} unwind=0x{:08x},",
				index,
				image.BeginAddress,
				image.EndAddress,
				size,
				image.UnwindData
			)?;
		}
		writeln!(f, "    ]")?;
		write!(f, "}}")
	}
}

//----------------------------------------------------------------

/// Runtime function.
#[derive(Copy, Clone)]
pub struct Function<'a, P> {
	pe: P,
	image: &'a RUNTIME_FUNCTION,
}
impl<'a, P: Pe<'a>> Function<'a, P> {
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
	pub fn unwind_info(&self) -> Result<UnwindInfo<'a, P>> {
		// Read as many bytes as we can for interpretation
		let bytes = self.pe.slice(
			self.image.UnwindData,
			mem::size_of::<UNWIND_INFO>(),
			if cfg!(feature = "unsafe_alignment") { 1 } else { mem::align_of::<UNWIND_INFO>() },
		)?;
		let image = unsafe { &*(bytes.as_ptr() as *const UNWIND_INFO) };
		// Calculate actual size including size of unwind codes
		let min_size_of = mem::size_of::<UNWIND_INFO>() + mem::size_of::<UNWIND_CODE>() * image.CountOfCodes as usize;
		if bytes.len() < min_size_of {
			return Err(Error::Bounds);
		}
		// Ok
		Ok(UnwindInfo { pe: self.pe, image })
	}
}
#[rustfmt::skip]
impl<'a, P: Pe<'a>> fmt::Debug for Function<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Function")
			.field("bytes.len", &self.bytes().map(<[_]>::len))
			.finish()
	}
}

//----------------------------------------------------------------

/// Unwind info.
#[derive(Copy, Clone)]
pub struct UnwindInfo<'a, P> {
	pe: P,
	image: &'a UNWIND_INFO,
}
impl<'a, P: Pe<'a>> UnwindInfo<'a, P> {
	/// Gets the PE instance.
	pub fn pe(&self) -> P {
		self.pe
	}
	/// Returns the underlying unwind info image.
	pub fn image(&self) -> &'a UNWIND_INFO {
		self.image
	}
	pub fn version(&self) -> u8 {
		self.image.VersionFlags & 0b00000111
	}
	pub fn flags(&self) -> u8 {
		self.image.VersionFlags >> 3
	}
	pub fn size_of_prolog(&self) -> usize {
		self.image.SizeOfProlog as usize
	}
	pub fn frame_register(&self) -> u8 {
		self.image.FrameRegisterOffset & 0b00001111
	}
	pub fn frame_offset(&self) -> u8 {
		self.image.FrameRegisterOffset >> 4
	}
	pub fn unwind_codes(&self) -> &'a [UNWIND_CODE] {
		let len = self.image.CountOfCodes as usize;
		unsafe { slice::from_raw_parts(self.image.UnwindCode.as_ptr(), len) }
	}
}
impl<'a, P: Pe<'a>> fmt::Debug for UnwindInfo<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("UnwindInfo")
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

#[cfg(test)]
pub(crate) fn test<'a, P: Pe<'a>>(pe: P) -> Result<()> {
	let exception = pe.exception()?;
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
