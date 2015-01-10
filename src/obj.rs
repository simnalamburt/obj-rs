use std::str::FromStr;
use std::collections::{HashMap, VecMap};
use std::simd::f32x4;
use lex::lex;
use error::{parse_error, ParseErrorKind};

static DEFAULT_GROUP: &'static str = "default";
static DEFAULT_MATERIAL: &'static str = "";

/// Parses a wavefront `.obj` file
pub fn obj<T: Buffer>(input: &mut T) -> Obj {
    let mut obj = Obj::new();

    // TODO : start_group, start_material
    let mut current_group = DEFAULT_GROUP.to_string();
    let mut current_material = DEFAULT_MATERIAL.to_string();
    let mut current_smooth = 0us;
    let mut current_merge = 0us;

    lex(input, |stmt, args| {
        macro_rules! f {
            ($args:ident) => ({ &$args.iter().map(|&input| n(input)).collect::<Vec<f32>>()[] })
        }
        macro_rules! s {
            ($param:ident) => { &$param.split('/').collect::<Vec<&str>>()[] }
        }

        match stmt {
            // Vertex data
            "v" => obj.vertices.push(match f!(args) {
                [x, y, z, w] => f32x4(x, y, z, w),
                [x, y, z] => f32x4(x, y, z, 1.0),
                _ => error!(WrongNumberOfArguments)
            }),
            "vt" => obj.tex_coords.push(match f!(args) {
                [u, v, w] => f32x4(u, v, w, 0.0),
                [u, v] => f32x4(u, v, 0.0, 0.0),
                [u] => f32x4(u, 0.0, 0.0, 0.0),
                _ => error!(WrongNumberOfArguments)
            }),
            "vn" => obj.normals.push(match f!(args) {
                [x, y, z] => f32x4(x, y, z, 0.0),
                _ => error!(WrongNumberOfArguments)
            }),
            "vp" => obj.param_vertices.push(match f!(args) {
                [u, v, w] => f32x4(u, v, w, 0.0),
                [u, v] => f32x4(u, v, 1.0, 0.0),
                [u] => f32x4(u, 0.0, 1.0, 0.0),
                _ => error!(WrongNumberOfArguments)
            }),

            // Free-form curve / surface attributes
            "cstype" => {
                let _rational: bool;
                let geometry = match args {
                    ["rat", ty] => {
                        _rational = true;
                        ty
                    }
                    [ty] => {
                        _rational = false;
                        ty
                    }
                    _ => unimplemented!()
                };

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
                [_deg_u, _deg_v]  => unimplemented!(),
                [_deg_u] => unimplemented!(),
                _ => unimplemented!(),
            },
            "bmat" => unimplemented!(),
            "step" => unimplemented!(),

            // Elements
            "p" => unimplemented!(),
            "l" => unimplemented!(),
            "f" => {
                if args.len() < 3 { unimplemented!() }
                let mut args = args.iter();
                let first = args.next().unwrap();

                macro_rules! m {
                    { $($name:ident $pat:pat => $exp:expr)* } => (
                        // First, detect the type of the vertices with the first argument
                        // Then apply it to the rest of the arguments
                        match s!(first) {
                            $(
                                $pat => Polygon::$name({
                                    let mut polygon = vec![ $exp ];
                                    for param in args {
                                        match s!(param) {
                                            $pat => polygon.push($exp),
                                            _ => unimplemented!()
                                        }
                                    }
                                    polygon
                                }),
                            )*
                            _ => unimplemented!()
                        }
                    )
                }

                obj.polygons.push(m! {
                    P   [p]        => (n(p))
                    PT  [p, t]     => (n(p), n(t))
                    PN  [p, "", u] => (n(p), n(u))
                    PTN [p, t, u]  => (n(p), n(t), n(u))
                });
            }
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
            "g" => match args {
                [name] if name != &current_group[] => {
                    // TODO : end_group, start_group
                    current_group = name.to_string();
                }
                _ => unimplemented!()
            },
            "s" => {
                let smooth = match args {
                    ["off"] => 0us,
                    [param] => n(param),
                    _ => error!(WrongNumberOfArguments)
                };
                if smooth != current_smooth {
                    // TODO : maybe(end_smooth), start_smooth
                    current_smooth = smooth;
                }
            }
            "mg" => {
                let merge = match args {
                    ["off"] => 0us,
                    [param] => n(param),
                    _ => error!(WrongNumberOfArguments)
                };
                if merge != current_merge {
                    // TODO : maybe(end_merge), start_merge
                    current_merge = merge;
                }
            }
            "o" => {
                if !obj.name.is_empty() { unimplemented!() }

                obj.name = args.connect(" ");
            }

            // Display / render attributes
            "bevel" => unimplemented!(),
            "c_interp" => unimplemented!(),
            "d_interp" => unimplemented!(),
            "lod" => unimplemented!(),
            "usemtl" => match args {
                [material] if material != &current_material[] => {
                    // TODO : end_material, start_material
                    current_material = material.to_string();
                },
                _ => error!(WrongNumberOfArguments)
            },
            "mtllib" => {
                let paths: Vec<String> = args.iter().map(|path| path.to_string()).collect();
                obj.material_libraries.push_all(&paths[]);
            }
            "shadow_obj" => unimplemented!(),
            "trace_obj" => unimplemented!(),
            "ctech" => unimplemented!(),
            "stech" => unimplemented!(),

            // Unexpected statement
            _ => error!(UnexpectedStatement)
        }

        fn n<T: FromStr>(input: &str) -> T {
            match input.parse::<T>() {
                Some(number) => number,
                None => unimplemented!()
            }
        }

        None
    });

    // TODO : end_group, end_material, maybe(end_smooth), maybe(end_merge)

    obj
}


