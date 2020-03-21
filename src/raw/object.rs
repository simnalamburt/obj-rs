// Copyright 2014-2017 Hyeon Kim
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Parses `.obj` format which stores 3D mesh data

use error::ObjResult;
use raw::lexer::lex;
use std::collections::HashMap;
use std::io::BufRead;
use vec_map::VecMap;

/// Parses &[&str] into &[f32].
macro_rules! f {
    ($args:expr) => (
        &{
            let mut ret = Vec::<f32>::new();
            ret.reserve($args.len());
            for arg in $args {
                ret.push(try!(arg.parse()))
            }
            ret
        }[..]
    )
}

// Helper macro for handling the indexes.
// If the index is < 0, then it represents an offset from the end of
// the current list. So -1 is the most recently added vertex.
// If the index is > 0 then it's simply the position in the list such
// that 1 is the first vertex.
macro_rules! idx ( ($i:expr, $l:expr) => ({
    let i = $i;
    if i < 0 {
        let i = (i * -1) as usize;
        $l.len() - i
    } else {
        (i - 1) as usize
    }
}));

/// Parses a wavefront `.obj` format.
pub fn parse_obj<T: BufRead>(input: T) -> ObjResult<RawObj> {
    let mut name = None;
    let mut material_libraries = Vec::new();

    let mut positions = Vec::new();
    let mut tex_coords = Vec::new();
    let mut normals = Vec::new();
    let mut param_vertices = Vec::new();

    let mut points = Vec::new();
    let mut lines = Vec::new();
    let mut polygons = Vec::new();

    let counter = Counter::new(&points, &lines, &polygons);
    let mut group_builder = counter.hash_map("default".to_string());
    let mut mesh_builder = counter.hash_map(String::new());
    let mut smoothing_builder = counter.vec_map();
    let mut merging_builder = counter.vec_map();

    try!(lex(input, |stmt, args| {
        match stmt {
            // Vertex data
            "v" => {
                let args = f!(args);
                positions.push(match args.len() {
                    4 => (args[0], args[1], args[2], args[3]),
                    3 => (args[0], args[1], args[2], 1.0),
                    _ => error!(WrongNumberOfArguments, "Expected 3 or 4 arguments"),
                })
            }
            "vt" => {
                let args = f!(args);
                tex_coords.push(match args.len() {
                    3 => (args[0], args[1], args[2]),
                    2 => (args[0], args[1], 0.0),
                    1 => (args[0], 0.0, 0.0),
                    _ => error!(WrongNumberOfArguments, "Expected 1, 2 or 3 arguments"),
                })
            }
            "vn" => {
                let args = f!(args);
                normals.push(match args.len() {
                    3 => (args[0], args[1], args[2]),
                    _ => error!(WrongNumberOfArguments, "Expected 3 arguments"),
                })
            }
            "vp" => {
                let args = f!(args);
                param_vertices.push(match args.len() {
                    3 => (args[0], args[1], args[2]),
                    2 => (args[0], args[1], 1.0),
                    1 => (args[0], 0.0, 1.0),
                    _ => error!(WrongNumberOfArguments, "Expected 1, 2 or 3 arguments"),
                })
            }

            // Free-form curve / surface attributes
            "cstype" => {
                let _rational: bool;
                let geometry = match args.len() {
                    2 if args[0] == "rat" => {
                        _rational = true;
                        args[1]
                    }
                    1 => {
                        _rational = false;
                        args[0]
                    }
                    _ => error!(WrongTypeOfArguments, "Expected 'rat xxx' or 'xxx' format"),
                };

                match geometry {
                    "bmatrix" => unimplemented!(),
                    "bezier" => unimplemented!(),
                    "bspline" => unimplemented!(),
                    "cardinal" => unimplemented!(),
                    "taylor" => unimplemented!(),
                    _ => error!(
                        WrongTypeOfArguments,
                        "Expected one of 'bmatrix', 'bezier', 'bspline', 'cardinal' and 'taylor'"
                    ),
                }
            }
            "deg" => {
                let args = f!(args);
                match args.len() {
                    2 => unimplemented!(), // (deg_u, deg_v)
                    1 => unimplemented!(), // (deg_u)
                    _ => error!(WrongNumberOfArguments, "Expected 1 or 2 arguments"),
                }
            }
            "bmat" => unimplemented!(),
            "step" => unimplemented!(),

            // Elements
            "p" => {
                for v in args {
                    let v: i32 = try!(v.parse());
                    let v = idx!(v, positions);
                    points.push(v);
                }
            }
            "l" => {
                if args.len() < 2 {
                    error!(WrongNumberOfArguments, "Expected at least 2 arguments")
                }
                let mut args = args.iter();
                let first = args.next().unwrap();
                let rest = args;

                let group = try!(parse_vertex_group(first));

                let line = match group {
                    (p, 0, 0) => {
                        let mut points = vec![idx!(p, positions)];
                        for gs in rest {
                            let group = try!(parse_vertex_group(gs));
                            if group.1 != 0 || group.2 != 0 {
                                error!(WrongTypeOfArguments, "Unexpected vertex format");
                            }

                            points.push(idx!(group.0, positions));
                        }
                        Line::P(points)
                    }
                    (p, t, 0) => {
                        let mut points = vec![(idx!(p, positions), idx!(t, tex_coords))];
                        for gs in rest {
                            let group = try!(parse_vertex_group(gs));
                            if group.2 != 0 {
                                error!(WrongTypeOfArguments, "Unexpected vertex format");
                            }

                            points.push((idx!(group.0, positions), idx!(group.1, tex_coords)));
                        }
                        Line::PT(points)
                    }
                    _ => {
                        error!(
                            WrongTypeOfArguments,
                            "Unexpected vertex format, expected `#` or `#/#`"
                        );
                    }
                };
                lines.push(line);
            }
            "fo" | "f" => {
                if args.len() < 3 {
                    error!(WrongNumberOfArguments, "Expected at least 3 arguments")
                }

                let mut args = args.iter();
                let first = args.next().unwrap();
                let rest = args;

                let group = try!(parse_vertex_group(first));

                let polygon = match group {
                    (p, 0, 0) => {
                        let mut polygon = vec![idx!(p, positions)];
                        for gs in rest {
                            let group = try!(parse_vertex_group(gs));
                            if group.1 != 0 || group.2 != 0 {
                                error!(WrongTypeOfArguments, "Unexpected vertex format");
                            }

                            polygon.push(idx!(group.0, positions));
                        }

                        Polygon::P(polygon)
                    }
                    (p, t, 0) => {
                        let mut polygon = vec![(idx!(p, positions), idx!(t, tex_coords))];
                        for gs in rest {
                            let group = try!(parse_vertex_group(gs));
                            if group.2 != 0 {
                                error!(WrongTypeOfArguments, "Unexpected vertex format");
                            }

                            polygon.push((idx!(group.0, positions), idx!(group.1, tex_coords)));
                        }

                        Polygon::PT(polygon)
                    }
                    (p, 0, n) => {
                        let mut polygon = vec![(idx!(p, positions), idx!(n, normals))];
                        for gs in rest {
                            let group = try!(parse_vertex_group(gs));
                            if group.1 != 0 {
                                error!(WrongTypeOfArguments, "Unexpected vertex format");
                            }

                            polygon.push((idx!(group.0, positions), idx!(group.2, normals)));
                        }

                        Polygon::PN(polygon)
                    }
                    (p, t, n) => {
                        let mut polygon =
                            vec![(idx!(p, positions), idx!(t, tex_coords), idx!(n, normals))];
                        for gs in rest {
                            let group = try!(parse_vertex_group(gs));
                            polygon.push((
                                idx!(group.0, positions),
                                idx!(group.1, tex_coords),
                                idx!(group.2, normals),
                            ));
                        }

                        Polygon::PTN(polygon)
                    }
                };

                polygons.push(polygon);
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
            "g" => match args.len() {
                1 => group_builder.start(args[0].to_string()),
                _ => error!(
                    WrongNumberOfArguments,
                    "Expected group name parameter, but nothing has been supplied"
                ),
            },
            "s" => match args.len() {
                1 if (args[0] == "off" || args[0] == "0") => smoothing_builder.end(),
                1 => smoothing_builder.start(try!(args[0].parse())),
                _ => error!(WrongNumberOfArguments, "Expected only 1 argument"),
            },
            "mg" => match args.len() {
                1 if (args[0] == "off" || args[0] == "0") => merging_builder.end(),
                1 => merging_builder.start(try!(args[0].parse())),
                _ => error!(WrongNumberOfArguments, "Expected only 1 argument"),
            },
            "o" => {
                name = match args.len() {
                    0 => None,
                    _ => Some(args.join(" ")),
                }
            }

            // Display / render attributes
            "bevel" => unimplemented!(),
            "c_interp" => unimplemented!(),
            "d_interp" => unimplemented!(),
            "lod" => unimplemented!(),
            "usemtl" => match args.len() {
                1 => mesh_builder.start(args[0].to_string()),
                _ => error!(WrongNumberOfArguments, "Expected only 1 argument"),
            },
            "mtllib" => {
                // TODO: .push_all()
                material_libraries.reserve(args.len());
                for path in args {
                    material_libraries.push(path.to_string());
                }
            }
            "shadow_obj" => unimplemented!(),
            "trace_obj" => unimplemented!(),
            "ctech" => unimplemented!(),
            "stech" => unimplemented!(),

            // Unexpected statement
            _ => error!(UnexpectedStatement, "Received unknown statement"),
        }

        Ok(())
    }));

    group_builder.end();
    mesh_builder.end();
    smoothing_builder.end();
    merging_builder.end();

    Ok(RawObj {
        name: name,
        material_libraries: material_libraries,

        positions: positions,
        tex_coords: tex_coords,
        normals: normals,
        param_vertices: param_vertices,

        points: points,
        lines: lines,
        polygons: polygons,

        groups: group_builder.result,
        meshes: mesh_builder.result,
        smoothing_groups: smoothing_builder.result,
        merging_groups: merging_builder.result,
    })
}

// Parses the vertex group in the face statement, missing entries
// are indicated with a 0 value
fn parse_vertex_group(s: &str) -> ObjResult<(i32, i32, i32)> {
    let mut indices = s.split('/');

    let first = indices.next().unwrap_or("");
    let second = indices.next().unwrap_or("");
    let third = indices.next().unwrap_or("");

    let first = try!(first.parse());
    let second = if second == "" {
        0
    } else {
        try!(second.parse())
    };

    let third = if third == "" { 0 } else { try!(third.parse()) };

    Ok((first, second, third))
}

/// Counts current total count of parsed `points`, `lines` and `polygons`.
struct Counter {
    points: *const Vec<Point>,
    lines: *const Vec<Line>,
    polygons: *const Vec<Polygon>,
}

impl Counter {
    /// Constructs a new `Counter`.
    fn new(
        points: *const Vec<Point>,
        lines: *const Vec<Line>,
        polygons: *const Vec<Polygon>,
    ) -> Self {
        Counter {
            points: points,
            lines: lines,
            polygons: polygons,
        }
    }

    /// Returns a current count of parsed `(points, lines, polygons)`.
    fn get(&self) -> (usize, usize, usize) {
        unsafe {
            (
                (*self.points).len(),
                (*self.lines).len(),
                (*self.polygons).len(),
            )
        }
    }

    /// Creates a `HashMap<String, Group>` builder which references `self` as counter.
    fn hash_map<'a>(&'a self, input: String) -> GroupBuilder<'a, HashMap<String, Group>, String> {
        let mut result = HashMap::with_capacity(1);
        result.insert(input.clone(), Group::new((0, 0, 0)));

        GroupBuilder {
            counter: self,
            current: Some(input),
            result: result,
        }
    }

    /// Creates a `VecMap<Group>` builder which references `self` as counter.
    fn vec_map<'a>(&'a self) -> GroupBuilder<'a, VecMap<Group>, usize> {
        GroupBuilder {
            counter: self,
            current: None,
            result: VecMap::new(),
        }
    }
}

