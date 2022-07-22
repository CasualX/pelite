/*!
Allow the shared code to differentiate for which target it's being compiled.
*/

/// Macro expands its argument only if this target is 64-bit windows.
#[cfg(all(windows, target_pointer_width = "64"))]
macro_rules! current_target {
	($($tt:tt)*) => ($($tt)*);
}
#[cfg(not(all(windows, target_pointer_width = "64")))]
macro_rules! current_target {
    ($($tt:tt)*) => {};
}

#[allow(unused_macros)]
macro_rules! branch {
	(pe32 $pe32:tt pe64 { $($pe64:tt)* }) => { $($pe64)* };
}
