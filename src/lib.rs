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
use std::u16;
pub use error::ObjResult;
use raw::{parse_obj, RawObj};
use raw::object::Polygon::*;

/// Load a wavefront `.obj` format into rust and OpenGL friendly format.
pub fn load_obj<T: BufRead, O: FromRawObj>(input: T) -> ObjResult<O> {
    let raw = try!(parse_obj(input));
    FromRawObj::from_raw_obj(raw)
}

/// Conversion from an `RawObj`
pub trait FromRawObj {
    /// Build a object with raw data.
    fn from_raw_obj(raw: RawObj) -> ObjResult<Self>;
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

impl FromRawObj for Obj {
    fn from_raw_obj(raw: RawObj) -> ObjResult<Self> {
        Ok(Obj {
            name: raw.name,
            vertices: raw.vertices.into_iter().map(|f32x4(x, y, z, _)| Vertex { position: [x, y, z] }).collect(),
            indices: {
                let mut buffer = Vec::with_capacity(raw.polygons.len() * 3);
                for polygon in raw.polygons.into_iter() {
                    match polygon {
                        P(ref vec) if vec.len() == 3 => for &idx in vec.iter() {
                            assert!(idx <= u16::MAX as u32);
                            buffer.push(idx as u16)
                        },
                        PT(ref vec) | PN(ref vec) if vec.len() == 3 => for &(idx, _) in vec.iter() {
                            assert!(idx <= u16::MAX as u32);
                            buffer.push(idx as u16)
                        },
                        PTN(ref vec) if vec.len() == 3 => for &(idx, _, _) in vec.iter() {
                            assert!(idx <= u16::MAX as u32);
                            buffer.push(idx as u16)
                        },
                        _ => error!(UntriangulatedModel, "Model should be triangulated first to be loaded properly")
                    }
                }
                buffer
            }
        })
    }
}
