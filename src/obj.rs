use std::simd::f32x4;
use lex::lex;
use error::{parse_error, ParseErrorKind};

macro_rules! f {
    ($args: ident) => {
        $args.iter()
            .map(|&input| from_str::<f32>(input).unwrap())
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
    let mut vertices = Vec::new();
    let mut tex_coords = Vec::new();
    let mut normals = Vec::new();
    let mut param_vertices = Vec::new();

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
            "cstype" => {}
            "deg" => {}
            "bmat" => {}
            "step" => {}

            // Elements
            "p" => {}
            "l" => {}
            "f" => {}
            "curv" => {}
            "curv2" => {}
            "surf" => {}

            // Free-form curve / surface body statements
            "parm" => {}
            "trim" => {}
            "hole" => {}
            "scrv" => {}
            "sp" => {}
            "end" => {}

            // Connectivity between free-form surfaces
            "con" => {}

            // Grouping
            "g" => {}
            "s" => {}
            "mg" => {}
            "o" => {}

            // Display / render attributes
            "bevel" => {}
            "c_interp" => {}
            "d_interp" => {}
            "lod" => {}
            "usemtl" => {}
            "mtllib" => {}
            "shadow_obj" => {}
            "trace_obj" => {}
            "ctech" => {}
            "stech" => {}

            // Unexpected statement
            _ => return Some(parse_error(ParseErrorKind::UnexpectedStatement))
        }

        None
    });
}

/// Parsed obj file
pub struct Obj;

impl Copy for Obj{}
