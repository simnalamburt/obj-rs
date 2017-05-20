// Copyright 2014-2017 Hyeon Kim
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Parses `.mtl` format which stores material data

use std::io::prelude::*;
use std::collections::HashMap;
use std::mem::replace;

use error::ObjResult;
use raw::lexer::lex;

/// Parses &[&str] into &[f32].
macro_rules! f {
    ($args:expr) => (
        &{
            let mut ret = Vec::<f32>::new();
            ret.reserve($args.len());
            for arg in $args {
                ret.push(try!(arg.parse()))
            }
            ret
        }[..]
    )
}

/// Parses a wavefront `.mtl` format *(incomplete)*
pub fn parse_mtl<T: BufRead>(input: T) -> ObjResult<RawMtl> {
    let mut materials = HashMap::new();

    // Properties of the material being currently parsed
    let mut name: Option<String> = None;
    let mut mat: Material = Material::default();

    try!(lex(input, |stmt, args| {
        match stmt {
            // Material name statement
            "newmtl" => {
                // Finish whatever material we were parsing
                if let Some(name) = name.take() {
                    materials.insert(name, replace(&mut mat, Material::default()));
                }

                match args.len() {
                    1 => name = Some(args[0].to_owned()),
                    _ => error!(WrongNumberOfArguments, "Expected exactly 1 argument")
                }
            }

            // Material color and illumination statements
            "Ka" => mat.ambient = Some(try!(parse_color(args))),
            "Kd" => mat.diffuse = Some(try!(parse_color(args))),
            "Ks" => mat.specular = Some(try!(parse_color(args))),
            "Ke" => mat.emissive = Some(try!(parse_color(args))),
            "Km" => unimplemented!(),
            "Tf" => mat.transmission_filter = Some(try!(parse_color(args))),
            "Ns" => {
                match args.len() {
                    1 => mat.specular_exponent = Some(try!(args[0].parse())),
                    _ => error!(WrongNumberOfArguments, "Expected exactly 1 argument")
                }
            }
            "Ni" => {
                match args.len() {
                    1 => mat.optical_density = Some(try!(args[0].parse())),
                    _ => error!(WrongNumberOfArguments, "Expected exactly 1 argument")
                }
            }
            "illum" => {
                match args.len() {
                    1 => mat.illumination_model = Some(try!(args[0].parse())),
                    _ => error!(WrongNumberOfArguments, "Expected exactly 1 argument")
                }
            }
            "d" => {
                match args.len() {
                    1 => mat.dissolve = Some(try!(args[0].parse())),
                    _ => error!(WrongNumberOfArguments, "Expected exactly 1 argument")
                }
            }
            "Tr" => {
                match args.len() {
                    1 => mat.dissolve = Some(1.0 - try!(args[0].parse::<f32>())),
                    _ => error!(WrongNumberOfArguments, "Expected exactly 1 argument")
                }
            }

            // Texture map statements
            "map_Ka" => mat.ambient_map = Some(try!(parse_texture_map(args))),
            "map_Kd" => mat.diffuse_map = Some(try!(parse_texture_map(args))),
            "map_Ks" => mat.specular_map = Some(try!(parse_texture_map(args))),
            "map_Ke" => mat.emissive_map = Some(try!(parse_texture_map(args))),
            "map_d" => mat.dissolve_map = Some(try!(parse_texture_map(args))),
            "map_aat" => unimplemented!(),
            "map_refl" => unimplemented!(),
            "map_bump" | "map_Bump" | "bump" => mat.bump_map = Some(try!(parse_texture_map(args))),
            "disp" => unimplemented!(),

            // Reflection map statement
            "refl" => unimplemented!(),

            // Unexpected statement
            _ => error!(UnexpectedStatement, "Received unknown statement")
        }

        Ok(())
    }));

    // Insert the final material
    if let Some(name) = name {
        materials.insert(name, mat);
    }

    Ok(RawMtl { materials })
}

