use std::simd::f32x4;
use lex::lex;
use error::{parse_error, ParseErrorKind};

/// Parses a wavefront `.obj` file
pub fn obj<T: Buffer>(input: &mut T) -> Obj {
    let mut obj = Obj::new();

    lex(input, |stmt, args| {
        macro_rules! f {
            ($args:ident) => {
                $args.iter()
                    .map(|&input| match input.parse::<f32>() {
                        Some(number) => number,
                        None => unimplemented!()
                    })
                    .collect::<Vec<f32>>()
                    .as_slice()
            }
        }

        macro_rules! s {
            ($param:ident) => {
                $param.split('/').collect::<Vec<&str>>().as_slice()
            }
        }

        macro_rules! error {
            ($kind:ident) => {
                return Some(parse_error(ParseErrorKind::$kind))
            }
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
                    { $($pat:pat => $exp:expr : $name:ident),* } => (
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

                obj.last_group().last_mesh().polygons.push(m! {
                    [p]         => (u(p)): P,
                    [p, t]      => (u(p), u(t)): PT,
                    [p, "", n]  => (u(p), u(n)): PN,
                    [p, t, n]   => (u(p), u(t), u(n)): PTN
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
                [name] => if obj.last_group().is_empty() {
                    obj.last_group().name = name.to_string()
                } else {
                    let group = Group::new(name, obj.last_group().last_mesh().material.as_slice());
                    obj.groups.push(group)
                },
                _ => unimplemented!()
            },
            "s" => match args {
                ["off"] | ["0"] => (),
                [param] => {
                    let _index = u(param);
                    unimplemented!()
                }
                _ => error!(WrongNumberOfArguments)
            },
            "mg" => unimplemented!(),
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
                [material] => {
                    let last_group = obj.last_group();
                    if last_group.last_mesh().is_empty() {
                        last_group.last_mesh().material = material.to_string()
                    } else {
                        last_group.meshes.push(Mesh::new(material))
                    }
                }
                _ => error!(WrongNumberOfArguments)
            },
            "mtllib" => {
                let paths: Vec<String> = args.iter().map(|path| path.to_string()).collect();
                obj.material_libraries.push_all(paths.as_slice());
            }
            "shadow_obj" => unimplemented!(),
            "trace_obj" => unimplemented!(),
            "ctech" => unimplemented!(),
            "stech" => unimplemented!(),

            // Unexpected statement
            _ => error!(UnexpectedStatement)
        }

        fn u(input: &str) -> u32 {
            match input.parse::<u32>() {
                Some(number) => number,
                None => unimplemented!()
            }
        }

        None
    });

    obj
}


/// Parsed obj file
pub struct Obj {
    pub name: String,

    pub vertices: Vec<f32x4>,
    pub tex_coords: Vec<f32x4>,
    pub normals: Vec<f32x4>,
    pub param_vertices: Vec<f32x4>,

    pub material_libraries: Vec<String>,

    pub groups: Vec<Group>
}

impl Obj {
    fn new() -> Self {
        Obj {
            name: String::new(),
            vertices: Vec::new(),
            tex_coords: Vec::new(),
            normals: Vec::new(),
            param_vertices: Vec::new(),
            material_libraries: Vec::new(),
            groups: vec![ Group::new("default", "") ]
        }
    }

    fn last_group<'a>(&'a mut self) -> &'a mut Group {
        let len = self.groups.len();
        &mut self.groups[len - 1]
    }
}

pub struct Group {
    pub name: String,
    pub meshes: Vec<Mesh>
}

impl Group {
    fn new(name: &str, material: &str) -> Self {
        Group { name: name.to_string(), meshes: vec![ Mesh::new(material) ] }
    }

    fn last_mesh<'a>(&'a mut self) -> &'a mut Mesh {
        let len = self.meshes.len();
        &mut self.meshes[len - 1]
    }

    fn is_empty(&self) -> bool {
        self.meshes.is_empty()
    }
}

pub struct Mesh {
    pub material: String,
    pub polygons: Vec<Polygon>
}

impl Mesh {
    fn new(material: &str) -> Self {
        Mesh { material: material.to_string(), polygons: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        self.polygons.is_empty()
    }
}

#[deriving(PartialEq, Eq, Show)]
pub enum Polygon {
    P(Vec<u32>),
    PT(Vec<(u32, u32)>),
    PN(Vec<(u32, u32)>),
    PTN(Vec<(u32, u32, u32)>)
}
