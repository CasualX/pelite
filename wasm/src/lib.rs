#![allow(non_snake_case)]

use std::{collections::HashMap, sync::Arc};
use std::{ops, ptr};

mod wasm32;
mod pefile;
mod hashes;
mod hex_dump;
mod section_entropy;
mod scanner;
mod resources;
mod disasm;
mod addr;

use self::wasm32::*;
use self::pefile::PeFile;
