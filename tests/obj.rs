#![feature(core, path, fs, io)]

extern crate obj;

use std::simd::f32x4;

fn fixture(filename: &str) -> obj::Obj {
    use std::path::Path;
    use std::fs::File;
    use std::io::BufReader;

    let path = Path::new("tests").join("fixtures").join(filename);
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => panic!("Failed to open \"{}\". \x1b[31m{}\x1b[0m", path.to_string_lossy(), e)
    };
    let input = BufReader::new(file);

    obj::load_obj(input)
}

macro_rules! test {
    ($($lhs:expr { $($x:expr, $y:expr, $z:expr, $w:expr;)* })*) => ({
        $({
            let mut index = 0us;
            $(
                let f32x4(x, y, z, w) = $lhs[index];
                eq!(x, stringify!($x).parse().unwrap(), stringify!($lhs[index].x));
                eq!(y, stringify!($y).parse().unwrap(), stringify!($lhs[index].x));
                eq!(z, stringify!($z).parse().unwrap(), stringify!($lhs[index].x));
                eq!(w, stringify!($w).parse().unwrap(), stringify!($lhs[index].x));
                index += 1;
            )*
            eq!($lhs.len(), index);
        })*
    });

    ($($lhs:expr { $($kind:ident $elem:expr)* })*) => ({
        $({
            let mut index = 0us;
            $(
                eq!($lhs[index], obj::obj::Polygon::$kind($elem));
                index += 1;
            )*
            eq!($lhs.len(), index);
        })*
    });

    ($($lhs:expr, $rhs:expr)*) => ({
        $(eq!($lhs, $rhs);)*
    });
}

macro_rules! eq {
    ($lhs:expr, $rhs:expr) => (eq!($lhs, $rhs, stringify!($lhs)));

    ($lhs:expr, $rhs:expr, $exp:expr) => ({
        let left = &($lhs);
        let right = &($rhs);

        if !((*left == *right) && (*right == *left)) {
            let _ = writeln!(&mut std::old_io::stdio::stderr(), "\x1b[33m{}\x1b[0m should be \x1b[33m{:?}\x1b[0m, \
                     but it was \x1b[33m{:?}\x1b[0m", $exp, *right, *left);
            panic!($exp);
        }
    });
}

