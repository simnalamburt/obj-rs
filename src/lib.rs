//! [Wavefront obj][obj] parser for Rust. It handles both `.obj` and `.mtl` formats.
//!
//! [obj]: //en.wikipedia.org/wiki/Wavefront_.obj_file

#![feature(core, plugin, io, collections, str_words)]
#![cfg_attr(feature = "glium-support", plugin(glium_macros))]
#![cfg_attr(test, feature(test))]
#![deny(warnings, missing_docs)]

#[cfg(feature = "glium-support")]
extern crate glium;

#[macro_use] mod error;
pub mod raw;

use std::io::BufRead;
use std::simd::f32x4;
pub use error::ObjResult;
use raw::parse_obj;

/// Load a wavefront `.obj` format into rust and OpenGL friendly format
pub fn load_obj<T: BufRead>(input: T) -> ObjResult<Obj> {
    let raw = try!(parse_obj(input));

    Ok(Obj {
        name: raw.name,
        vertices: raw.vertices.into_iter().map(|f32x4(x, y, z, _)| Vertex { position: [x, y, z] }).collect(),
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

/// 3D model object
pub struct Obj {
    /// Name of the model
    pub name: Option<String>,
    /// Vertex buffer of the model
    pub vertices: Vec<Vertex>,
    /// Index buffer of the model
    pub indices: Vec<u16>,
}

/// Vertex data type of `Obj`
#[derive(Copy, PartialEq, Clone, Debug)]
#[cfg_attr(feature = "glium-support", vertex_format)]
pub struct Vertex {
    /// Position vector of a vertex
    pub position: [f32; 3]
}
