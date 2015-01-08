extern crate obj;

use std::io::{BufferedReader, File};
use std::simd::f32x4;
use obj::obj::Polygon::PT;

#[test]
fn test_obj() {
    let path = Path::new("tests").join("fixtures").join("cube.obj");
    let mut input = BufferedReader::new(File::open(&path));

    let obj = obj::obj(&mut input);

    macro_rules! eq {
        ($($lhs:expr $rhs:expr)*) => ({
            $(
                assert_eq!($lhs, $rhs);
            )*
        })
    }

    macro_rules! vec_eq {
        ($($lhs:expr : $x:expr, $y:expr, $z:expr, $w:expr)*) => ({
            $(
                let f32x4(x, y, z, w) = $lhs;
                assert_eq!(x, stringify!($x).parse().unwrap());
                assert_eq!(y, stringify!($y).parse().unwrap());
                assert_eq!(z, stringify!($z).parse().unwrap());
                assert_eq!(w, stringify!($w).parse().unwrap());
            )*
        })
    }

    eq! {
        obj.name                                    "Cube".to_string()
        obj.material_libraries                      vec![ "cube.mtl" ]

        obj.vertices.len()                          8
        obj.tex_coords.len()                        14
        obj.normals.len()                           0
        obj.param_vertices.len()                    0

        obj.points.len()                            0
        obj.lines.len()                             0
        obj.polygons.len()                          6

        obj.groups.len()                            1
        obj.meshes.len()                            1
        obj.smoothing_groups.len()                  0
        obj.merging_groups.len()                    0
    };

    vec_eq! {
        obj.vertices[0] :                           1.000000, -1.000000, -1.000000, 1.0
        obj.vertices[1] :                           1.000000, -1.000000,  1.000000, 1.0
        obj.vertices[2] :                          -1.000000, -1.000000,  1.000000, 1.0
        obj.vertices[3] :                          -1.000000, -1.000000, -1.000000, 1.0
        obj.vertices[4] :                           1.000000,  1.000000, -0.999999, 1.0
        obj.vertices[5] :                           0.999999,  1.000000,  1.000001, 1.0
        obj.vertices[6] :                          -1.000000,  1.000000,  1.000000, 1.0
        obj.vertices[7] :                          -1.000000,  1.000000, -1.000000, 1.0

        obj.tex_coords[00] :                        1.004952,  0.498633,  0.000000, 0.0
        obj.tex_coords[01] :                        0.754996,  0.498236,  0.000000, 0.0
        obj.tex_coords[02] :                        0.755393,  0.248279,  0.000000, 0.0
        obj.tex_coords[03] :                        1.005349,  0.248677,  0.000000, 0.0
        obj.tex_coords[04] :                        0.255083,  0.497442,  0.000000, 0.0
        obj.tex_coords[05] :                        0.255480,  0.247485,  0.000000, 0.0
        obj.tex_coords[06] :                        0.505437,  0.247882,  0.000000, 0.0
        obj.tex_coords[07] :                        0.505039,  0.497839,  0.000000, 0.0
        obj.tex_coords[08] :                        0.754598,  0.748193,  0.000000, 0.0
        obj.tex_coords[09] :                        0.504642,  0.747795,  0.000000, 0.0
        obj.tex_coords[10] :                        0.505834, -0.002074,  0.000000, 0.0
        obj.tex_coords[11] :                        0.755790, -0.001677,  0.000000, 0.0
        obj.tex_coords[12] :                        0.005127,  0.497044,  0.000000, 0.0
        obj.tex_coords[13] :                        0.005524,  0.247088,  0.000000, 0.0
    };

    eq! {
        obj.polygons[0]                             PT( vec![ (1, 1), (2, 2), (3, 3), (4, 4)   ] )
        obj.polygons[1]                             PT( vec![ (5, 5), (8, 6), (7, 7), (6, 8)   ] )
        obj.polygons[2]                             PT( vec![ (1, 9), (5, 10), (6, 8), (2, 2)  ] )
        obj.polygons[3]                             PT( vec![ (2, 2), (6, 8), (7, 7), (3, 3)   ] )
        obj.polygons[4]                             PT( vec![ (3, 3), (7, 7), (8, 11), (4, 12) ] )
        obj.polygons[5]                             PT( vec![ (5, 5), (1, 13), (4, 14), (8, 6) ] )
    };
}
