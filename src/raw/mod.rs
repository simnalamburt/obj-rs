//! Provides low-level API for Wavefront OBJ format.

mod lexer;
pub mod object;
pub mod material;

pub use self::object::{parse_obj, RawObj};
pub use self::material::{parse_mtl, RawMtl};
