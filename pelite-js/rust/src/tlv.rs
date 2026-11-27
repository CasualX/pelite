//! Private wire format shared with `decodeTlv` in `src/pelite.js`.
//! A one-byte tag determines a fixed payload, a u8 or u32 byte length, or a
//! container terminated by End. Numeric payloads and lengths are little-endian.

use std::fmt;

#[repr(u8)]
enum Tag {
	False = 0,
	True = 1,
	U8 = 2,
	U16 = 3,
	U32 = 4,
	U64 = 5,
	U128 = 6,
	I8 = 7,
	I16 = 8,
	I32 = 9,
	I64 = 10,
	I128 = 11,
	F32 = 12,
	F64 = 13,
	StringU8len = 14,
	StringU32len = 15,
	Null = 16,
	BytesU8len = 17,
	BytesU32len = 18,
	Array = 19,
	Object = 20,
	End = 255,
}

impl Tag {
	fn is_safe_object_key(tag: u8) -> bool {
		// All valid object key tags precede Null.
		tag < Self::Null as u8
	}
}

#[derive(Debug)]
pub struct Error(String);

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&self.0)
	}
}

impl std::error::Error for Error {}

impl serde::ser::Error for Error {
	fn custom<T: fmt::Display>(message: T) -> Self {
		Self(message.to_string())
	}
}

pub fn to_vec(value: impl serde::ser::Serialize) -> Result<Vec<u8>, Error> {
	let mut encoder = Encoder { bytes: Vec::new() };
	value.serialize(&mut encoder)?;
	Ok(encoder.bytes)
}

struct Encoder {
	bytes: Vec<u8>,
}

impl Encoder {
	fn tag(&mut self, tag: Tag) {
		self.bytes.push(tag as u8);
	}

	fn blob(&mut self, u8_len_tag: Tag, u32_len_tag: Tag, bytes: &[u8]) -> Result<(), Error> {
		if let Ok(len) = u8::try_from(bytes.len()) {
			self.tag(u8_len_tag);
			self.bytes.push(len);
		}
		else {
			let len = u32::try_from(bytes.len()).map_err(|_| serde::ser::Error::custom("byte length exceeds u32"))?;
			self.tag(u32_len_tag);
			self.bytes.extend_from_slice(&len.to_le_bytes());
		}
		self.bytes.extend_from_slice(bytes);
		Ok(())
	}

	fn container(&mut self, tag: Tag) -> Compound<'_> {
		self.tag(tag);
		Compound { encoder: self, variant: false }
	}

	fn variant(&mut self, name: &str) -> Result<(), Error> {
		use serde::ser::Serializer;
		self.tag(Tag::Object);
		self.serialize_str(name)
	}

	fn variant_container(&mut self, name: &str, tag: Tag) -> Result<Compound<'_>, Error> {
		self.variant(name)?;
		self.tag(tag);
		Ok(Compound { encoder: self, variant: true })
	}
}

macro_rules! serialize_numbers {
	($($method:ident($ty:ty) => $tag:ident),* $(,)?) => {
		$(fn $method(self, value: $ty) -> Result<(), Error> {
			self.tag(Tag::$tag);
			self.bytes.extend_from_slice(&value.to_le_bytes());
			Ok(())
		})*
	};
}

impl<'a> serde::ser::Serializer for &'a mut Encoder {
	type Ok = ();
	type Error = Error;
	type SerializeSeq = Compound<'a>;
	type SerializeTuple = Compound<'a>;
	type SerializeTupleStruct = Compound<'a>;
	type SerializeTupleVariant = Compound<'a>;
	type SerializeMap = Compound<'a>;
	type SerializeStruct = Compound<'a>;
	type SerializeStructVariant = Compound<'a>;

	fn serialize_bool(self, value: bool) -> Result<(), Error> {
		self.tag(if value { Tag::True } else { Tag::False });
		Ok(())
	}

	serialize_numbers! {
		serialize_u8(u8) => U8,
		serialize_u16(u16) => U16,
		serialize_u32(u32) => U32,
		serialize_u64(u64) => U64,
		serialize_u128(u128) => U128,

		serialize_i8(i8) => I8,
		serialize_i16(i16) => I16,
		serialize_i32(i32) => I32,
		serialize_i64(i64) => I64,
		serialize_i128(i128) => I128,

		serialize_f32(f32) => F32,
		serialize_f64(f64) => F64,
	}

	fn serialize_char(self, value: char) -> Result<(), Error> {
		self.serialize_str(value.encode_utf8(&mut [0; 4]))
	}

	fn serialize_str(self, value: &str) -> Result<(), Error> {
		self.blob(Tag::StringU8len, Tag::StringU32len, value.as_bytes())
	}

