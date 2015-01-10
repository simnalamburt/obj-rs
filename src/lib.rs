#![experimental]
#![allow(unstable)]

pub use obj::obj;
pub use mtl::mtl;

#[macro_use]
pub mod error;

mod lex;
pub mod obj;
pub mod mtl;
