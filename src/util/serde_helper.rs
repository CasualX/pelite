#[allow(unused_imports)] // Lint false positive
pub(crate) use serde::ser::{Serialize, Serializer, SerializeStruct, SerializeMap, SerializeSeq};

pub(crate) struct SerdeIter<I>(pub(crate) I);
impl<T: Serialize, I: Clone + Iterator<Item = T>> Serialize for SerdeIter<I> {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		serializer.collect_seq(self.0.clone())
	}
}

pub(crate) struct SerdeKV<I>(pub(crate) I);
impl<K: Serialize, V: Serialize, I: Clone + Iterator<Item = (K, V)>> Serialize for SerdeKV<I> {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		serializer.collect_map(self.0.clone())
	}
}

pub(crate) fn serde_strn<S: Serializer>(name: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
	::std::str::from_utf8(super::strn(name)).ok().serialize(serializer)
}
