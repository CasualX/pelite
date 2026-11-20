//! Authenticode security directory parsing.

use core::{fmt, iter, mem};

use crate::image::WIN_CERTIFICATE;
use crate::util::AlignTo;
use crate::{Error, Result};

/// Security directory.
///
/// Certificates are stored in the file and are not mapped into the loaded image.
///
/// # Examples
///
/// ```
/// # #![allow(unused_variables)]
/// use pelite::pe64::PeFile;
///
/// # #[allow(dead_code)]
/// fn example(file: PeFile<'_>) -> pelite::Result<()> {
/// 	for certificate in file.security()? {
/// 		let certificate = certificate?;
/// 		let payload = certificate.payload();
/// 	}
/// 	Ok(())
/// }
/// ```
#[derive(Copy, Clone)]
pub struct SecurityDirectory<'a> {
	image: &'a [u8],
}
impl<'a> SecurityDirectory<'a> {
	/// # Safety
	/// The image must be aligned for `WIN_CERTIFICATE`.
	pub(crate) unsafe fn new(image: &'a [u8]) -> Self {
		debug_assert!(image.as_ptr().aligned_to(mem::align_of::<WIN_CERTIFICATE>()));
		Self { image }
	}
	/// Returns the raw certificate table, including any padding.
	pub fn image(&self) -> &'a [u8] {
		self.image
	}
	/// Returns an iterator over the certificates in file order.
	///
	/// The first entry with a truncated header, invalid length, or truncated padding
	/// yields [`Error::Bounds`] and ends iteration.
	pub fn iter(&self) -> SecurityDirectoryIter<'a> {
		SecurityDirectoryIter { image: self.image }
	}
}
impl<'a> IntoIterator for SecurityDirectory<'a> {
	type Item = Result<Certificate<'a>>;
	type IntoIter = SecurityDirectoryIter<'a>;
	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}
impl fmt::Debug for SecurityDirectory<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("SecurityDirectory")
			.field("certificates", &crate::util::DebugList(self.iter()))
			.finish()
	}
}

/// Iterator over the entries of a security directory.
#[derive(Clone)]
pub struct SecurityDirectoryIter<'a> {
	image: &'a [u8],
}
impl<'a> Iterator for SecurityDirectoryIter<'a> {
	type Item = Result<Certificate<'a>>;
	fn next(&mut self) -> Option<Self::Item> {
		let image = mem::take(&mut self.image);
		if image.is_empty() {
			return None;
		}
		let header_size = mem::size_of::<WIN_CERTIFICATE>();
		if image.len() < header_size {
			return Some(Err(Error::Bounds));
		}
		// SAFETY: The table starts at an aligned address and advances in multiples of eight bytes.
		let header = unsafe { &*image.as_ptr().cast::<WIN_CERTIFICATE>() };
		let length = header.dwLength as usize;
		let Some(payload) = image.get(header_size..length) else {
			return Some(Err(Error::Bounds));
		};
		// The checked slice range above bounds length by isize::MAX, so rounding up fits in usize.
		let Some(remaining) = image.get(length.next_multiple_of(8)..) else {
			return Some(Err(Error::Bounds));
		};
		self.image = remaining;
		Some(Ok(Certificate { image: header, payload }))
	}
}
impl iter::FusedIterator for SecurityDirectoryIter<'_> {}

/// A certificate table entry and its raw payload.
#[derive(Copy, Clone)]
pub struct Certificate<'a> {
	image: &'a WIN_CERTIFICATE,
	payload: &'a [u8],
}
impl<'a> Certificate<'a> {
	/// Returns the raw certificate header.
	pub fn image(&self) -> &'a WIN_CERTIFICATE {
		self.image
	}
	/// Returns the certificate type.
	///
	/// Known types include
	/// [X.509][crate::image::WIN_CERT_TYPE_X509],
	/// [PKCS SignedData][crate::image::WIN_CERT_TYPE_PKCS_SIGNED_DATA], and
	/// [PKCS1 module sign][crate::image::WIN_CERT_TYPE_PKCS1_SIGN].
	pub fn certificate_type(&self) -> u16 {
		self.image.wCertificateType
	}
	/// Returns the bytes following the header up to the entry's declared length.
	///
	/// The format depends on the [certificate type][Self::certificate_type]. The
	/// declared length can include padding. With `serde`, human-readable serializers
	/// encode the payload as base64. Binary serializers receive raw bytes.
	pub fn payload(&self) -> &'a [u8] {
		self.payload
	}
}
impl fmt::Debug for Certificate<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Certificate")
			.field("image", self.image)
			.field("payload.len", &self.payload.len())
			.finish()
	}
}

serde_impl! {
	impl Serialize for SecurityDirectory<'_> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			#[derive(serde::Serialize)]
			struct EntryError {
				#[serde(rename = "$error")]
				error: &'static str,
			}
			let iter = self.iter();
			// Some binary serializers require the sequence length up front.
			let len = (!serializer.is_human_readable()).then(|| iter.clone().count());
			let mut seq = serializer.serialize_seq(len)?;
			for entry in iter {
				match entry {
					Ok(certificate) => seq.serialize_element(&certificate)?,
					Err(error) => seq.serialize_element(&EntryError { error: error.to_str() })?,
				}
			}
			seq.end()
		}
	}

	impl Serialize for Certificate<'_> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let mut state = serializer.serialize_struct("Certificate", 2)?;
			state.serialize_field("image", self.image)?;
			state.serialize_field("payload", &Payload(self.payload))?;
			state.end()
		}
	}

	struct Payload<'a>(&'a [u8]);
	impl Serialize for Payload<'_> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let &Payload(payload) = self;
			if serializer.is_human_readable() {
				return serializer.serialize_str(&basenc::Base64Std.encode(payload));
			}
			else {
				serializer.serialize_bytes(payload)
			}
		}
	}
}