/// Helper for creating `groups`, `meshes`, `smoothing_groups` and `merging_groups` member of
/// `Obj`.
struct GroupBuilder<'a, T, K> {
    counter: &'a Counter,
    current: Option<K>, // Some(K) if some group has been started
    // None    otherwise
    result: T,
}

impl<'a, T, K> GroupBuilder<'a, T, K>
where
    T: Map<K, Group>,
    K: Clone + Key,
{
    /// Starts a group whose name is `input`.
    fn start(&mut self, input: K) {
        let count = self.counter.get();
        if let Some(ref current) = self.current {
            if *current == input {
                return;
            }
            if self.result.get_mut(current).unwrap().end(count) {
                let res = self.result.remove(&current);
                assert!(res.is_some());
            }
        }
        (|| {
            if let Some(ref mut group) = self.result.get_mut(&input) {
                group.start(count);
                return;
            }
            let res = self.result.insert(input.clone(), Group::new(count));
            assert!(res.is_none());
        })();
        self.current = Some(input);
    }

    /// Ends a current group.
    fn end(&mut self) {
        if let Some(ref current) = self.current {
            if self
                .result
                .get_mut(current)
                .unwrap()
                .end(self.counter.get())
            {
                let result = self.result.remove(current);
                assert!(result.is_some());
            }
        } else {
            return;
        }
        self.current = None;
    }
}