#[test]
fn cube() {
    let obj = fixture("cube.obj");

    test! {
        obj.name,                       "Cube".to_string()
        obj.material_libraries,         vec![ "cube.mtl" ]

        obj.vertices.len(),             8
        obj.tex_coords.len(),           14
        obj.normals.len(),              0
        obj.param_vertices.len(),       0

        obj.points.len(),               0
        obj.lines.len(),                0
        obj.polygons.len(),             6

        obj.groups.len(),               1
        obj.meshes.len(),               1
        obj.smoothing_groups.len(),     0
        obj.merging_groups.len(),       0
    };

    test! {
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

    test! {
        obj.polygons {
            PT  vec![ (1, 1), (2, 2), (3, 3), (4, 4)   ]
            PT  vec![ (5, 5), (8, 6), (7, 7), (6, 8)   ]
            PT  vec![ (1, 9), (5, 10), (6, 8), (2, 2)  ]
            PT  vec![ (2, 2), (6, 8), (7, 7), (3, 3)   ]
            PT  vec![ (3, 3), (7, 7), (8, 11), (4, 12) ]
            PT  vec![ (5, 5), (1, 13), (4, 14), (8, 6) ]
        }
    };

    test! {
        obj.groups.get("default").unwrap().points.len(),        0
        obj.groups.get("default").unwrap().lines.len(),         0
        obj.groups.get("default").unwrap().polygons.len(),      1
        obj.groups.get("default").unwrap().polygons[0].start,   0
        obj.groups.get("default").unwrap().polygons[0].end,     6

        obj.meshes.get("Material").unwrap().points.len(),       0
        obj.meshes.get("Material").unwrap().lines.len(),        0
        obj.meshes.get("Material").unwrap().polygons.len(),     1
        obj.meshes.get("Material").unwrap().polygons[0].start,  0
        obj.meshes.get("Material").unwrap().polygons[0].end,    6
    };
}

#[test]
fn dome() {
    let obj = fixture("dome.obj");

    test! {
        obj.name,                       "Dome".to_string()
        obj.material_libraries,         vec![ "dome.mtl" ]

        obj.vertices.len(),             33
        obj.tex_coords.len(),           0
        obj.normals.len(),              0
        obj.param_vertices.len(),       0

        obj.points.len(),               0
        obj.lines.len(),                0
        obj.polygons.len(),             62

        obj.groups.len(),               1
        obj.meshes.len(),               1
        obj.smoothing_groups.len(),     2
        obj.merging_groups.len(),       0
    };

    test! {
        obj.vertices {
           -0.382683,  0.923880,  0.000000, 1.0;
           -0.707107,  0.707107,  0.000000, 1.0;
           -0.923880,  0.382683,  0.000000, 1.0;
           -1.000000, -0.000000,  0.000000, 1.0;
           -0.270598,  0.923880, -0.270598, 1.0;
           -0.500000,  0.707107, -0.500000, 1.0;
           -0.653282,  0.382683, -0.653281, 1.0;
           -0.707107, -0.000000, -0.707107, 1.0;
           -0.000000,  0.923880, -0.382683, 1.0;
           -0.000000,  0.707107, -0.707107, 1.0;
           -0.000000,  0.382683, -0.923879, 1.0;
           -0.000000, -0.000000, -1.000000, 1.0;
           -0.000000,  1.000000,  0.000000, 1.0;
            0.270598,  0.923880, -0.270598, 1.0;
            0.500000,  0.707107, -0.500000, 1.0;
            0.653281,  0.382683, -0.653281, 1.0;
            0.707106, -0.000000, -0.707107, 1.0;
            0.382683,  0.923880, -0.000000, 1.0;
            0.707106,  0.707107, -0.000000, 1.0;
            0.923879,  0.382683, -0.000000, 1.0;
            1.000000, -0.000000, -0.000000, 1.0;
            0.270598,  0.923880,  0.270598, 1.0;
            0.500000,  0.707107,  0.500000, 1.0;
            0.653281,  0.382683,  0.653281, 1.0;
            0.707106, -0.000000,  0.707107, 1.0;
           -0.000000,  0.923880,  0.382683, 1.0;
           -0.000000,  0.707107,  0.707107, 1.0;
           -0.000000,  0.382683,  0.923879, 1.0;
           -0.000000, -0.000000,  1.000000, 1.0;
           -0.270598,  0.923880,  0.270598, 1.0;
           -0.500000,  0.707107,  0.500000, 1.0;
           -0.653281,  0.382683,  0.653281, 1.0;
           -0.707107, -0.000000,  0.707107, 1.0;
        }
    };

    test! {
        obj.polygons {
            P   vec!(4, 3, 7)
            P   vec!(3, 2, 6)
            P   vec!(1, 5, 6)
            P   vec!(7, 11, 12)
            P   vec!(6, 10, 11)
            P   vec!(5, 9, 10)
            P   vec!(11, 16, 17)
            P   vec!(11, 10, 15)
            P   vec!(10, 9, 14)
            P   vec!(16, 20, 21)
            P   vec!(15, 19, 20)
            P   vec!(14, 18, 19)
            P   vec!(20, 24, 25)
            P   vec!(20, 19, 23)
            P   vec!(18, 22, 23)
            P   vec!(24, 28, 29)
            P   vec!(24, 23, 27)
            P   vec!(23, 22, 26)
            P   vec!(28, 32, 33)
            P   vec!(28, 27, 31)
            P   vec!(27, 26, 30)
            P   vec!(1, 13, 5)
            P   vec!(5, 13, 9)
            P   vec!(9, 13, 14)
            P   vec!(14, 13, 18)
            P   vec!(18, 13, 22)
            P   vec!(22, 13, 26)
            P   vec!(26, 13, 30)
            P   vec!(32, 3, 4)
            P   vec!(31, 2, 3)
            P   vec!(30, 1, 2)
            P   vec!(30, 13, 1)
            P   vec!(8, 4, 7)
            P   vec!(7, 3, 6)
            P   vec!(2, 1, 6)
            P   vec!(8, 7, 12)
            P   vec!(7, 6, 11)
            P   vec!(6, 5, 10)
            P   vec!(12, 11, 17)
            P   vec!(16, 11, 15)
            P   vec!(15, 10, 14)
            P   vec!(17, 16, 21)
            P   vec!(16, 15, 20)
            P   vec!(15, 14, 19)
            P   vec!(21, 20, 25)
            P   vec!(24, 20, 23)
            P   vec!(19, 18, 23)
            P   vec!(25, 24, 29)
            P   vec!(28, 24, 27)
            P   vec!(27, 23, 26)
            P   vec!(29, 28, 33)
            P   vec!(32, 28, 31)
            P   vec!(31, 27, 30)
            P   vec!(33, 32, 4)
            P   vec!(32, 31, 3)
            P   vec!(31, 30, 2)
            P   vec!(33, 4, 8)
            P   vec!(29, 33, 25)
            P   vec!(12, 17, 21)
            P   vec!(12, 33, 8)
            P   vec!(33, 21, 25)
            P   vec!(21, 33, 12)
        }
    };

    test! {
        obj.groups.get("default").unwrap().points.len(),        0
        obj.groups.get("default").unwrap().lines.len(),         0
        obj.groups.get("default").unwrap().polygons.len(),      1
        obj.groups.get("default").unwrap().polygons[0].start,   0
        obj.groups.get("default").unwrap().polygons[0].end,     62

        obj.meshes.get("None").unwrap().points.len(),           0
        obj.meshes.get("None").unwrap().lines.len(),            0
        obj.meshes.get("None").unwrap().polygons.len(),         1
        obj.meshes.get("None").unwrap().polygons[0].start,      0
        obj.meshes.get("None").unwrap().polygons[0].end,        62

        obj.smoothing_groups[1].points.len(),                   0
        obj.smoothing_groups[1].lines.len(),                    0
        obj.smoothing_groups[1].polygons.len(),                 1
        obj.smoothing_groups[1].polygons[0].start,              0
        obj.smoothing_groups[1].polygons[0].end,                56

        obj.smoothing_groups[2].points.len(),                   0
        obj.smoothing_groups[2].lines.len(),                    0
        obj.smoothing_groups[2].polygons.len(),                 1
        obj.smoothing_groups[2].polygons[0].start,              56
        obj.smoothing_groups[2].polygons[0].end,                62
    };
}
