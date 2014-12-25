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

macro_rules! wrong_number {
    () => {
        Some(parse_error(ParseErrorKind::WrongNumberOfArguments))
    }
}


/// Parses a wavefront `.obj` file
pub fn obj<T: Buffer>(input: &mut T) {
    let mut name = String::new();

    let mut vertices = Vec::new();
    let mut tex_coords = Vec::new();
    let mut normals = Vec::new();
    let mut param_vertices = Vec::new();

    let mut material_libraries = Vec::new();

    lex(input, |stmt, args| {
        match stmt {
            // Vertex data
            "v" => vertices.push(match f!(args) {
                [x, y, z, w] => f32x4(x, y, z, w),
                [x, y, z] => f32x4(x, y, z, 1.0),
                _ => return wrong_number!()
            }),
            "vt" => tex_coords.push(match f!(args) {
                [u, v, w] => f32x4(u, v, w, 0.0),
                [u, v] => f32x4(u, v, 0.0, 0.0),
                [u] => f32x4(u, 0.0, 0.0, 0.0),
                _ => return wrong_number!()
            }),
            "vn" => normals.push(match f!(args) {
                [x, y, z] => f32x4(x, y, z, 0.0),
                _ => return wrong_number!()
            }),
            "vp" => param_vertices.push(match f!(args) {
                [u, v, w] => f32x4(u, v, w, 0.0),
                [u, v] => f32x4(u, v, 1.0, 0.0),
                [u] => f32x4(u, 0.0, 1.0, 0.0),
                _ => return wrong_number!()
            }),

            // Free-form curve / surface attributes
            "cstype" => {
                let rational: bool;
                let geometry = match args {
                    ["rat", ty] => {
                        rational = true;
                        ty
                    }
                    [ty] => {
                        rational = false;
                        ty
                    }
                    _ => unimplemented!()
                };

                let _ = rational; // TODO : remove this line
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
                [_, _]  => unimplemented!(),
                [_] => unimplemented!(),
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
            "g" => unimplemented!(),
            "s" => unimplemented!(),
            "mg" => unimplemented!(),
            "o" => {
                name = args.connect(" ");
            }

            // Display / render attributes
            "bevel" => unimplemented!(),
            "c_interp" => unimplemented!(),
            "d_interp" => unimplemented!(),
            "lod" => unimplemented!(),
            "usemtl" => unimplemented!(),
            "mtllib" => {
                let paths: Vec<String> = args.iter().map(|path| path.to_string()).collect();
                material_libraries.push_all(paths.as_slice());
            }
            "shadow_obj" => unimplemented!(),
            "trace_obj" => unimplemented!(),
            "ctech" => unimplemented!(),
            "stech" => unimplemented!(),

            // Unexpected statement
            _ => return Some(parse_error(ParseErrorKind::UnexpectedStatement))
        }

        None
    });
}

/// Parsed obj file
pub struct Obj;

impl Copy for Obj{}