/// Constant which is used to represent undefined bound of range.
const UNDEFINED: usize = ::std::usize::MAX;

impl Group {
    fn new(count: (usize, usize, usize)) -> Self {
        let mut ret = Group {
            points: Vec::with_capacity(1),
            lines: Vec::with_capacity(1),
            polygons: Vec::with_capacity(1),
        };
        ret.start(count);
        ret
    }

    fn start(&mut self, count: (usize, usize, usize)) {
        self.points.push(Range {
            start: count.0,
            end: UNDEFINED,
        });
        self.lines.push(Range {
            start: count.1,
            end: UNDEFINED,
        });
        self.polygons.push(Range {
            start: count.2,
            end: UNDEFINED,
        })
    }

    /// Closes group, return true if self is empty
    fn end(&mut self, count: (usize, usize, usize)) -> bool {
        end(&mut self.points, count.0);
        end(&mut self.lines, count.1);
        end(&mut self.polygons, count.2);

        fn end(vec: &mut Vec<Range>, end: usize) {
            let last = vec.len() - 1;
            assert_eq!(vec[last].end, UNDEFINED);
            if vec[last].start != end {
                vec[last].end = end;
            } else {
                vec.pop();
            }
        }

        self.points.is_empty() && self.lines.is_empty() && self.polygons.is_empty()
    }
}

