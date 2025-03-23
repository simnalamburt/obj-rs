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
                file: "cube-uv-num.png".to_string(),
                ..MtlTextureMap::default()
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

#[test]
fn mtl_map_options() -> TestResult {
    let mtl = fixture("map_options.mtl")?;

    // material #1 -- skip

    // material #2 TextureMaps
    {
        let tex_maps = mtl.materials.get("TextureMaps").ok_or("not found")?;

        let a_map = tex_maps
            .ambient_map
            .clone()
            .ok_or("ambient_map not found")?;
        assert_eq!(a_map.file, "ambient.png");

        let d_map = tex_maps
            .diffuse_map
            .clone()
            .ok_or("diffuse_map not found")?;
        assert_eq!(d_map.file, "diffuse.png");
        assert_eq!(d_map.origin_offset, [0.1, 0.2, 0.3]);
        assert_eq!(d_map.scale, [2.0, 3.0, 4.0]);

        let s_map = tex_maps
            .specular_map
            .clone()
            .ok_or("specular_map not found")?;
        assert_eq!(s_map.file, "specular.png");
        assert_eq!(s_map.turbulence, [0.05, 0.05, 0.0]);

        let e_map = tex_maps
            .emissive_map
            .clone()
            .ok_or("emissive_map not found")?;
        assert_eq!(e_map.file, "emissive.png");
        assert_eq!(e_map.resolution, 512);

        let ds_map = tex_maps
            .dissolve_map
            .clone()
            .ok_or("dissolve_map not found")?;
        assert_eq!(ds_map.file, "dissolve.png");
        assert_eq!(ds_map.clamping, true);

        let b_map = tex_maps.bump_map.clone().ok_or("bump_map not found")?;
        assert_eq!(b_map.file, "normal.png");
        assert_eq!(b_map.bump_multiplier, 2.5);
        assert_eq!(b_map.base_gain, [0.2, 0.8]);
        assert_eq!(b_map.clamping, false);
    }

    // material #3 BlendingOptions
    {
        let blend_opts = mtl.materials.get("BlendingOptions").ok_or("not found")?;

        let d_map = blend_opts
            .diffuse_map
            .clone()
            .ok_or("dissolve_map not found")?;
        assert_eq!(d_map.file, "texture.png");
        assert_eq!(d_map.blend_u, true);
        assert_eq!(d_map.blend_v, false);
    }

    Ok(())
}
