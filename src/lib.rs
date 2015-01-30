//! [Wavefront obj][obj] parser for Rust. It handles both `.obj` and `.mtl` formats.
//!
//! [obj]: //en.wikipedia.org/wiki/Wavefront_.obj_file

#![deny(warnings, missing_docs)]

#![feature(core, collections, io)]
#![cfg_attr(test, feature(test))]

#[macro_use] mod error;
mod lex;
pub mod obj;
pub mod mtl;

pub use obj::{load_obj, Obj};
pub use mtl::{load_mtl, Mtl};
