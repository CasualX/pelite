use super::*;

impl<'a> PeFile<'a> {
	#[doc = include_str!("../docs/tls.md")]
	#[inline]
	pub fn tls(self) -> Result<TlsDirectory<'a, Self>> {
		TlsDirectory::try_from(self)
	}
}

impl<'a> PeView<'a> {
	#[doc = include_str!("../docs/tls.md")]
	#[inline]
	pub fn tls(self) -> Result<TlsDirectory<'a, Self>> {
		TlsDirectory::try_from(self)
	}
}

//----------------------------------------------------------------

/// TLS Directory.
///
/// # Examples
///
/// ```
/// # #![allow(unused_variables)]
/// use pelite::pe64::{Pe, PeFile};
///
/// # #[allow(dead_code)]
/// fn example(file: PeFile<'_>) -> pelite::Result<()> {
/// 	// Access the TLS directory
/// 	let tls = file.tls()?;
///
/// 	// Access the initialized thread-local data
/// 	let raw_data = tls.raw_data()?;
///
/// 	// Access the TLS slot
/// 	let slot = tls.slot()?;
///
/// 	// Access the TLS callbacks
/// 	let callbacks = tls.callbacks()?;
///
/// 	Ok(())
/// }
/// ```
#[derive(Copy, Clone)]
pub struct TlsDirectory<'a, P> {
	pe: P,
	image: &'a IMAGE_TLS_DIRECTORY,
}
impl<'a, P: Copy + Pe<'a>> TlsDirectory<'a, P> {
	pub(crate) fn try_from(pe: P) -> Result<TlsDirectory<'a, P>> {
		let datadir = pe.data_directory().get(IMAGE_DIRECTORY_ENTRY_TLS).ok_or(Error::Bounds)?;
		if datadir.VirtualAddress == 0 {
			return Err(Error::Null);
		}
		if datadir.Size < mem::size_of::<IMAGE_TLS_DIRECTORY>() as u32 {
			return Err(Error::Bounds);
		}
		let image = pe.derva(datadir.VirtualAddress)?;
		Ok(TlsDirectory { pe, image })
	}
	/// Returns the PE instance.
	pub fn pe(&self) -> P {
		self.pe
	}
	/// Returns the underlying TLS directory image.
	pub fn image(&self) -> &'a IMAGE_TLS_DIRECTORY {
		self.image
	}
	/// Returns the initialized TLS data.
	///
	/// # Errors
	///
	/// * [`Invalid`][crate::Error::Invalid]: The data range is reversed or its raw section range is malformed.
	/// * [`Overflow`][crate::Error::Overflow]: The declared data length cannot fit in `usize`.
	/// * [`Null`][crate::Error::Null], [`Bounds`][crate::Error::Bounds], [`Misaligned`][crate::Error::Misaligned], or [`ZeroFill`][crate::Error::ZeroFill]: The data cannot be read from the image.
	pub fn raw_data(&self) -> Result<&'a [u8]> {
		if self.image.StartAddressOfRawData > self.image.EndAddressOfRawData {
			return Err(Error::Invalid);
		}
		let len = usize::try_from(self.image.EndAddressOfRawData - self.image.StartAddressOfRawData).map_err(|_| Error::Overflow)?;
		self.pe.deref_slice(self.image.StartAddressOfRawData.into(), len)
	}
	/// Returns the TLS slot location.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]: The slot address is zero.
	/// * [`Bounds`][crate::Error::Bounds], [`Misaligned`][crate::Error::Misaligned], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The slot cannot be read from the image.
	pub fn slot(&self) -> Result<&'a u32> {
		self.pe.deref(self.image.AddressOfIndex.into())
	}
	/// Returns the TLS callback addresses.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]: The callback pointer is null.
	/// * [`Bounds`][crate::Error::Bounds]: The callbacks have no terminating zero or lie outside the image.
	/// * [`Misaligned`][crate::Error::Misaligned], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The callback array cannot be read.
	pub fn callbacks(&self) -> Result<&'a [Va]> {
		self.pe.deref_slice_s(self.image.AddressOfCallBacks.into(), 0)
	}
}
impl<'a, P: Copy + Pe<'a>> fmt::Debug for TlsDirectory<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("TlsDirectory")
			.field("raw_data.len", &format_args!("{:?}", self.raw_data().map(|raw_data| raw_data.len())))
			.field("callbacks.len", &format_args!("{:?}", &self.callbacks().map(|cbs| cbs.len())))
			.finish()
	}
}

//----------------------------------------------------------------

serde_impl! {
	impl<'a, P: Copy + Pe<'a>> Serialize for TlsDirectory<'a, P> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let is_human_readable = serializer.is_human_readable();
			let mut state = serializer.serialize_struct("TlsDirectory", 4)?;
			state.serialize_field("image", self.image())?;
			if cfg!(feature = "basenc") && is_human_readable {
				#[cfg(feature = "basenc")]
				state.serialize_field("raw_data", &self.raw_data().ok().map(|data| basenc::Base64Std.encode(data)))?;
			}
			else {
				state.serialize_field("raw_data", &self.raw_data().ok())?;
			}
			state.serialize_field("slot", &self.slot().ok())?;
			state.serialize_field("callbacks", &self.callbacks().ok())?;
			state.end()
		}
	}
}

//----------------------------------------------------------------

#[cfg(test)]
pub(crate) fn test_tls<'a, P: Copy + Pe<'a>>(pe: P) -> Result<()> {
	let tls = pe.tls()?;
	let _ = format!("{:?}", tls);
	let _raw_data = tls.raw_data();
	let _slot = tls.slot();
	let _callbacks = tls.callbacks();
	Ok(())
}
