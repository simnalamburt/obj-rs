//! [Wavefront obj][obj] parser for Rust. It handles both `.obj` and `.mtl` formats.
//!
//! [obj]: //en.wikipedia.org/wiki/Wavefront_.obj_file

#![feature(core, io, collections, str_words)]
#![cfg_attr(test, feature(test))]
#![deny(warnings, missing_docs)]

#[macro_use] pub mod error;
pub mod raw;

use std::io::BufRead;
use std::simd::f32x4;
use error::ObjResult;
use raw::parse_obj;

/// Load a wavefront `.obj` format into rust and OpenGL friendly format
pub fn load_obj<T: BufRead>(input: T) -> ObjResult<Obj> {
    let raw = try!(parse_obj(input));

    Ok(Obj {
        name: raw.name,
        vertices: raw.vertices.into_iter().map(|f32x4(x, y, z, _)| (x, y, z)).collect(),
        indices: {
            let mut buffer = Vec::new();
            for polygon in raw.polygons.into_iter() {
                use raw::object::Polygon::*;
                let indices = match polygon {
                    P(ref i) if i.len() == 3 => i,
                    _ => error!(UntriangulatedModel, "Model should be triangulated first to be loaded properly")
                };
                for &index in indices.iter() {
                    assert!(index <= std::u16::MAX as u32);
                    buffer.push(index as u16)
                }
            }
            buffer
        }
    })
}

/// 3D Model object
pub struct Obj {
    /// Name of the model
    pub name: Option<String>,
    /// Vertex buffer of the model
    pub vertices: Vec<(f32, f32, f32)>,
    /// Index buffer of the model
    pub indices: Vec<u16>,
}
