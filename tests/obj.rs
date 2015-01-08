extern crate obj;

use std::io::{BufferedReader, File};
use std::simd::f32x4;
use obj::obj::Polygon;

fn fixture(filename: &str) -> obj::obj::Obj {
    let path = Path::new("tests").join("fixtures").join(filename);
    let mut input = BufferedReader::new(File::open(&path));

    obj::obj(&mut input)
}

macro_rules! eq {
    ($($lhs:expr { $($x:expr, $y:expr, $z:expr, $w:expr;)* })*) => ({
        $({
            let mut index = 0u;
            $(
                let f32x4(x, y, z, w) = $lhs[index];
                assert_eq!(x, stringify!($x).parse().unwrap());
                assert_eq!(y, stringify!($y).parse().unwrap());
                assert_eq!(z, stringify!($z).parse().unwrap());
                assert_eq!(w, stringify!($w).parse().unwrap());
                index += 1;
            )*
            assert_eq!($lhs.len(), index);
        })*
    });

    ($($lhs:expr { $($kind:ident $elem:expr)* })*) => ({
        $({
            let mut index = 0u;
            $(
                assert_eq!($lhs[index], Polygon::$kind($elem));
                index += 1;
            )*
            assert_eq!($lhs.len(), index);
        })*
    });

    ($($lhs:expr $rhs:expr)*) => ({
        $(assert_eq!($lhs, $rhs);)*
    });
}

#[test]
fn cube() {
    let obj = fixture("cube.obj");

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

    eq! {
        obj.vertices {
            1.000000, -1.000000, -1.000000, 1.0;
            1.000000, -1.000000,  1.000000, 1.0;
           -1.000000, -1.000000,  1.000000, 1.0;
           -1.000000, -1.000000, -1.000000, 1.0;
            1.000000,  1.000000, -0.999999, 1.0;
            0.999999,  1.000000,  1.000001, 1.0;
           -1.000000,  1.000000,  1.000000, 1.0;
           -1.000000,  1.000000, -1.000000, 1.0;
        }

        obj.tex_coords {
            1.004952,  0.498633,  0.000000, 0.0;
            0.754996,  0.498236,  0.000000, 0.0;
            0.755393,  0.248279,  0.000000, 0.0;
            1.005349,  0.248677,  0.000000, 0.0;
            0.255083,  0.497442,  0.000000, 0.0;
            0.255480,  0.247485,  0.000000, 0.0;
            0.505437,  0.247882,  0.000000, 0.0;
            0.505039,  0.497839,  0.000000, 0.0;
            0.754598,  0.748193,  0.000000, 0.0;
            0.504642,  0.747795,  0.000000, 0.0;
            0.505834, -0.002074,  0.000000, 0.0;
            0.755790, -0.001677,  0.000000, 0.0;
            0.005127,  0.497044,  0.000000, 0.0;
            0.005524,  0.247088,  0.000000, 0.0;
        }
    };

    eq! {
        obj.polygons {
            PT vec![ (1, 1), (2, 2), (3, 3), (4, 4)   ]
            PT vec![ (5, 5), (8, 6), (7, 7), (6, 8)   ]
            PT vec![ (1, 9), (5, 10), (6, 8), (2, 2)  ]
            PT vec![ (2, 2), (6, 8), (7, 7), (3, 3)   ]
            PT vec![ (3, 3), (7, 7), (8, 11), (4, 12) ]
            PT vec![ (5, 5), (1, 13), (4, 14), (8, 6) ]
        }
    };
}
