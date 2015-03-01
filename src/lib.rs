/*!

[Wavefront OBJ][obj] parser for Rust. It handles both `.obj` and `.mtl` formats. [GitHub][]

```rust
use std::fs::File;
use std::io::BufReader;
use obj::*;

let input = BufReader::new(File::open("tests/fixtures/normal-cone.obj").unwrap());
let dome: Obj = load_obj(input).unwrap();

// Do whatever you want
dome.vertices;
dome.indices;
```

<img src="http://simnalamburt.github.io/obj-rs/sample.png" style="max-width:100%">

[obj]: https://en.wikipedia.org/wiki/Wavefront_.obj_file
[GitHub]: https://github.com/simnalamburt/obj-rs

*/

#![feature(core, plugin, io, collections, str_words, std_misc)]
#![cfg_attr(feature = "glium-support", plugin(glium_macros))]
#![cfg_attr(test, feature(test))]
#![deny(warnings, missing_docs)]

#[cfg(feature = "glium-support")]
extern crate glium;

#[macro_use] mod error;
pub mod raw;

use std::io::BufRead;
use std::simd::f32x4;
use std::num::cast;
use std::collections::HashMap;
use std::collections::hash_map::Entry::*;

use raw::{parse_obj, RawObj};
use raw::object::Polygon;
use raw::object::Polygon::*;

pub use error::ObjResult;

/// Load a wavefront OBJ file into Rust & OpenGL friendly format.
pub fn load_obj<V: FromRawVertex, T: BufRead>(input: T) -> ObjResult<Obj<V>> {
    let raw = try!(parse_obj(input));
    Obj::new(raw)
}

/// 3D model object loaded from wavefront OBJ.
pub struct Obj<V = Vertex> {
    /// Object's name.
    pub name: Option<String>,
    /// Vertex buffer.
    pub vertices: Vec<V>,
    /// Index buffer.
    pub indices: Vec<u16>,
}

impl<V: FromRawVertex> Obj<V> {
    fn new(raw: RawObj) -> ObjResult<Self> {
        let (vertices, indices) = try!(FromRawVertex::process(raw.positions, raw.normals, raw.polygons));

        Ok(Obj {
            name: raw.name,
            vertices: vertices,
            indices: indices
        })
    }
}

/// Conversion from `RawObj`'s raw data.
pub trait FromRawVertex {
    /// Build vertex and index buffer from raw object data.
    fn process(vertices: Vec<f32x4>, normals: Vec<f32x4>, polygons: Vec<Polygon>) -> ObjResult<(Vec<Self>, Vec<u16>)>;
}

/// Vertex data type of `Obj` which contains position and normal data of a vertex.
#[derive(Copy, PartialEq, Clone, Debug)]
#[cfg_attr(feature = "glium-support", vertex_format)]
pub struct Vertex {
    /// Position vector of a vertex.
    pub position: [f32; 3],
    /// Normal vertor of a vertex.
    pub normal: [f32; 3],
}

impl FromRawVertex for Vertex {
    fn process(positions: Vec<f32x4>, normals: Vec<f32x4>, polygons: Vec<Polygon>) -> ObjResult<(Vec<Self>, Vec<u16>)> {
        let mut vb = Vec::with_capacity(polygons.len() * 3);
        let mut ib = Vec::with_capacity(polygons.len() * 3);
        {
            let mut cache = HashMap::new();
            let mut map = |pi: usize, ni: usize| {
                // Look up cache
                let index = match cache.entry((pi, ni)) {
                    // Cache fail -> make new, store it on cache
                    Vacant(entry) => {
                        let p = positions[pi];
                        let n = normals[ni];
                        let vertex = Vertex { position: [p.0, p.1, p.2], normal: [n.0, n.1, n.2] };

                        let index = cast(vb.len()).unwrap();
                        vb.push(vertex);
                        entry.insert(index);
                        index
                    }
                    // Cache hit -> use it
                    Occupied(entry) => {
                        *entry.get()
                    }
                };
                ib.push(index)
            };

            for polygon in polygons.into_iter() {
                match polygon {
                    P(_) | PT(_) => error!(InsufficientData, "Tried to extract normal data which are not contained in the model"),
                    PN(ref vec) if vec.len() == 3 => {
                        for &(pi, ni) in vec.iter() { map(pi, ni) }
                    }
                    PTN(ref vec) if vec.len() == 3 => {
                        for &(pi, _, ni) in vec.iter() { map(pi, ni) }
                    }
                    _ => error!(UntriangulatedModel, "Model should be triangulated first to be loaded properly")
                }
            }
        }
        vb.shrink_to_fit();
        Ok((vb, ib))
    }
}

/// Vertex data type of `Obj` which contains only position data of a vertex.
#[derive(Copy, PartialEq, Clone, Debug)]
#[cfg_attr(feature = "glium-support", vertex_format)]
pub struct Position {
    /// Position vector of a vertex.
    pub position: [f32; 3]
}

impl FromRawVertex for Position {
    fn process(vertices: Vec<f32x4>, _: Vec<f32x4>, polygons: Vec<Polygon>) -> ObjResult<(Vec<Self>, Vec<u16>)> {
        let vb = vertices.into_iter().map(|v| Position { position: [v.0, v.1, v.2] }).collect();
        let mut ib = Vec::with_capacity(polygons.len() * 3);
        {
            let mut map = |pi| { ib.push(cast(pi).unwrap()) };
            for polygon in polygons.into_iter() {
                match polygon {
                    P(ref vec) if vec.len() == 3 => {
                        for &pi in vec.iter() { map(pi) }
                    }
                    PT(ref vec) | PN(ref vec) if vec.len() == 3 => {
                        for &(pi, _) in vec.iter() { map(pi) }
                    }
                    PTN(ref vec) if vec.len() == 3 => {
                        for &(pi, _, _) in vec.iter() { map(pi) }
                    }
                    _ => error!(UntriangulatedModel, "Model should be triangulated first to be loaded properly")
                }
            }
        }
        Ok((vb, ib))
    }
}
