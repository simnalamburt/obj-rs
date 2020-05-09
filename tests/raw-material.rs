use obj::raw::material::{parse_mtl, Material, MtlColor, MtlTextureMap, RawMtl};
use obj::ObjResult;
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

fn fixture(filename: &str) -> ObjResult<RawMtl> {
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;

    let path = Path::new("tests").join("fixtures").join(filename);
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => panic!(
            "Failed to open \"{}\". \x1b[31m{}\x1b[0m",
            path.to_string_lossy(),
            e
        ),
    };
    let input = BufReader::new(file);

    parse_mtl(input)
}

#[test]
fn cube() -> TestResult {
    let mtl = fixture("cube.mtl")?;
    assert_eq!(mtl.materials.len(), 1);

    let mat = mtl.materials.get("Material").ok_or("not found")?;
    assert_eq!(
        mat,
        &Material {
            ambient: Some(MtlColor::Rgb(0.0, 0.0, 0.0)),
            diffuse: Some(MtlColor::Rgb(0.64, 0.64, 0.64)),
            specular: Some(MtlColor::Rgb(0.5, 0.5, 0.5)),
            dissolve: Some(1.0),
            specular_exponent: Some(96.078431),
            optical_density: Some(1.0),
            illumination_model: Some(2),
            diffuse_map: Some(MtlTextureMap {
                file: "cube-uv-num.png".to_owned()
            }),
            ..Material::default()
        }
    );

    Ok(())
}

#[test]
fn untitled() -> TestResult {
    let mtl = fixture("untitled.mtl")?;
    assert_eq!(mtl.materials.len(), 2);

    let mat = mtl.materials.get("Material").ok_or("not found")?;
    assert_eq!(
        mat,
        &Material {
            ambient: Some(MtlColor::Rgb(0.0, 0.0, 0.0)),
            diffuse: Some(MtlColor::Rgb(0.64, 0.64, 0.64)),
            specular: Some(MtlColor::Rgb(0.5, 0.5, 0.5)),
            dissolve: Some(1.0),
            specular_exponent: Some(96.078431),
            optical_density: Some(1.0),
            illumination_model: Some(2),
            ..Material::default()
        }
    );

    let mat = mtl.materials.get("None").ok_or("not found")?;
    assert_eq!(
        mat,
        &Material {
            ambient: Some(MtlColor::Rgb(0.0, 0.0, 0.0)),
            diffuse: Some(MtlColor::Rgb(0.8, 0.8, 0.8)),
            specular: Some(MtlColor::Rgb(0.8, 0.8, 0.8)),
            dissolve: Some(1.0),
            specular_exponent: Some(0.0),
            illumination_model: Some(2),
            ..Material::default()
        }
    );

    Ok(())
}
