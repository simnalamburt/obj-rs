//! Parses `.obj` format which stores 3D mesh data

use std::collections::{HashMap, VecMap};
use std::simd::f32x4;
use lex::lex;
use error::{parse_error, ParseErrorKind};

/// Parses a wavefront `.obj` format
pub fn load_obj<T: Buffer>(mut input: T) -> Obj {
    let mut name = String::new();
    let mut material_libraries = Vec::new();

    let mut vertices = Vec::new();
    let mut tex_coords = Vec::new();
    let mut normals = Vec::new();
    let mut param_vertices = Vec::new();

    let points = Vec::new();
    let lines = Vec::new();
    let mut polygons = Vec::new();

    let counter = Counter::new(&points, &lines, &polygons);
    let mut group_builder       = counter.hash_map("default".to_string());
    let mut mesh_builder        = counter.hash_map(String::new());
    let mut smoothing_builder   = counter.vec_map();
    let mut merging_builder     = counter.vec_map();

    lex(&mut input, |stmt, args| {
        macro_rules! f {
            ($args:ident) => ({ &$args.iter().map(|&input| n(input)).collect::<Vec<f32>>()[] })
        }
        macro_rules! s {
            ($param:ident) => { &$param.split('/').collect::<Vec<&str>>()[] }
        }

        match stmt {
            // Vertex data
            "v" => vertices.push(match f!(args) {
                [x, y, z, w] => f32x4(x, y, z, w),
                [x, y, z] => f32x4(x, y, z, 1.0),
                _ => error!(WrongNumberOfArguments)
            }),
            "vt" => tex_coords.push(match f!(args) {
                [u, v, w] => f32x4(u, v, w, 0.0),
                [u, v] => f32x4(u, v, 0.0, 0.0),
                [u] => f32x4(u, 0.0, 0.0, 0.0),
                _ => error!(WrongNumberOfArguments)
            }),
            "vn" => normals.push(match f!(args) {
                [x, y, z] => f32x4(x, y, z, 0.0),
                _ => error!(WrongNumberOfArguments)
            }),
            "vp" => param_vertices.push(match f!(args) {
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

                polygons.push(m! {
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
                [name] => group_builder.start(name.to_string()),
                _ => unimplemented!()
            },
            "s" => match args {
                ["off"] | ["0"] => smoothing_builder.end(),
                [param] => smoothing_builder.start(n(param)),
                _ => error!(WrongNumberOfArguments)
            },
            "mg" => match args {
                ["off"] | ["0"] => merging_builder.end(),
                [param] => merging_builder.start(n(param)),
                _ => error!(WrongNumberOfArguments)
            },
            "o" => {
                if !name.is_empty() { unimplemented!() }

                name = args.connect(" ");
            }

            // Display / render attributes
            "bevel" => unimplemented!(),
            "c_interp" => unimplemented!(),
            "d_interp" => unimplemented!(),
            "lod" => unimplemented!(),
            "usemtl" => match args {
                [material] => mesh_builder.start(material.to_string()),
                _ => error!(WrongNumberOfArguments)
            },
            "mtllib" => {
                let paths: Vec<String> = args.iter().map(|path| path.to_string()).collect();
                material_libraries.push_all(&paths[]);
            }
            "shadow_obj" => unimplemented!(),
            "trace_obj" => unimplemented!(),
            "ctech" => unimplemented!(),
            "stech" => unimplemented!(),

            // Unexpected statement
            _ => error!(UnexpectedStatement)
        }

        fn n<T: ::std::str::FromStr>(input: &str) -> T {
            match input.parse() {
                Ok(number) => number,
                Err(_)=> unimplemented!()
            }
        }

        None
    });

    group_builder.end();
    mesh_builder.end();
    smoothing_builder.end();
    merging_builder.end();

    Obj {
        name: name,
        material_libraries: material_libraries,

        vertices: vertices,
        tex_coords: tex_coords,
        normals: normals,
        param_vertices: param_vertices,

        points: points,
        lines: lines,
        polygons: polygons,

        groups: group_builder.result,
        meshes: mesh_builder.result,
        smoothing_groups: smoothing_builder.result,
        merging_groups: merging_builder.result
    }
}


/// Counts current total count of parsed `points`, `lines` and `polygons`.
struct Counter {
    points:     *const Vec<Point>,
    lines:      *const Vec<Line>,
    polygons:   *const Vec<Polygon>,
}

impl Counter {
    /// Constructs a new `Counter`.
    fn new(points: *const Vec<Point>, lines: *const Vec<Line>, polygons: *const Vec<Polygon>) -> Self {
        Counter {
            points:     points,
            lines:      lines,
            polygons:   polygons
        }
    }

    /// Returns a current count of parsed `(points, lines, polygons)`.
    fn get(&self) -> (usize, usize, usize) {
        unsafe { ((*self.points).len(), (*self.lines).len(), (*self.polygons).len()) }
    }

    /// Creates a `HashMap<String, Group>` builder which references `self` as counter.
    fn hash_map<'a>(&'a self, input: String) -> GroupBuilder<'a, HashMap<String, Group>, String> {
        let mut init = Vec::with_capacity(1);
        init.start(0);

        let mut result = HashMap::with_capacity(1);
        result.insert(input.clone(), Group {
            points:     init.clone(),
            lines:      init.clone(),
            polygons:   init
        });

        GroupBuilder {
            counter: self,
            current: Some(input),
            result: result
        }
    }

    /// Creates a `VecMap<Group>` builder which references `self` as counter.
    fn vec_map<'a>(&'a self) -> GroupBuilder<'a, VecMap<Group>, usize> {
        GroupBuilder {
            counter: self,
            current: None,
            result: VecMap::new()
        }
    }
}