/// Parses a color from the arguments of a statement
fn parse_color(args: &[&str]) -> ObjResult<MtlColor> {
    if args.is_empty() {
        error!(WrongNumberOfArguments, "Expected at least 1 argument");
    }

    Ok(match args[0] {
        "xyz" => {
            let args = f!(&args[1..]);
            match args.len() {
                1 => MtlColor::Xyz(args[0], args[0], args[0]),
                3 => MtlColor::Xyz(args[0], args[1], args[2]),
                _ => error!(WrongNumberOfArguments, "Expected 1 or 3 color values")
            }
        }

        "spectral" => {
            match args.len() {
                2 => MtlColor::Spectral(args[1].to_owned(), 1.0),
                3 => MtlColor::Spectral(args[1].to_owned(), try!(args[2].parse())),
                _ => error!(WrongNumberOfArguments, "Expected 2 or 3 arguments")
            }
        }

        _ => {
            let args = f!(args);
            match args.len() {
                1 => MtlColor::Rgb(args[0], args[0], args[0]),
                3 => MtlColor::Rgb(args[0], args[1], args[2]),
                _ => error!(WrongNumberOfArguments, "Expected 1 or 3 color values")
            }
        }
    })
}

/// Parses a texture map specification from the arguments of a statement
fn parse_texture_map(args: &[&str]) -> ObjResult<MtlTextureMap> {
    if args.len() == 1 {
        Ok(MtlTextureMap { file: args[0].to_owned() })
    } else {
        error!(WrongNumberOfArguments, "Expected exactly 1 argument")
    }
}

/// Low-level Rust binding for `.mtl` format *(incomplete)*.
#[derive(Clone, Debug)]
pub struct RawMtl {
    /// Map from the material name to its properties
    pub materials: HashMap<String, Material>
}

/// A single material from a `.mtl` file
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Material {
    /// The ambient color, specified by `Ka`
    pub ambient: Option<MtlColor>,
    /// The diffuse color, specified by `Kd`
    pub diffuse: Option<MtlColor>,
    /// The specular color, specified by `Ks`
    pub specular: Option<MtlColor>,
    /// The emissive color, specified by `Ke`
    pub emissive: Option<MtlColor>,
    /// The transmission filter, specified by `Tf`
    pub transmission_filter: Option<MtlColor>,
    /// The illumination model to use for this material; see the `.mtl` spec for more details.
    pub illumination_model: Option<u32>,
    /// The dissolve (opacity) of the material, specified by `d`
    pub dissolve: Option<f32>,
    /// The specular exponent, specified by `Ne`
    pub specular_exponent: Option<f32>,
    /// The optical density, i.e. index of refraction, specified by `Ni`
    pub optical_density: Option<f32>,
    /// The ambient color map, specified by `map_Ka`
    pub ambient_map: Option<MtlTextureMap>,
    /// The diffuse color map, specified by `map_Kd`
    pub diffuse_map: Option<MtlTextureMap>,
    /// The specular color map, specified by `map_Ks`
    pub specular_map: Option<MtlTextureMap>,
    /// The emissive color map, specified by `map_Ke`
    pub emissive_map: Option<MtlTextureMap>,
    /// The dissolve map, specified by `map_d`
    pub dissolve_map: Option<MtlTextureMap>,
    /// The bump map (normal map), specified by `bump`
    pub bump_map: Option<MtlTextureMap>,
}

/// A color specified in a `.mtl` file
#[derive(Clone, PartialEq, Debug)]
pub enum MtlColor {
    /// Color in the RGB color space
    Rgb(f32, f32, f32),
    /// Color in the CIEXYZ color space
    Xyz(f32, f32, f32),
    /// Color using a spectral curve
    ///
    /// The first argument is the name of a `.rfl` file specifying the curve.
    /// The second argument is a multiplier for the values in the `.rfl` file.
    Spectral(String, f32)
}

/// A texture map specified in a `.mtl` file
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MtlTextureMap {
    /// The name of the texture file
    pub file: String,
    // TODO: support arguments to the texture map
}
