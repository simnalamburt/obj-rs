//! Parses `.obj` format which stores 3D mesh data

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;
use std::io::BufRead;
use std::mem;

use crate::error::{make_error, ObjResult};
use crate::raw::lexer::lex;
use crate::raw::util::parse_args;

macro_rules! parse_args {
    {
        $first:expr, $rest:expr,
        $($pat:pat => $type:ident::$name:ident[$exp:expr]),*,
        ! => $error:expr
    } => (
        match split_vertex_group($first)[..] {
            $($pat => $type::$name({
                let mut points = vec![$exp];
                for param in $rest {
                    match split_vertex_group(param)[..] {
                        $pat => points.push($exp),
                        _ => $error
                    }
                }
                points
            }),)*
            _ => $error
        }
    )
}

// Helper function for handling the indexes.
//
// If total size of the collection is 5:
//
// - ["1", "2", "3", "4", "5"] → [0, 1, 2, 3, 4]
// - ["-5", "-4", "-3", "-2", "-1"] → [0, 1, 2, 3, 4]
// - ["0"] → Error
// - ["6"] → Error
// - ["-6"] → Error
//
// If the index is < 0, then it represents an offset from the end of
// the current list. So -1 is the most recently added vertex.
//
// If the index is > 0 then it's simply the position in the list such
// that 1 is the first vertex.
fn try_index<T>(collection: &[T], input: &str) -> ObjResult<usize> {
    use crate::error::{LoadError, LoadErrorKind, ObjError};

    let len: isize = collection.len().try_into().map_err(|_| {
        ObjError::Load(LoadError::new(
            LoadErrorKind::IndexOutOfRange,
            "Too many items in collection",
        ))
    })?;

    // Should be [-len, -1] ∪ [1, len]
    let index: isize = input.parse()?;

    let ret = if index < -len {
        // (∞, -len)
        make_error!(IndexOutOfRange, "Too small index value");
    } else if index < 0 {
        // [-len, 0)
        len + index
    } else if index == 0 {
        // {0}
        make_error!(IndexOutOfRange, "Index value shouldn't be zero");
    } else if index <= len {
        // (0, len]
        index - 1
    } else {
        // (len, ∞)
        make_error!(IndexOutOfRange, "Too big index value");
    };

    Ok(ret as usize)
}

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
    let mut group_builder = GroupBuilder::with_default(&counter, String::from("default"));
    let mut mesh_builder = GroupBuilder::with_default(&counter, String::new());
    let mut smoothing_builder = GroupBuilder::new(&counter);
    let mut merging_builder = GroupBuilder::new(&counter);

    lex(input, |stmt, args: &[&str]| {
        match stmt {
            // Vertex data
            "v" => positions.push(match parse_args(args)?[..] {
                [x, y, z, w] => (x, y, z, w),
                [x, y, z] => (x, y, z, 1.0),
                _ => make_error!(WrongNumberOfArguments, "Expected 3 or 4 arguments"),
            }),
            "vt" => tex_coords.push(match parse_args(args)?[..] {
                [u, v, w] => (u, v, w),
                [u, v] => (u, v, 0.0),
                [u] => (u, 0.0, 0.0),
                _ => make_error!(WrongNumberOfArguments, "Expected 1, 2 or 3 arguments"),
            }),
            "vn" => normals.push(match parse_args(args)?[..] {
                [x, y, z] => (x, y, z),
                _ => make_error!(WrongNumberOfArguments, "Expected 3 arguments"),
            }),
            "vp" => param_vertices.push(match parse_args(args)?[..] {
                [u, v, w] => (u, v, w),
                [u, v] => (u, v, 1.0),
                [u] => (u, 0.0, 1.0),
                _ => make_error!(WrongNumberOfArguments, "Expected 1, 2 or 3 arguments"),
            }),

            // Free-form curve / surface attributes
            // TODO: Use rational information
            "cstype" => {
                let geometry = match args {
                    ["rat", ty] => *ty,
                    [ty] => *ty,
                    _ => make_error!(WrongTypeOfArguments, "Expected 'rat xxx' or 'xxx' format"),
                };

                match geometry {
                    "bmatrix" => unimplemented!(),
                    "bezier" => unimplemented!(),
                    "bspline" => unimplemented!(),
                    "cardinal" => unimplemented!(),
                    "taylor" => unimplemented!(),
                    _ => make_error!(
                        WrongTypeOfArguments,
                        "Expected one of 'bmatrix', 'bezier', 'bspline', 'cardinal' and 'taylor'"
                    ),
                }
            }
            "deg" => match parse_args(args)?[..] {
                [_deg_u, _deg_v] => unimplemented!(),
                [_deg_u] => unimplemented!(),
                _ => make_error!(WrongNumberOfArguments, "Expected 1 or 2 arguments"),
            },
            "bmat" => unimplemented!(),
            "step" => unimplemented!(),

            // Elements
            "p" => {
                for v in args {
                    let v = try_index(&positions, v)?;
                    points.push(v);
                }
            }
            "l" => match args {
                [] => make_error!(WrongNumberOfArguments, "Expected at least 2 arguments"),
                [first, rest @ ..] => {
                    if args.len() < 2 {
                        make_error!(WrongNumberOfArguments, "Expected at least 2 arguments")
                    }

                    let line = parse_args! {
                        first, rest,
                        [p] => Line::P[try_index(&positions, p)?],
                        [p, t] => Line::PT[(try_index(&positions, p)?, try_index(&tex_coords, t)?)],
                        ! => make_error!(WrongTypeOfArguments, "Unexpected vertex format, expected `#`, or `#/#`")
                    };

                    lines.push(line);
                }
            },
            "fo" | "f" => match args {
                [] => make_error!(WrongNumberOfArguments, "Expected at least 3 arguments"),
                [first, rest @ ..] => {
                    if args.len() < 3 {
                        make_error!(WrongNumberOfArguments, "Expected at least 3 arguments")
                    }

                    let polygon = parse_args! {
                        first, rest,
                        [p] => Polygon::P[try_index(&positions, p)?],
                        [p, t] => Polygon::PT[(try_index(&positions, p)?, try_index(&tex_coords, t)?)],
                        [p, "", n] => Polygon::PN[(try_index(&positions, p)?, try_index(&normals, n)?)],
                        [p, t, n] => Polygon::PTN[(try_index(&positions, p)?, try_index(&tex_coords, t)?, try_index(&normals, n)?)],
                        ! => make_error!(WrongTypeOfArguments, "Unexpected vertex format, expected `#`, `#/#`, `#//#`, or `#/#/#`")
                    };

                    polygons.push(polygon);
                }
            },
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
                [name] => group_builder.start((*name).to_string()),
                _ => make_error!(
                    WrongNumberOfArguments,
                    "Expected group name parameter, but nothing has been supplied"
                ),
            },
            "s" => match args {
                ["off"] | ["0"] => smoothing_builder.end(),
                [param] => smoothing_builder.start(param.parse()?),
                _ => make_error!(WrongNumberOfArguments, "Expected only 1 argument"),
            },
            "mg" => match args {
                ["off"] | ["0"] => merging_builder.end(),
                [param] => merging_builder.start(param.parse()?),
                _ => make_error!(WrongNumberOfArguments, "Expected only 1 argument"),
            },
            "o" => {
                name = match args {
                    [] => None,
                    _ => Some(args.join(" ")),
                    // TODO: "name a  b" will be parsed as "name a b"
                }
            }

            // Display / render attributes
            "bevel" => unimplemented!(),
            "c_interp" => unimplemented!(),
            "d_interp" => unimplemented!(),
            "lod" => unimplemented!(),
            "usemtl" => match args {
                [material] => mesh_builder.start((*material).to_string()),
                _ => make_error!(WrongNumberOfArguments, "Expected only 1 argument"),
            },
            "mtllib" => {
                material_libraries.reserve(args.len());
                for &path in args {
                    material_libraries.push(path.to_string());
                }
            }
            "shadow_obj" => unimplemented!(),
            "trace_obj" => unimplemented!(),
            "ctech" => unimplemented!(),
            "stech" => unimplemented!(),

            // Unexpected statement
            _ => make_error!(UnexpectedStatement, "Received unknown statement"),
        }

        Ok(())
    })?;

    group_builder.end();
    mesh_builder.end();
    smoothing_builder.end();
    merging_builder.end();

    Ok(RawObj {
        name,
        material_libraries,

        positions,
        tex_coords,
        normals,
        param_vertices,

        points,
        lines,
        polygons,

        groups: group_builder.result,
        meshes: mesh_builder.result,
        smoothing_groups: smoothing_builder.result,
        merging_groups: merging_builder.result,
    })
}

