#![feature(core, collections, io)]
#![cfg_attr(test, feature(test))]

pub use obj::obj;
pub use mtl::mtl;

#[macro_use]
pub mod error;

mod lex;
pub mod obj;
pub mod mtl;
