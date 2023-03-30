//! Parses `.mtl` format which stores material data

use std::collections::HashMap;
use std::io::BufRead;
use std::mem::take;

use crate::error::{make_error, ObjResult};
use crate::raw::lexer::lex;
use crate::raw::util::parse_args;

/// Parses a wavefront `.mtl` format *(incomplete)*
pub fn parse_mtl<T: BufRead>(input: T) -> ObjResult<RawMtl> {
    let mut materials = HashMap::new();

    // Properties of the material being currently parsed
    let mut name: Option<String> = None;
    let mut mat: Material = Material::default();

    lex(input, |stmt, args| {
        match stmt {
            // Material name statement
            "newmtl" => {
                // Finish whatever material we were parsing
                if let Some(name) = name.take() {
                    materials.insert(name, take(&mut mat));
                }

                match args {
                    [arg] => name = Some((*arg).to_string()),
                    _ => make_error!(WrongNumberOfArguments, "Expected exactly 1 argument"),
                }
            }

            // Material color and illumination statements
            "Ka" => mat.ambient = Some(parse_color(args)?),
            "Kd" => mat.diffuse = Some(parse_color(args)?),
            "Ks" => mat.specular = Some(parse_color(args)?),
            "Ke" => mat.emissive = Some(parse_color(args)?),
            "Km" => unimplemented!(),
            "Tf" => mat.transmission_filter = Some(parse_color(args)?),
            "Ns" => match args {
                [arg] => mat.specular_exponent = Some(arg.parse()?),
                _ => make_error!(WrongNumberOfArguments, "Expected exactly 1 argument"),
            },
            "Ni" => match args {
                [arg] => mat.optical_density = Some(arg.parse()?),
                _ => make_error!(WrongNumberOfArguments, "Expected exactly 1 argument"),
            },
            "illum" => match args {
                [arg] => mat.illumination_model = Some(arg.parse()?),
                _ => make_error!(WrongNumberOfArguments, "Expected exactly 1 argument"),
            },
            "d" => match args {
                [arg] => mat.dissolve = Some(arg.parse()?),
                _ => make_error!(WrongNumberOfArguments, "Expected exactly 1 argument"),
            },
            "Tr" => match args {
                [arg] => mat.dissolve = Some(1.0 - arg.parse::<f32>()?),
                _ => make_error!(WrongNumberOfArguments, "Expected exactly 1 argument"),
            },

            // Texture map statements
            "map_Ka" => mat.ambient_map = Some(parse_texture_map(args)?),
            "map_Kd" => mat.diffuse_map = Some(parse_texture_map(args)?),
            "map_Ks" => mat.specular_map = Some(parse_texture_map(args)?),
            "map_Ke" => mat.emissive_map = Some(parse_texture_map(args)?),
            "map_d" => mat.dissolve_map = Some(parse_texture_map(args)?),
            "map_aat" => unimplemented!(),
            "map_refl" => unimplemented!(),
            "map_bump" | "map_Bump" | "bump" => mat.bump_map = Some(parse_texture_map(args)?),
            "disp" => unimplemented!(),

            // Reflection map statement
            "refl" => unimplemented!(),

            // Unexpected statement
            _ => make_error!(UnexpectedStatement, "Received unknown statement"),
        }

        Ok(())
    })?;

    // Insert the final material
    if let Some(name) = name {
        materials.insert(name, mat);
    }

    Ok(RawMtl { materials })
}

/// Parses a color from the arguments of a statement
fn parse_color(args: &[&str]) -> ObjResult<MtlColor> {
    if args.is_empty() {
        make_error!(WrongNumberOfArguments, "Expected at least 1 argument");
    }

    Ok(match args[0] {
        "xyz" => match parse_args(&args[1..])?[..] {
            [x] => MtlColor::Xyz(x, x, x),
            [x, y, z] => MtlColor::Xyz(x, y, z),
            _ => make_error!(WrongNumberOfArguments, "Expected 1 or 3 color values"),
        },

        "spectral" => match args[1..] {
            [name] => MtlColor::Spectral(name.to_string(), 1.0),
            [name, multiplier] => MtlColor::Spectral(name.to_string(), multiplier.parse()?),
            _ => make_error!(WrongNumberOfArguments, "Expected 1 or 2 arguments"),
        },

        _ => match parse_args(args)?[..] {
            [r] => MtlColor::Rgb(r, r, r),
            [r, g, b] => MtlColor::Rgb(r, g, b),
            _ => make_error!(WrongNumberOfArguments, "Expected 1 or 3 color values"),
        },
    })
}

/// Parses a texture map specification from the arguments of a statement
fn parse_texture_map(args: &[&str]) -> ObjResult<MtlTextureMap> {
    match args {
        [file] => Ok(MtlTextureMap {
            file: (*file).to_string(),
        }),
        _ => make_error!(WrongNumberOfArguments, "Expected exactly 1 argument"),
    }
}

/// Low-level Rust binding for `.mtl` format *(incomplete)*.
#[derive(Clone, Debug)]
pub struct RawMtl {
    /// Map from the material name to its properties
    pub materials: HashMap<String, Material>,
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
    Spectral(String, f32),
}

/// A texture map specified in a `.mtl` file
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MtlTextureMap {
    /// The name of the texture file
    pub file: String,
    // TODO: support arguments to the texture map
}
