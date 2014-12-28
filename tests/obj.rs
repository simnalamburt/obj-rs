#![feature(macro_rules)]

extern crate obj;

use std::io::{BufferedReader, File};

#[test]
fn test_obj() {
    let path = Path::new("tests").join("fixtures").join("cube.obj");
    let mut input = BufferedReader::new(File::open(&path));

    let obj = obj::obj(&mut input);

    macro_rules! eq {
        { $($lhs:expr $rhs:expr)* } => ({
            $(
                assert_eq!($lhs, $rhs);
            )*
        })
    }

    eq! {
        obj.name                                    "Cube".to_string()

        obj.vertices.len()                          8
        obj.tex_coords.len()                        14
        obj.normals.len()                           0
        obj.param_vertices.len()                    0

        obj.groups.len()                            1
        obj.groups[0].meshes.len()                  1
        obj.groups[0].meshes[0].polygons.len()      6
    };
}