/// Custom trait to interface `HashMap` and `VecMap`.
trait Map<K: Key, V> {
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
        self.get_mut(*k)
    }
    fn remove(&mut self, k: &usize) -> Option<V> {
        self.remove(*k)
    }
}

/// A trait which should be implemented by a type passed into `Key` of `Map`.
trait Key: Eq {}

impl Key for String {}
impl Key for usize {}

/// Low-level Rust binding for `.obj` format.
pub struct RawObj {
    /// Name of the object.
    pub name: Option<String>,
    /// `.mtl` files which required by this object.
    pub material_libraries: Vec<String>,

    /// Position vectors of each vertex.
    pub positions: Vec<(f32, f32, f32, f32)>,
    /// Texture coordinates of each vertex.
    pub tex_coords: Vec<(f32, f32, f32)>,
    /// Normal vectors of each vertex.
    pub normals: Vec<(f32, f32, f32)>,
    /// Parametric vertices.
    pub param_vertices: Vec<(f32, f32, f32)>,

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
    pub merging_groups: VecMap<Group>,
}

/// The `Point` type which stores the index of the position vector.
pub type Point = usize;

/// The `Line` type.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Line {
    /// A series of line segments which contain only the position data of each vertex
    P(Vec<usize>),
    /// A series of line segments which contain both position and texture coordinate
    /// data of each vertex
    PT(Vec<(usize, usize)>),
}

/// The `Polygon` type.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Polygon {
    /// A polygon which contains only the position data of each vertex.
    P(Vec<usize>),
    /// A polygon which contains both position and texture coordinate data of each vertex.
    PT(Vec<(usize, usize)>),
    /// A polygon which contains both position and normal data of each vertex.
    PN(Vec<(usize, usize)>),
    /// A polygon which contains all position, texture coordinate and normal data of each vertex.
    PTN(Vec<(usize, usize, usize)>),
}

/// A group which contains ranges of points, lines and polygons
#[derive(Clone, Debug)]
pub struct Group {
    /// Multiple range of points
    pub points: Vec<Range>,
    /// Multiple range of lines
    pub lines: Vec<Range>,
    /// Multiple range of polygons
    pub polygons: Vec<Range>,
}

/// A struct which represent `[start, end)` range.
#[derive(Copy, PartialEq, Eq, Clone, Debug)]
pub struct Range {
    /// The lower bound of the range (inclusive).
    pub start: usize,
    /// The upper bound of the range (exclusive).
    pub end: usize,
}