/// Helper for creating `groups`, `meshes`, `smoothing_groups` and `merging_groups` member of
/// `Obj`.
struct GroupBuilder<'a, T, K> {
    counter: &'a Counter,
    current: Option<K>, // Some(K) if some group has been started
                        // None    otherwise
    result: T
}

impl<'a, T, K> GroupBuilder<'a, T, K> where
    T: Map<K, Group>,
    K: Clone + Key
{
    /// Starts a group whose name is `input`.
    fn start(&mut self, input: K) {
        let (points, lines, polygons) = self.counter.get();

        match self.current {
            Some(ref current) if *current != input => {
                let is_empty = {
                    let old = &mut self.result[*current];
                    old.points  .end(points);
                    old.lines   .end(lines);
                    old.polygons.end(polygons);

                    old.points.is_empty() && old.lines.is_empty() && old.polygons.is_empty()
                };

                if is_empty {
                    let result = self.result.remove(current);
                    assert!(result.is_some());
                }
            }
            Some(_) => return,
            None => ()
        }

        let mut group = Group::new();
        group.points   .start(points);
        group.lines    .start(lines);
        group.polygons .start(polygons);

        self.current = Some(input.clone());

        let result = self.result.insert(input, group);
        assert!(result.is_none());
    }

    /// Ends a current group.
    fn end(&mut self) {
        match self.current {
            Some(ref current) => {
                let (points, lines, polygons) = self.counter.get();
                let old = &mut self.result[*current];
                old.points  .end(points);
                old.lines   .end(lines);
                old.polygons.end(polygons);
            }
            None => return
        }

        self.current = None;
    }
}


/// Custom trait to interface `HashMap` and `VecMap`.
trait Map<K: Key, V: ?Sized> : ::std::ops::IndexMut<K, Output=V> {
    /// Interface of `insert` function.
    fn insert(&mut self, K, V) -> Option<V>;
    /// Interface of `get_mut` function.
    fn get_mut(&mut self, k: &K) -> Option<&mut V>;
    /// Interface of `remove` function.
    fn remove(&mut self, k: &K) -> Option<V>;
}

impl<V> Map<String, V> for HashMap<String, V> {
    fn insert(&mut self, k: String, v: V) -> Option<V> {
        self.insert(k, v)
    }

