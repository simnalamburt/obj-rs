#![experimental]

fn lex<T: Buffer>(input: &mut T, callback: |&str, &[&str]|) -> Option<std::io::IoError> {
    for maybe_line in input.lines() {
        match maybe_line {
            Ok(line) => {
                let line = line.as_slice();
                let line = line.split('#').next().unwrap();

                let mut words = line.words();
                match words.next() {
                    Some(stmt) => {
                        let args: Vec<&str> = words.collect();
                        callback(stmt, args.as_slice())
                    }
                    None => {}
                }
            }
            Err(e) => { return Some(e); }
        }
    }
    None
}

#[test]
fn test_lex() {
    let input = r#"
   statement0      arg0  arg1	arg2#argX   argX
statement1 arg0    arg1
# Comment
statement2 Hello, world!
"#;

    lex(&mut input.as_bytes(), |stmt, args| {
        match stmt {
            "statement0" => assert_eq!(args, ["arg0", "arg1", "arg2"]),
            "statement1" => assert_eq!(args, ["arg0", "arg1"]),
            "statement2" => assert_eq!(args, ["Hello,", "world!"]),
            _ => panic!()
        }
    });
}


/// Parses a wavefront `.obj` file
pub fn obj<T: Buffer>(input: &mut T) {
    lex(input, |stmt, args| {
        match stmt {
            // Vertex data
            "v" => {}
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
            _ => {}
        }
    });
}

pub fn mtl<T: Buffer>(input: &mut T) {
    lex(input, |stmt, args| {
        match stmt {
            // Material name statement
            "newmtl" => {}

            // Material color and illumination statements
            "Ka" => {}
            "Kd" => {}
            "Ks" => {}
            "Ke" => {}
            "Km" => {}
            "Ns" => {}
            "Ni" => {}
            "Tr" => {}
            "Tf" => {}
            "illum" => {}
            "d" => {}

            // Texture map statements
            "map_Ka" => {}
            "map_Kd" => {}
            "map_Ks" => {}
            "map_d" => {}
            "map_aat" => {}
            "map_refl" => {}
            "map_bump" | "map_Bump" | "bump" => {}
            "disp" => {}

            // Reflection map statement
            "refl" => {}

            // Unexpected statement
            _ => {}
        }
    });
}
