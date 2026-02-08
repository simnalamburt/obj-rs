use obj::{Obj, ObjResult, Position, TexturedVertex, Vertex, load_obj};
use std::fs::File;
use std::io::{BufReader, Error};

fn fixture(name: &str) -> Result<BufReader<File>, Error> {
    let file = File::open(format!("tests/fixtures/{}", name))?;
    Ok(BufReader::new(file))
}

#[test]
fn normal_cone() -> ObjResult<()> {
    let obj: Obj = load_obj(fixture("normal-cone.obj")?)?;

    assert_eq!(obj.name, Some("Cone".to_string()));
    assert_eq!(obj.vertices.len(), 96);
    assert_eq!(
        obj.vertices[0],
        Vertex {
            position: [-0.382682, -1.000000, -0.923880],
            normal: [-0.259887, 0.445488, -0.856737],
        }
    );
    assert_eq!(
        obj.vertices[1],
        Vertex {
            position: [0.000000, 1.000000, 0.000000],
            normal: [-0.259887, 0.445488, -0.856737],
        }
    );
    assert_eq!(obj.indices.len(), 96);
    assert_eq!(obj.indices[0], 0);
    assert_eq!(obj.indices[1], 1);
    assert_eq!(obj.indices[2], 2);

    Ok(())
}

#[test]
fn dome() -> ObjResult<()> {
    let obj: Obj<Position> = load_obj(fixture("dome.obj")?)?;

    assert_eq!(obj.name, Some("Dome".to_string()));
    assert_eq!(
        obj.vertices[0],
        Position {
            position: [-0.382683, 0.923880, 0.000000]
        }
    );
    assert_eq!(
        obj.vertices[1],
        Position {
            position: [-0.707107, 0.707107, 0.000000]
        }
    );
    assert_eq!(obj.indices[0], 3);
    assert_eq!(obj.indices[1], 2);
    assert_eq!(obj.indices[2], 6);

    Ok(())
}

#[test]
fn textured_cube() -> ObjResult<()> {
    let obj: Obj<TexturedVertex, u32> = load_obj(fixture("textured-cube.obj")?)?;

    assert_eq!(obj.name, Some("cube".to_string()));
    assert_eq!(obj.vertices.len(), 24);
    assert_eq!(
        obj.vertices[0],
        TexturedVertex {
            position: [-0.500000, -0.500000, 0.500000],
            normal: [0.000000, 0.000000, 1.000000],
            texture: [0.000000, 0.000000, 0.000000]
        }
    );
    assert_eq!(
        obj.vertices[1],
        TexturedVertex {
            position: [0.500000, -0.500000, 0.500000],
            normal: [0.000000, 0.000000, 1.000000],
            texture: [1.000000, 0.000000, 0.000000]
        }
    );
    assert_eq!(
        obj.vertices[2],
        TexturedVertex {
            position: [-0.500000, 0.500000, 0.500000],
            normal: [0.000000, 0.000000, 1.000000],
            texture: [0.000000, 1.000000, 0.000000]
        }
    );
    assert_eq!(
        obj.vertices[3],
        TexturedVertex {
            position: [0.500000, 0.500000, 0.500000],
            normal: [0.000000, 0.000000, 1.000000],
            texture: [1.000000, 1.000000, 0.000000]
        }
    );
    assert_eq!(
        obj.vertices[4],
        TexturedVertex {
            position: [-0.500000, 0.500000, 0.500000],
            normal: [0.000000, 1.000000, 0.000000],
            texture: [0.000000, 0.000000, 0.000000]
        }
    );
    assert_eq!(
        obj.vertices[5],
        TexturedVertex {
            position: [0.500000, 0.500000, 0.500000],
            normal: [0.000000, 1.000000, 0.000000],
            texture: [1.000000, 0.000000, 0.000000]
        }
    );
    assert_eq!(obj.indices.len(), 36);
    assert_eq!(obj.indices[0], 0);
    assert_eq!(obj.indices[1], 1);
    assert_eq!(obj.indices[2], 2);
    assert_eq!(obj.indices[3], 2);
    assert_eq!(obj.indices[4], 1);
    assert_eq!(obj.indices[5], 3);
    assert_eq!(obj.indices[6], 4);
    assert_eq!(obj.indices[7], 5);

    Ok(())
}