/// Splits a string with '/'.
fn split_vertex_group(input: &str) -> Vec<&str> {
    input.split('/').collect()
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
            points,
            lines,
            polygons,
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
}

/// Helper for creating `groups`, `meshes`, `smoothing_groups` and `merging_groups` member of
/// `Obj`.
struct GroupBuilder<'a, K> {
    counter: &'a Counter,
    /// `Some(K)` if some group has been started, `None` otherwise.
    current: Option<K>,
    result: HashMap<K, Group>,
}

impl<'a, K> GroupBuilder<'a, K>
where
    K: Clone + Eq + Hash,
{
    fn new(counter: &'a Counter) -> Self {
        GroupBuilder {
            counter,
            current: None,
            result: HashMap::new(),
        }
    }

    fn with_default(counter: &'a Counter, default: K) -> Self {
        let mut result = HashMap::with_capacity(1);
        result.insert(default.clone(), Group::new((0, 0, 0)));

        GroupBuilder {
            counter,
            current: Some(default),
            result,
        }
    }

    /// Starts a group whose name is `input`.
    fn start(&mut self, input: K) {
        let count = self.counter.get();

        match self.current {
            // Started same group twice, do nothing
            Some(ref current) if *current == input => return,
            // Changing group
            Some(ref mut current) => {
                // Close the past group and start a new one
                let past = mem::replace(current, input.clone());
                match self.result.entry(past) {
                    Entry::Vacant(_) => unreachable!(),
                    Entry::Occupied(mut e) => {
                        let was_empty_group = e.get_mut().end(count);
                        // Remove the past group if the past group is empty
                        if was_empty_group {
                            e.remove();
                        }
                    }
                }
            }
            // Starting a new group
            None => self.current = Some(input.clone()),
        }

        self.result
            .entry(input)
            .and_modify(|e| e.start(count))
            .or_insert_with(|| Group::new(count));
    }

    /// Ends a current group.
    fn end(&mut self) {
        match mem::replace(&mut self.current, None) {
            // Closed group twice, do nothing
            None => {}
            // Closing a group
            Some(current) => {
                match self.result.entry(current) {
                    Entry::Vacant(_) => unreachable!(),
                    Entry::Occupied(mut e) => {
                        let count = self.counter.get();
                        let was_empty_group = e.get_mut().end(count);
                        // Remove the past group if the past group is empty
                        if was_empty_group {
                            e.remove();
                        }
                    }
                }
            }
        }
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
    pub smoothing_groups: HashMap<usize, Group>,
    /// Merging groups.
    pub merging_groups: HashMap<usize, Group>,
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