/// Parsed obj file
pub struct Obj {
    pub name: String,
    pub material_libraries: Vec<String>,

    pub vertices: Vec<f32x4>,
    pub tex_coords: Vec<f32x4>,
    pub normals: Vec<f32x4>,
    pub param_vertices: Vec<f32x4>,

    pub points: Vec<Point>,
    pub lines: Vec<Line>,
    pub polygons: Vec<Polygon>,

    pub groups: HashMap<String, Group>,
    pub meshes: HashMap<String, Group>,
    pub smoothing_groups: VecMap<Group>,
    pub merging_groups: VecMap<Group>
}

impl Obj {
    fn new() -> Self {
        let mut ret = Obj {
            name: String::new(),
            material_libraries: Vec::new(),

            vertices: Vec::new(),
            tex_coords: Vec::new(),
            normals: Vec::new(),
            param_vertices: Vec::new(),

            points: Vec::new(),
            lines: Vec::new(),
            polygons: Vec::new(),

            groups: HashMap::with_capacity(1),
            meshes: HashMap::with_capacity(1),
            smoothing_groups: VecMap::new(),
            merging_groups: VecMap::new()
        };

        ret.groups.insert(DEFAULT_GROUP.to_string(), Group::new());
        ret.meshes.insert(DEFAULT_MATERIAL.to_string(), Group::new());
        ret
    }
}

pub type Point = usize;

#[derive(Copy)]
pub enum Line {
    P([u32; 2]),
    PT([(u32, u32); 2])
}

#[derive(PartialEq, Eq, Show)]
pub enum Polygon {
    P(Vec<u32>),
    PT(Vec<(u32, u32)>),
    PN(Vec<(u32, u32)>),
    PTN(Vec<(u32, u32, u32)>)
}

pub struct Group {
    pub points: Vec<Range>,
    pub lines: Vec<Range>,
    pub polygons: Vec<Range>
}

impl Group {
    fn new() -> Self {
        Group {
            points: Vec::new(),
            lines: Vec::new(),
            polygons: Vec::new()
        }
    }
}

#[derive(Copy)]
pub struct Range {
    pub start: usize,
    pub end: usize
}
