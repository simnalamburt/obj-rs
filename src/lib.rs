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

<img src="http://simnalamburt.github.io/obj-rs/screenshot.png" style="max-width:100%">

[obj]: https://en.wikipedia.org/wiki/Wavefront_.obj_file
[GitHub]: https://github.com/simnalamburt/obj-rs

*/

#![deny(missing_docs)]

#[cfg(feature = "glium-support")] #[macro_use] extern crate glium;
extern crate vec_map;
extern crate rustc_serialize;

#[macro_use] mod error;
pub mod raw;

use std::io::BufRead;
use std::collections::hash_map::{HashMap, Entry};
use raw::object::Polygon;
pub use error::{ObjResult, ObjError};

/// Load a wavefront OBJ file into Rust & OpenGL friendly format.
pub fn load_obj<V: FromRawVertex, T: BufRead>(input: T) -> ObjResult<Obj<V>> {
    let raw = try!(raw::parse_obj(input));
    Obj::new(raw)
}

/// 3D model object loaded from wavefront OBJ.
#[derive(RustcEncodable, RustcDecodable)]
pub struct Obj<V = Vertex> {
    /// Object's name.
    pub name: Option<String>,
    /// Vertex buffer.
    pub vertices: Vec<V>,
    /// Index buffer.
    pub indices: Vec<u16>,
}

impl<V: FromRawVertex> Obj<V> {
    /// Create `Obj` from `RawObj` object.
    pub fn new(raw: raw::RawObj) -> ObjResult<Self> {
        let (vertices, indices) = try!(FromRawVertex::process(raw.positions, raw.normals, raw.polygons));

        Ok(Obj {
            name: raw.name,
            vertices: vertices,
            indices: indices
        })
    }
}

/// Conversion from `RawObj`'s raw data.
pub trait FromRawVertex : Sized {
    /// Build vertex and index buffer from raw object data.
    fn process(vertices: Vec<(f32, f32, f32, f32)>, normals: Vec<(f32, f32, f32)>, polygons: Vec<Polygon>) -> ObjResult<(Vec<Self>, Vec<u16>)>;
}

/// Vertex data type of `Obj` which contains position and normal data of a vertex.
#[derive(Copy, PartialEq, Clone, Debug, RustcEncodable, RustcDecodable)]
pub struct Vertex {
    /// Position vector of a vertex.
    pub position: [f32; 3],
    /// Normal vertor of a vertex.
    pub normal: [f32; 3],
}

#[cfg(feature = "glium-support")]
implement_vertex!(Vertex, position, normal);

impl FromRawVertex for Vertex {
    fn process(positions: Vec<(f32, f32, f32, f32)>, normals: Vec<(f32, f32, f32)>, polygons: Vec<Polygon>) -> ObjResult<(Vec<Self>, Vec<u16>)> {
        let mut vb = Vec::with_capacity(polygons.len() * 3);
        let mut ib = Vec::with_capacity(polygons.len() * 3);
        {
            let mut cache = HashMap::new();
            let mut map = |pi: usize, ni: usize| {
                // Look up cache
                let index = match cache.entry((pi, ni)) {
                    // Cache miss -> make new, store it on cache
                    Entry::Vacant(entry) => {
                        let p = positions[pi];
                        let n = normals[ni];
                        let vertex = Vertex { position: [p.0, p.1, p.2], normal: [n.0, n.1, n.2] };

                        let index= vb.len() as u16;
                        vb.push(vertex);
                        entry.insert(index);
                        index
                    }
                    // Cache hit -> use it
                    Entry::Occupied(entry) => {
                        *entry.get()
                    }
                };
                ib.push(index)
            };

            for polygon in polygons {
                match polygon {
                    Polygon::P(_) | Polygon::PT(_) => error!(InsufficientData, "Tried to extract normal data which are not contained in the model"),
                    Polygon::PN(ref vec) if vec.len() == 3 => {
                        for &(pi, ni) in vec { map(pi, ni) }
                    }
                    Polygon::PTN(ref vec) if vec.len() == 3 => {
                        for &(pi, _, ni) in vec { map(pi, ni) }
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
#[derive(Copy, PartialEq, Clone, Debug, RustcEncodable, RustcDecodable)]
pub struct Position {
    /// Position vector of a vertex.
    pub position: [f32; 3]
}

#[cfg(feature = "glium-support")]
implement_vertex!(Position, position);

impl FromRawVertex for Position {
    fn process(vertices: Vec<(f32, f32, f32, f32)>, _: Vec<(f32, f32, f32)>, polygons: Vec<Polygon>) -> ObjResult<(Vec<Self>, Vec<u16>)> {
        let vb = vertices.into_iter().map(|v| Position { position: [v.0, v.1, v.2] }).collect();
        let mut ib = Vec::with_capacity(polygons.len() * 3);
        {
            let mut map = |pi: usize| { ib.push(pi as u16) };

            for polygon in polygons {
                match polygon {
                    Polygon::P(ref vec) if vec.len() == 3 => {
                        for &pi in vec { map(pi) }
                    }
                    Polygon::PT(ref vec) | Polygon::PN(ref vec) if vec.len() == 3 => {
                        for &(pi, _) in vec { map(pi) }
                    }
                    Polygon::PTN(ref vec) if vec.len() == 3 => {
                        for &(pi, _, _) in vec { map(pi) }
                    }
                    _ => error!(UntriangulatedModel, "Model should be triangulated first to be loaded properly")
                }
            }
        }
        Ok((vb, ib))
    }
}


#[cfg(feature = "glium-support")]
mod glium_support {
    use glium::{vertex, index, VertexBuffer, IndexBuffer};
    use glium::backend::Facade;
    use super::Obj;

    impl<V: vertex::Vertex> Obj<V> {
        /// Retrieve glium-compatible vertex buffer from Obj
        pub fn vertex_buffer<F: Facade>(&self, facade: &F) -> Result<VertexBuffer<V>, vertex::BufferCreationError> {
            VertexBuffer::new(facade, &self.vertices)
        }

        /// Retrieve glium-compatible index buffer from Obj
        pub fn index_buffer<F: Facade>(&self, facade: &F) -> Result<IndexBuffer<u16>, index::BufferCreationError> {
            IndexBuffer::new(facade, index::PrimitiveType::TrianglesList, &self.indices)
        }
    }
}
