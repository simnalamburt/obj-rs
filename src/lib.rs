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
use raw::object::Polygon;

/// Load a wavefront `.obj` format into rust and OpenGL friendly format.
pub fn load_obj<V: FromRawVertex, T: BufRead>(input: T) -> ObjResult<Obj<V>> {
    let raw = try!(parse_obj(input));
    Obj::new(raw)
}

/// 3D model object.
pub struct Obj<V = Vertex> {
    /// Name of the model.
    pub name: Option<String>,
    /// Vertex buffer of the model.
    pub vertices: Vec<V>,
    /// Index buffer of the model.
    pub indices: Vec<u16>,
}

/// Vertex data type of `Obj`.
#[derive(Copy, PartialEq, Clone, Debug)]
#[cfg_attr(feature = "glium-support", vertex_format)]
pub struct Vertex {
    /// Position vector of a vertex.
    pub position: [f32; 3]
}

/// Conversion from `RawObj`'s raw data.
pub trait FromRawVertex {
    /// Build vertex and index buffer from raw object data.
    fn process(vertices: Vec<f32x4>, polygons: Vec<Polygon>) -> ObjResult<(Vec<Self>, Vec<u16>)>;
}

impl FromRawVertex for Vertex {
    fn process(vertices: Vec<f32x4>, polygons: Vec<Polygon>) -> ObjResult<(Vec<Self>, Vec<u16>)> {
        Ok(({
            vertices.into_iter()
                .map(|f32x4(x, y, z, _)| Vertex { position: [x, y, z] })
                .collect()
        }, {
            let mut buffer = Vec::with_capacity(polygons.len() * 3);
            for polygon in polygons.into_iter() {
                match polygon {
                    Polygon::P(ref vec) if vec.len() == 3 => for &idx in vec.iter() {
                        assert!(idx <= u16::MAX as u32);
                        buffer.push(idx as u16)
                    },
                    Polygon::PT(ref vec) | Polygon::PN(ref vec) if vec.len() == 3 => for &(idx, _) in vec.iter() {
                        assert!(idx <= u16::MAX as u32);
                        buffer.push(idx as u16)
                    },
                    Polygon::PTN(ref vec) if vec.len() == 3 => for &(idx, _, _) in vec.iter() {
                        assert!(idx <= u16::MAX as u32);
                        buffer.push(idx as u16)
                    },
                    _ => error!(UntriangulatedModel, "Model should be triangulated first to be loaded properly")
                }
            }
            buffer
        }))
    }
}

impl<V: FromRawVertex> Obj<V> {
    fn new(raw: RawObj) -> ObjResult<Self> {
        let (vertices, indices) = try!(FromRawVertex::process(raw.vertices, raw.polygons));

        Ok(Obj {
            name: raw.name,
            vertices: vertices,
            indices: indices
        })
    }
}
