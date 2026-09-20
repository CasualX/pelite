#![allow(non_snake_case)]

use std::{collections::HashMap, sync::Arc};
use std::ptr;

mod wasm32;
mod pefile;

use crate::wasm32::*;
