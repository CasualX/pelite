
/// Ensures alignment of at least 16 bytes.
#[repr(align(16))]
pub struct Align16<T>(pub T);

/// Ensures alignment of at least 512 bytes.
#[repr(align(512))]
pub struct Align512<T>(pub T);

/// Ensures alignment of at least 4K bytes.
#[repr(align(4096))]
pub struct Align4K<T>(pub T);
