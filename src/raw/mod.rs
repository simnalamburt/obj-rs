//! Provides low-level API for Wavefront OBJ format.

mod lexer;
pub mod material;
pub mod object;

pub use self::material::{parse_mtl, RawMtl};
pub use self::object::{parse_obj, RawObj};
