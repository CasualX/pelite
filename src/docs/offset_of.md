Returns the byte offset of `symbol` from the start of the image.

`symbol` must refer to data in `self.image()`, rather than a copy of that data.

# Panics

Panics if `symbol` does not refer to data within the image. Rust's lifetime system cannot express this relationship, so the method checks it at runtime.
