// Copyright 2014-2017 Hyeon Kim
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate obj;

use obj::raw::{RawObj, parse_obj};

fn fixture(filename: &str) -> RawObj {
    use std::path::Path;
    use std::fs::File;
    use std::io::BufReader;

    let path = Path::new("tests").join("fixtures").join(filename);
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => panic!("Failed to open \"{}\". \x1b[31m{}\x1b[0m", path.to_string_lossy(), e)
    };
    let input = BufReader::new(file);

    parse_obj(input).unwrap()
}

macro_rules! test_v4 {
    ($lhs:expr => $($x:expr, $y:expr, $z:expr, $w:expr;)*) => ({
        let mut index = 0usize;
        $(
            let (x, y, z, w) = $lhs[index];
            eq!(x, stringify!($x).parse().unwrap(), stringify!($lhs[index].x));
            eq!(y, stringify!($y).parse().unwrap(), stringify!($lhs[index].y));
            eq!(z, stringify!($z).parse().unwrap(), stringify!($lhs[index].z));
            eq!(w, stringify!($w).parse().unwrap(), stringify!($lhs[index].w));
            index += 1;
        )*
        eq!($lhs.len(), index);
    });
}

macro_rules! test_v3 {
    ($lhs:expr => $($x:expr, $y:expr, $z:expr;)*) => ({
        let mut index = 0usize;
        $(
            let (x, y, z) = $lhs[index];
            eq!(x, stringify!($x).parse().unwrap(), stringify!($lhs[index].x));
            eq!(y, stringify!($y).parse().unwrap(), stringify!($lhs[index].y));
            eq!(z, stringify!($z).parse().unwrap(), stringify!($lhs[index].z));
            index += 1;
        )*
        eq!($lhs.len(), index);
    });
}

