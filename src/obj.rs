use std::simd::f32x4;
use lex::lex;

fn f<'a>(args: &'a [&str]) -> Vec<f32> {
    args.iter().map(|&input| from_str::<f32>(input).unwrap()).collect()
}

/// Parses a wavefront `.obj` file
pub fn obj<T: Buffer>(input: &mut T) {

    let mut vertices = Vec::new();

    lex(input, |stmt, args| {
        match stmt {
            // Vertex data
            "v" => vertices.push(match f(args).as_slice() {
                [x, y, z, w] => { f32x4(x, y, z, w) }
                [x, y, z] => { f32x4(x, y, z, 1.0) }
                _ => panic!()
            }),
            "vt" => {}
            "vn" => {}
            "vp" => {}

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
            _ => panic!("Unexpected statement: {} {}", stmt, args.connect(" "))
        }
    });
}
