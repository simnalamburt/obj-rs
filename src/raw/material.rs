//! Parses `.mtl` format which stores material data

use std::io::prelude::*;
use error::ObjResult;
use raw::lexer::lex;

/// Parses a wavefront `.obj` format *(incomplete)*
pub fn parse_mtl<T: BufRead>(input: T) -> ObjResult<Mtl> {
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

    Ok(Mtl)
}

/// Low-level Rust binding for `.mtl` format *(incomplete)*.
#[derive(Copy)]
pub struct Mtl;
