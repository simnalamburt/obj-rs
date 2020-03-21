// Copyright 2014-2017 Hyeon Kim
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Provides low-level API for Wavefront OBJ format.

mod lexer;
pub mod material;
pub mod object;

pub use self::material::{parse_mtl, RawMtl};
pub use self::object::{parse_obj, RawObj};