macro_rules! test {
    ($($lhs:expr => { $($kind:ident $elem:expr)* })*) => ({
        $({
            let mut index = 0usize;
            $(
                eq!($lhs[index], obj::raw::object::Polygon::$kind($elem));
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
            use std::io::Write;
            let _ = writeln!(&mut std::io::stderr(), "\x1b[33m{}\x1b[0m should be \x1b[33m{:?}\x1b[0m, \
                     but it was \x1b[33m{:?}\x1b[0m", $exp, *right, *left);
            panic!($exp);
        }
    });
}

#[test]
fn empty() {
    let obj = fixture("empty.obj");

    test! {
        obj.name,                       None
        obj.material_libraries,         Vec::<String>::new()

        obj.positions.len(),            0
        obj.tex_coords.len(),           0
        obj.normals.len(),              0
        obj.param_vertices.len(),       0

        obj.points.len(),               0
        obj.lines.len(),                0
        obj.polygons.len(),             0

        obj.groups.len(),               0
        obj.meshes.len(),               0
        obj.smoothing_groups.len(),     0
        obj.merging_groups.len(),       0
    }
}

#[test]
fn cube() {
    let obj = fixture("cube.obj");

    test! {
        obj.name,                       Some("Cube".to_string())
        obj.material_libraries,         vec![ "cube.mtl" ]

        obj.positions.len(),            8
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

    test_v4! { obj.positions =>
        1.000000, -1.000000, -1.000000, 1.0;
        1.000000, -1.000000,  1.000000, 1.0;
       -1.000000, -1.000000,  1.000000, 1.0;
       -1.000000, -1.000000, -1.000000, 1.0;
        1.000000,  1.000000, -0.999999, 1.0;
        0.999999,  1.000000,  1.000001, 1.0;
       -1.000000,  1.000000,  1.000000, 1.0;
       -1.000000,  1.000000, -1.000000, 1.0;
    };

    test_v3! { obj.tex_coords =>
        1.004952,  0.498633,  0.000000;
        0.754996,  0.498236,  0.000000;
        0.755393,  0.248279,  0.000000;
        1.005349,  0.248677,  0.000000;
        0.255083,  0.497442,  0.000000;
        0.255480,  0.247485,  0.000000;
        0.505437,  0.247882,  0.000000;
        0.505039,  0.497839,  0.000000;
        0.754598,  0.748193,  0.000000;
        0.504642,  0.747795,  0.000000;
        0.505834, -0.002074,  0.000000;
        0.755790, -0.001677,  0.000000;
        0.005127,  0.497044,  0.000000;
        0.005524,  0.247088,  0.000000;
    };

    test! {
        obj.polygons => {
            PT  vec![ (0, 0), (1, 1), (2, 2), (3, 3)   ]
            PT  vec![ (4, 4), (7, 5), (6, 6), (5, 7)   ]
            PT  vec![ (0, 8), (4, 9), (5, 7), (1, 1)   ]
            PT  vec![ (1, 1), (5, 7), (6, 6), (2, 2)   ]
            PT  vec![ (2, 2), (6, 6), (7, 10), (3, 11) ]
            PT  vec![ (4, 4), (0, 12), (3, 13), (7, 5) ]
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
        obj.name,                       Some("Dome".to_string())
        obj.material_libraries,         vec![ "dome.mtl" ]

        obj.positions.len(),            33
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

    test_v4! { obj.positions =>
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
    };

    test! {
        obj.polygons => {
            P   vec!(3, 2, 6)
            P   vec!(2, 1, 5)
            P   vec!(0, 4, 5)
            P   vec!(6, 10, 11)
            P   vec!(5, 9, 10)
            P   vec!(4, 8, 9)
            P   vec!(10, 15, 16)
            P   vec!(10, 9, 14)
            P   vec!(9, 8, 13)
            P   vec!(15, 19, 20)
            P   vec!(14, 18, 19)
            P   vec!(13, 17, 18)
            P   vec!(19, 23, 24)
            P   vec!(19, 18, 22)
            P   vec!(17, 21, 22)
            P   vec!(23, 27, 28)
            P   vec!(23, 22, 26)
            P   vec!(22, 21, 25)
            P   vec!(27, 31, 32)
            P   vec!(27, 26, 30)
            P   vec!(26, 25, 29)
            P   vec!(0, 12, 4)
            P   vec!(4, 12, 8)
            P   vec!(8, 12, 13)
            P   vec!(13, 12, 17)
            P   vec!(17, 12, 21)
            P   vec!(21, 12, 25)
            P   vec!(25, 12, 29)
            P   vec!(31, 2, 3)
            P   vec!(30, 1, 2)
            P   vec!(29, 0, 1)
            P   vec!(29, 12, 0)
            P   vec!(7, 3, 6)
            P   vec!(6, 2, 5)
            P   vec!(1, 0, 5)
            P   vec!(7, 6, 11)
            P   vec!(6, 5, 10)
            P   vec!(5, 4, 9)
            P   vec!(11, 10, 16)
            P   vec!(15, 10, 14)
            P   vec!(14, 9, 13)
            P   vec!(16, 15, 20)
            P   vec!(15, 14, 19)
            P   vec!(14, 13, 18)
            P   vec!(20, 19, 24)
            P   vec!(23, 19, 22)
            P   vec!(18, 17, 22)
            P   vec!(24, 23, 28)
            P   vec!(27, 23, 26)
            P   vec!(26, 22, 25)
            P   vec!(28, 27, 32)
            P   vec!(31, 27, 30)
            P   vec!(30, 26, 29)
            P   vec!(32, 31, 3)
            P   vec!(31, 30, 2)
            P   vec!(30, 29, 1)
            P   vec!(32, 3, 7)
            P   vec!(28, 32, 24)
            P   vec!(11, 16, 20)
            P   vec!(11, 32, 7)
            P   vec!(32, 20, 24)
            P   vec!(20, 32, 11)
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

#[test]
fn sponza() {
    // Sponza atrium model, it's reasonably big and more importantly uses negative indexes
    // for some of the face specifications.
    let obj = fixture("sponza.obj");

    test! {
        obj.name,              Some("sponza.lwo".to_owned())
        obj.positions.len(),   39742
        obj.polygons.len(),    36347
    }
}

#[test]
fn lines_points() {
    // Basic test for lines and points geometry statements
    let obj = fixture("lines_points.obj");

    test! {
        obj.name,             Some("lines.obj".to_owned())
        obj.positions.len(),  3
        obj.tex_coords.len(), 2
        obj.lines.len(),      2
        obj.points.len(),     3
    }
}
