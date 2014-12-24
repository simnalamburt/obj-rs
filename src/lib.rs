#![experimental]
#![feature(macro_rules)]

pub use obj::obj;
pub use mtl::mtl;

pub mod error;
mod lex;
pub mod obj;
pub mod mtl;
