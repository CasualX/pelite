extern crate proc_macro;

use proc_macro::*;

/// Auto derive the `Pod` trait for structs.
///
/// The type is checked for requirements of the `Pod` trait:
///
/// * Require `#[repr(C)]` annotation.
/// * Require all struct members to implement `Pod`.
/// * Require the size of the struct equal to the sum of the sizes of its members.
///
#[proc_macro_derive(Pod)]
pub fn pod_derive(input: TokenStream) -> TokenStream {
	return format!("::pelite::derive_pod!{{ {} }}", input).parse().unwrap()
}

/// Implement the `Pod` trait for use in the `pelite` crate itself.
#[doc(hidden)]
#[proc_macro_derive(_Pod)]
pub fn _pod_derive(input: TokenStream) -> TokenStream {
	return format!("crate::derive_pod!{{ {} }}", input).parse().unwrap()
}
