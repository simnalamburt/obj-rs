#![feature(core, collections, io)]
#![cfg_attr(test, feature(test))]

pub use obj::load_obj;
pub use mtl::load_mtl;

#[macro_use]
pub mod error;

mod lex;
pub mod obj;
pub mod mtl;