	fn serialize_bytes(self, value: &[u8]) -> Result<(), Error> {
		self.blob(Tag::BytesU8len, Tag::BytesU32len, value)
	}

	fn serialize_none(self) -> Result<(), Error> {
		self.serialize_unit()
	}

	fn serialize_unit(self) -> Result<(), Error> {
		self.tag(Tag::Null);
		Ok(())
	}

	fn serialize_unit_struct(self, _name: &'static str) -> Result<(), Error> {
		self.serialize_unit()
	}

	fn serialize_unit_variant(self, _name: &'static str, _index: u32, variant: &'static str) -> Result<(), Error> {
		self.serialize_str(variant)
	}

	fn serialize_some<T: ?Sized + serde::ser::Serialize>(self, value: &T) -> Result<(), Error> {
		value.serialize(self)
	}

	fn serialize_newtype_struct<T: ?Sized + serde::ser::Serialize>(self, _name: &'static str, value: &T) -> Result<(), Error> {
		value.serialize(self)
	}

	fn serialize_newtype_variant<T: ?Sized + serde::ser::Serialize>(self, _name: &'static str, _index: u32, variant: &'static str, value: &T) -> Result<(), Error> {
		self.variant(variant)?;
		value.serialize(&mut *self)?;
		self.tag(Tag::End);
		Ok(())
	}

	fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Error> {
		Ok(self.container(Tag::Array))
	}

	fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Error> {
		self.serialize_seq(Some(len))
	}

	fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Error> {
		self.serialize_tuple(len)
	}

	fn serialize_tuple_variant(self, _name: &'static str, _index: u32, variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Error> {
		self.variant_container(variant, Tag::Array)
	}

	fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Error> {
		Ok(self.container(Tag::Object))
	}

	fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Error> {
		Ok(self.container(Tag::Object))
	}

	fn serialize_struct_variant(self, _name: &'static str, _index: u32, variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Error> {
		self.variant_container(variant, Tag::Object)
	}

	fn is_human_readable(&self) -> bool {
		// Pelite uses this flag for its existing display names and base64 fields.
		true
	}
}

struct Compound<'a> {
	encoder: &'a mut Encoder,
	variant: bool,
}

impl Compound<'_> {
	fn element<T: ?Sized + serde::ser::Serialize>(&mut self, value: &T) -> Result<(), Error> {
		value.serialize(&mut *self.encoder)
	}

	fn field<T: ?Sized + serde::ser::Serialize>(&mut self, key: &'static str, value: &T) -> Result<(), Error> {
		use serde::ser::Serializer;
		self.encoder.serialize_str(key)?;
		self.element(value)
	}

	fn finish(self) -> Result<(), Error> {
		self.encoder.tag(Tag::End);
		if self.variant {
			self.encoder.tag(Tag::End);
		}
		Ok(())
	}
}

macro_rules! serialize_compound {
	($($trait:ident::$method:ident($($key:ident: $ty:ty)?) => $write:ident),* $(,)?) => {
		$(impl serde::ser::$trait for Compound<'_> {
			type Ok = ();
			type Error = Error;

			fn $method<T: ?Sized + serde::ser::Serialize>(&mut self, $($key: $ty,)? value: &T) -> Result<(), Error> {
				self.$write($($key,)? value)
			}

			fn end(self) -> Result<(), Error> {
				self.finish()
			}
		})*
	};
}

serialize_compound! {
	SerializeSeq::serialize_element() => element,
	SerializeTuple::serialize_element() => element,
	SerializeTupleStruct::serialize_field() => element,
	SerializeTupleVariant::serialize_field() => element,
	SerializeStruct::serialize_field(key: &'static str) => field,
	SerializeStructVariant::serialize_field(key: &'static str) => field,
}

impl serde::ser::SerializeMap for Compound<'_> {
	type Ok = ();
	type Error = Error;

	fn serialize_key<T: ?Sized + serde::ser::Serialize>(&mut self, key: &T) -> Result<(), Error> {
		let offset = self.encoder.bytes.len();
		self.element(key)?;
		let Some(&tag) = self.encoder.bytes.get(offset) else {
			return Err(serde::ser::Error::custom("map key did not serialize a value"));
		};
		if !Tag::is_safe_object_key(tag) {
			return Err(serde::ser::Error::custom("unsupported map key type"));
		}
		Ok(())
	}

	fn serialize_value<T: ?Sized + serde::ser::Serialize>(&mut self, value: &T) -> Result<(), Error> {
		self.element(value)
	}

	fn end(self) -> Result<(), Error> {
		self.finish()
	}
}
