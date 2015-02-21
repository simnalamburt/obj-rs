//! [Wavefront obj][obj] parser for Rust. It handles both `.obj` and `.mtl` formats.
//!
//! [obj]: //en.wikipedia.org/wiki/Wavefront_.obj_file

#![feature(core, io, collections, str_words)]
#![cfg_attr(test, feature(test))]
#![deny(warnings, missing_docs)]

#[macro_use] pub mod error;
pub mod raw;

use std::io::BufRead;
use error::ObjResult;
use raw::parse_obj;

/// Load a wavefront `.obj` format into rust and OpenGL friendly format
pub fn load_obj<T: BufRead>(input: T) -> ObjResult<Obj> {
    let raw = try!(parse_obj(input));

    Ok(Obj {
        name: raw.name
    })
}

/// 3D Model object
pub struct Obj {
    /// Name of the model
    pub name: String
}
