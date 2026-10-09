#![allow(non_snake_case)]

use std::{collections::HashMap, sync::Arc};
use std::ptr;

mod wasm32;
mod pefile;
mod hashes;
mod hex_dump;
mod section_entropy;

use crate::wasm32::*;
