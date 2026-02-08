//! Provides low-level API for Wavefront OBJ format.

mod lexer;
pub mod material;
pub mod object;
mod util;

pub use self::material::{RawMtl, parse_mtl};
pub use self::object::{RawObj, parse_obj};
