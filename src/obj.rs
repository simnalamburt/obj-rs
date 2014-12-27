use std::simd::f32x4;
use lex::lex;
use error::{parse_error, ParseErrorKind};

macro_rules! f {
    ($args: ident) => {
        $args.iter()
            .map(|&input| match from_str::<f32>(input) {
                Some(number) => number,
                None => unimplemented!()
            })
            .collect::<Vec<f32>>()
            .as_slice()
    }
}

macro_rules! error {
    ($kind: ident) => {
        return Some(parse_error(ParseErrorKind::$kind))
    }
}


/// Parses a wavefront `.obj` file
pub fn obj<T: Buffer>(input: &mut T) {
    let mut obj = Obj::new();

    lex(input, |stmt, args| {
        match stmt {
            // Vertex data
            "v" => obj.vertices.push(match f!(args) {
                [x, y, z, w] => f32x4(x, y, z, w),
                [x, y, z] => f32x4(x, y, z, 1.0),
                _ => error!(WrongNumberOfArguments)
            }),
            "vt" => obj.tex_coords.push(match f!(args) {
                [u, v, w] => f32x4(u, v, w, 0.0),
                [u, v] => f32x4(u, v, 0.0, 0.0),
                [u] => f32x4(u, 0.0, 0.0, 0.0),
                _ => error!(WrongNumberOfArguments)
            }),
            "vn" => obj.normals.push(match f!(args) {
                [x, y, z] => f32x4(x, y, z, 0.0),
                _ => error!(WrongNumberOfArguments)
            }),
            "vp" => obj.param_vertices.push(match f!(args) {
                [u, v, w] => f32x4(u, v, w, 0.0),
                [u, v] => f32x4(u, v, 1.0, 0.0),
                [u] => f32x4(u, 0.0, 1.0, 0.0),
                _ => error!(WrongNumberOfArguments)
            }),

            // Free-form curve / surface attributes
            "cstype" => {
                let _rational: bool;
                let geometry = match args {
                    ["rat", ty] => {
                        _rational = true;
                        ty
                    }
                    [ty] => {
                        _rational = false;
                        ty
                    }
                    _ => unimplemented!()
                };

                match geometry {
                    "bmatrix" => unimplemented!(),
                    "bezier" => unimplemented!(),
                    "bspline" => unimplemented!(),
                    "cardinal" => unimplemented!(),
                    "taylor" => unimplemented!(),
                    _ => unimplemented!()
                }
            }
            "deg" => match f!(args) {
                [_deg_u, _deg_v]  => unimplemented!(),
                [_deg_u] => unimplemented!(),
                _ => unimplemented!(),
            },
            "bmat" => unimplemented!(),
            "step" => unimplemented!(),

            // Elements
            "p" => unimplemented!(),
            "l" => unimplemented!(),
            "f" => unimplemented!(),
            "curv" => unimplemented!(),
            "curv2" => unimplemented!(),
            "surf" => unimplemented!(),

            // Free-form curve / surface body statements
            "parm" => unimplemented!(),
            "trim" => unimplemented!(),
            "hole" => unimplemented!(),
            "scrv" => unimplemented!(),
            "sp" => unimplemented!(),
            "end" => unimplemented!(),

            // Connectivity between free-form surfaces
            "con" => unimplemented!(),

            // Grouping
            "g" => match args {
                [name] => obj.groups.push(Group::new(name)),
                _ => unimplemented!()
            },
            "s" => unimplemented!(),
            "mg" => unimplemented!(),
            "o" => {
                if !obj.name.is_empty() { unimplemented!() }

                obj.name = args.connect(" ");
            }

            // Display / render attributes
            "bevel" => unimplemented!(),
            "c_interp" => unimplemented!(),
            "d_interp" => unimplemented!(),
            "lod" => unimplemented!(),
            "usemtl" => match args {
                [material] => obj.groups.as_mut_slice().last_mut().unwrap().meshes.push(Mesh::new(material)),
                _ => error!(WrongNumberOfArguments)
            },
            "mtllib" => {
                let paths: Vec<String> = args.iter().map(|path| path.to_string()).collect();
                obj.material_libraries.push_all(paths.as_slice());
            }
            "shadow_obj" => unimplemented!(),
            "trace_obj" => unimplemented!(),
            "ctech" => unimplemented!(),
            "stech" => unimplemented!(),

            // Unexpected statement
            _ => error!(UnexpectedStatement)
        }

        None
    });
}

/// Parsed obj file
pub struct Obj {
    pub name: String,

    pub vertices: Vec<f32x4>,
    pub tex_coords: Vec<f32x4>,
    pub normals: Vec<f32x4>,
    pub param_vertices: Vec<f32x4>,

    pub material_libraries: Vec<String>,

    pub groups: Vec<Group>
}

impl Obj {
    fn new() -> Self {
        Obj {
            name: String::new(),
            vertices: Vec::new(),
            tex_coords: Vec::new(),
            normals: Vec::new(),
            param_vertices: Vec::new(),
            material_libraries: Vec::new(),
            groups: vec![ Group::new("default") ]
        }
    }
}

pub struct Group {
    pub name: String,
    pub meshes: Vec<Mesh>
}

impl Group {
    fn new(name: &str) -> Self {
        Group { name: name.to_string(), meshes: vec![ Mesh::new("") ] }
    }
}

pub struct Mesh {
    pub material: String
}

impl Mesh {
    fn new(material: &str) -> Self {
        Mesh { material: material.to_string() }
    }
}
