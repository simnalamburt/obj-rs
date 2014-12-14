#![experimental]

fn lex<T: Buffer>(input: &mut T, callback: |&str, &[&str]|) -> Option<std::io::IoError> {
    for maybe_line in input.lines() {
        match maybe_line {
            Ok(line) => {
                let line = line.as_slice();
                let line = line.split('#').next().unwrap();

                let mut words = line.words();
                match words.next() {
                    Some(header) => {
                        let args: Vec<&str> = words.collect();
                        callback(header, args.as_slice())
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
   header0      arg0  arg1	arg2#argX   argX
header1 arg0    arg1
# Comment
header2 Hello, world!
"#;

    lex(&mut input.as_bytes(), |header, args| {
        match header {
            "header0" => assert_eq!(args, ["arg0", "arg1", "arg2"]),
            "header1" => assert_eq!(args, ["arg0", "arg1"]),
            "header2" => assert_eq!(args, ["Hello,", "world!"]),
            _ => panic!()
        }
    });
}


/// Parses a wavefront `.obj` file
pub fn load<T: Buffer>(input: &mut T) {
    lex(input, |header, args| {
        match header {
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

            // Unexpected header
            _ => {}
        }
    });
}
