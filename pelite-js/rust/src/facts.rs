//! Independently owned factmap layers. Mutations validate before changing storage.

use super::*;

/// Failures reported by the facts WASM API.
#[derive(Debug)]
enum Error {
	Utf8(str::Utf8Error),
	IndexOutOfBounds,
	InvalidPointerWidth,
	ParseMap(binfact::ParseError),
	ParseFact(binfact::ParseLineError),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Utf8(err) => err.fmt(f),
			Self::IndexOutOfBounds => f.write_str("index out of bounds"),
			Self::InvalidPointerWidth => f.write_str("pointer width must be 32 or 64"),
			Self::ParseMap(err) => err.fmt(f),
			Self::ParseFact(err) => err.fmt(f),
		}
	}
}

impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			Self::Utf8(err) => Some(err),
			Self::ParseMap(err) => Some(err),
			Self::ParseFact(err) => Some(err),
			Self::IndexOutOfBounds | Self::InvalidPointerWidth => None,
		}
	}
}

/// Resolve negative indices from the end without narrowing or overflowing.
#[inline]
fn index(i: i32, len: usize) -> Option<usize> {
	let resolved = if i < 0 { len.checked_sub(i.unsigned_abs() as usize) } else { Some(i as usize) };
	resolved.filter(|&i| i < len)
}

/// Owned WASM handle, independent of a PE image.
#[derive(Clone)]
pub struct Facts {
	layers: Vec<Layer>,
	pointer_width: binfact::ty::PointerWidth,
}

#[derive(Clone, Default)]
pub struct Layer {
	pub label: String,
	// Retain interleaved fact kinds and duplicate records in their original order.
	pub map: binfact::FactMap,
}

impl Facts {
	pub fn len(&self) -> usize {
		self.layers.len()
	}
	pub fn get(&self, i: i32) -> Option<&Layer> {
		self.layers.get(index(i, self.layers.len())?)
	}
	pub fn get_mut(&mut self, i: i32) -> Option<&mut Layer> {
		let i = index(i, self.layers.len())?;
		self.layers.get_mut(i)
	}
	pub fn add(&mut self, map: binfact::FactMap) -> usize {
		let i = self.layers.len();
		self.layers.push(Layer { label: String::new(), map });
		i
	}
	pub fn delete(&mut self, i: i32) -> Option<()> {
		let i = index(i, self.layers.len())?;
		self.layers.remove(i);
		Some(())
	}
	pub fn merge(&mut self, base: i32, i: i32) -> Option<()> {
		let base = index(base, self.layers.len())?;
		let i = index(i, self.layers.len())?;
		if base != i {
			let other = self.layers.remove(i);
			let base = if base > i { base - 1 } else { base };
			self.layers[base].map.facts.extend(other.map.facts);
		}
		Some(())
	}
	pub fn shift(&mut self, i: i32, destination: i32) -> Option<()> {
		let i = index(i, self.layers.len())?;
		let destination = index(destination, self.layers.len())?;
		let layer = self.layers.remove(i);
		self.layers.insert(destination, layer);
		Some(())
	}
	pub fn get_label(&self, i: i32) -> Option<&str> {
		Some(&self.get(i)?.label)
	}
	pub fn set_label(&mut self, i: i32, label: &str) -> Option<()> {
		self.get_mut(i)?.label = label.into();
		Some(())
	}
	pub fn add_fact(&mut self, i: i32, fact: binfact::Fact) -> Option<()> {
		self.get_mut(i)?.map.facts.push(fact);
		Some(())
	}
}

#[unsafe(export_name = "factsNew")]
pub fn new(pointer_width: u32) -> *mut Facts {
	let pointer_width = match pointer_width {
		32 => binfact::ty::PointerWidth::X32,
		64 => binfact::ty::PointerWidth::X64,
		_ => {
			return_error(Error::InvalidPointerWidth);
			return ptr::null_mut();
		},
	};
	Box::into_raw(Box::new(Facts { layers: Vec::new(), pointer_width }))
}

#[unsafe(export_name = "factsDrop")]
pub unsafe fn drop(handle: *mut Facts) {
	let _ = unsafe { Box::from_raw(handle) };
}

fn return_result<T: serde::Serialize, E: ToString>(result: Result<T, E>) {
	match result {
		Ok(value) => return_json(value),
		Err(err) => return_error(err),
	}
}

#[unsafe(export_name = "factsGetNumLayers")]
pub unsafe fn get_num_layers(handle: *const Facts) {
	return_json(unsafe { &*handle }.len());
}

#[unsafe(export_name = "factsGetLayer")]
pub unsafe fn get_layer(handle: *const Facts, i: i32) {
	return_result(unsafe { &*handle }.get(i)
		.map(|layer| layer.map.to_string())
		.ok_or(Error::IndexOutOfBounds));
}

#[unsafe(export_name = "factsAddLayer")]
pub unsafe fn add_layer(handle: *mut Facts, facts: *const [u8]) {
	let handle = unsafe { &mut *handle };
	return_result((|| -> Result<_, Error> {
		let facts = str::from_utf8(unsafe { &*facts }).map_err(Error::Utf8)?;
		let width = handle.pointer_width;
		let map = binfact::FactMap::parse(facts, width).map_err(Error::ParseMap)?;
		Ok(handle.add(map))
	})());
}

#[unsafe(export_name = "factsDeleteLayer")]
pub unsafe fn delete_layer(handle: *mut Facts, i: i32) {
	return_result(unsafe { &mut *handle }.delete(i).ok_or(Error::IndexOutOfBounds));
}

#[unsafe(export_name = "factsMergeLayer")]
pub unsafe fn merge_layer(handle: *mut Facts, base: i32, i: i32) {
	return_result(unsafe { &mut *handle }.merge(base, i).ok_or(Error::IndexOutOfBounds));
}

#[unsafe(export_name = "factsShiftLayer")]
pub unsafe fn shift_layer(handle: *mut Facts, i: i32, destination: i32) {
	return_result(unsafe { &mut *handle }.shift(i, destination).ok_or(Error::IndexOutOfBounds));
}

#[unsafe(export_name = "factsGetLayerLabel")]
pub unsafe fn get_label(handle: *const Facts, i: i32) {
	return_result(unsafe { &*handle }.get_label(i).ok_or(Error::IndexOutOfBounds));
}

#[unsafe(export_name = "factsSetLayerLabel")]
pub unsafe fn set_label(handle: *mut Facts, i: i32, label: *const [u8]) {
	return_result((|| -> Result<_, Error> {
		let label = str::from_utf8(unsafe { &*label }).map_err(Error::Utf8)?;
		unsafe { &mut *handle }.set_label(i, label)
			.ok_or(Error::IndexOutOfBounds)
	})());
}

#[unsafe(export_name = "factsAddFact")]
pub unsafe fn add_fact(handle: *mut Facts, i: i32, fact: *const [u8]) {
	let handle = unsafe { &mut *handle };
	return_result((|| -> Result<_, Error> {
		let fact = str::from_utf8(unsafe { &*fact }).map_err(Error::Utf8)?;
		let width = handle.pointer_width;
		let fact = binfact::Fact::parse(fact.trim_ascii(), width).map_err(Error::ParseFact)?;
		handle.add_fact(i, fact).ok_or(Error::IndexOutOfBounds)
	})());
}