    fn get_mut(&mut self, k: &String) -> Option<&mut V> {
        self.get_mut(k)
    }

    fn remove(&mut self, k: &String) -> Option<V> {
        self.remove(k)
    }
}

impl<V> Map<usize, V> for VecMap<V> {
    fn insert(&mut self, k: usize, v: V) -> Option<V> {
        self.insert(k, v)
    }

    fn get_mut(&mut self, k: &usize) -> Option<&mut V> {
        self.get_mut(k)
    }

    fn remove(&mut self, k: &usize) -> Option<V> {
        self.remove(k)
    }
}

/// A trait which should be implemented by a type passed into `Key` of `Map`.
trait Key : Eq {}

impl Key for String {}
impl Key for usize {}


/// Custom trait for `Vec<Range>`.
trait RangeVec {
    /// Starts new range
    fn start(&mut self, usize);

    /// Tie up the loose end of the `Vec<Range>`
    fn end(&mut self, usize);
}

/// Constant which is used to represent undefined bound of range.
static UNDEFINED: usize = ::std::usize::MAX;

impl RangeVec for Vec<Range> {
    fn start(&mut self, start: usize) {
        self.push(Range {
            start: start,
            end: UNDEFINED
        })
    }

    fn end(&mut self, end: usize) {
        let last = self.len() - 1;
        assert_eq!(self[last].end, UNDEFINED);
        if self[last].start != end {
            self[last].end = end;
        } else {
            self.pop();
        }
    }
}


/// Low-level Rust binding for `.obj` format.
pub struct Obj {
    /// Name of the object.
    pub name: String,
    /// `.mtl` files which required by this object.
    pub material_libraries: Vec<String>,

    /// Position vectors of each vertex.
    pub vertices: Vec<f32x4>,
    /// Texture coordinates of each vertex.
    pub tex_coords: Vec<f32x4>,
    /// Normal vectors of each vertex.
    pub normals: Vec<f32x4>,
    /// Parametric vertices.
    pub param_vertices: Vec<f32x4>,

    /// Points which stores the index data of position vectors.
    pub points: Vec<Point>,
    /// Lines which store the index data of vectors.
    pub lines: Vec<Line>,
    /// Polygons which store the index data of vectors.
    pub polygons: Vec<Polygon>,

    /// Groups of multiple geometries.
    pub groups: HashMap<String, Group>,
    /// Geometries which consist in a same material.
    pub meshes: HashMap<String, Group>,
    /// Smoothing groups.
    pub smoothing_groups: VecMap<Group>,
    /// Merging groups.
    pub merging_groups: VecMap<Group>
}

/// The `Point` type which stores the index of the position vector.
pub type Point = usize;

/// The `Line` type.
#[derive(Clone, Copy)]
pub enum Line {
    /// A line which contains only the position data of both ends
    P([u32; 2]),
    /// A line which contains both position and texture coordinate data of both ends
    PT([(u32, u32); 2])
}

/// The `Polygon` type.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Polygon {
    /// A polygon which contains only the position data of each vertex.
    P(Vec<u32>),
    /// A polygon which contains both position and texture coordinate data of each vertex.
    PT(Vec<(u32, u32)>),
    /// A polygon which contains both position and normal data of each vertex.
    PN(Vec<(u32, u32)>),
    /// A polygon which contains all position, texture coordinate and normal data of each vertex.
    PTN(Vec<(u32, u32, u32)>)
}

/// A group which contains multiple range of points, lines and polygons
#[derive(Clone, Debug)]
pub struct Group {
    /// Multiple range of points
    pub points: Vec<Range>,
    /// Multiple range of lines
    pub lines: Vec<Range>,
    /// Multiple range of polygons
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

/// A struct which represent `[start, end)` range.
#[derive(Clone, Copy, Debug)]
pub struct Range {
    /// The lower bound of the range (inclusive).
    pub start: usize,
    /// The upper bound of the range (exclusive).
    pub end: usize
}
