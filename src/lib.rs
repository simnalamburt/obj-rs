//! [Wavefront obj][obj] parser for Rust. It handles both `.obj` and `.mtl` formats.
//!
//! [obj]: //en.wikipedia.org/wiki/Wavefront_.obj_file

#![feature(core, io, collections, str_words)]
#![cfg_attr(test, feature(test))]
#![deny(warnings, missing_docs)]

#[macro_use] pub mod error;
mod lex;
pub mod obj;
pub mod mtl;

pub use obj::{load_obj, Obj};
pub use mtl::{load_mtl, Mtl};
