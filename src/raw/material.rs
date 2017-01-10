// Copyright 2014-2017 Hyeon Kim
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Parses `.mtl` format which stores material data

use std::io::prelude::*;
use error::ObjResult;
use raw::lexer::lex;

/// Parses a wavefront `.mtl` format *(incomplete)*
pub fn parse_mtl<T: BufRead>(input: T) -> ObjResult<RawMtl> {
    try!(lex(input, |stmt, _| {
        match stmt {
            // Material name statement
            "newmtl" => unimplemented!(),

            // Material color and illumination statements
            "Ka" => unimplemented!(),
            "Kd" => unimplemented!(),
            "Ks" => unimplemented!(),
            "Ke" => unimplemented!(),
            "Km" => unimplemented!(),
            "Ns" => unimplemented!(),
            "Ni" => unimplemented!(),
            "Tr" => unimplemented!(),
            "Tf" => unimplemented!(),
            "illum" => unimplemented!(),
            "d" => unimplemented!(),

            // Texture map statements
            "map_Ka" => unimplemented!(),
            "map_Kd" => unimplemented!(),
            "map_Ks" => unimplemented!(),
            "map_d" => unimplemented!(),
            "map_aat" => unimplemented!(),
            "map_refl" => unimplemented!(),
            "map_bump" | "map_Bump" | "bump" => unimplemented!(),
            "disp" => unimplemented!(),

            // Reflection map statement
            "refl" => unimplemented!(),

            // Unexpected statement
            _ => error!(UnexpectedStatement, "Received unknown statement")
        }

        Ok(())
    }));

    Ok(RawMtl)
}

/// Low-level Rust binding for `.mtl` format *(incomplete)*.
#[derive(Clone, Copy)]
pub struct RawMtl;
