extern crate obj;

use obj::*;
use std::fs::File;
use std::io::BufReader;

macro_rules! fixture {
    ($name:expr) => {
        BufReader::new(File::open(concat!("tests/fixtures/", $name)).unwrap())
    };
}

#[test]
fn normal_cone() {
    let obj: Obj = load_obj(fixture!("normal-cone.obj")).unwrap();

    macro_rules! v {
        (($($p:expr),*), ($($n:expr),*)) => ({
            Vertex {
                position: [$(stringify!($p).parse::<f32>().unwrap()),*],
                normal: [$(stringify!($n).parse::<f32>().unwrap()),*],
            }
        })
    }

    assert_eq!(obj.name, Some("Cone".to_string()));
    assert_eq!(obj.vertices.len(), 96);
    assert_eq!(
        obj.vertices[0],
        v!(
            (-0.382682, -1.000000, -0.923880),
            (-0.259887, 0.445488, -0.856737)
        )
    );
    assert_eq!(
        obj.vertices[1],
        v!(
            (0.000000, 1.000000, 0.000000),
            (-0.259887, 0.445488, -0.856737)
        )
    );
    assert_eq!(obj.indices.len(), 96);
    assert_eq!(obj.indices[0], 0);
    assert_eq!(obj.indices[1], 1);
    assert_eq!(obj.indices[2], 2);
}

#[test]
fn dome() {
    let obj: Obj<Position> = load_obj(fixture!("dome.obj")).unwrap();

    macro_rules! p {
        ($($x:expr),*) => (Position { position: [$(stringify!($x).parse::<f32>().unwrap()),*] })
    }

    assert_eq!(obj.name, Some("Dome".to_string()));
    assert_eq!(obj.vertices[0], p!(-0.382683, 0.923880, 0.000000));
    assert_eq!(obj.vertices[1], p!(-0.707107, 0.707107, 0.000000));
    assert_eq!(obj.indices[0], 3);
    assert_eq!(obj.indices[1], 2);
    assert_eq!(obj.indices[2], 6);
}

#[test]
fn textured_cube() {
    let obj: Obj<TexturedVertex, u32> = load_obj(fixture!("textured-cube.obj")).unwrap();

    macro_rules! vt {
        (($($p:expr),*), ($($n:expr),*), ($($t:expr),*)) => ({
            TexturedVertex {
                position: [$(stringify!($p).parse::<f32>().unwrap()),*],
                normal: [$(stringify!($n).parse::<f32>().unwrap()),*],
                texture: [$(stringify!($t).parse::<f32>().unwrap()),*]
            }
        })
    }

    assert_eq!(obj.name, Some("cube".to_string()));
    assert_eq!(obj.vertices.len(), 24);
    dbg!(&obj.vertices);
    assert_eq!(
        obj.vertices[0],
        vt!(
            (-0.500000, -0.500000, 0.500000),
            (0.000000, 0.000000, 1.000000),
            (0.000000, 0.000000, 0.000000)
        )
    );
    assert_eq!(
        obj.vertices[1],
        vt!(
            (0.500000, -0.500000, 0.500000),
            (0.000000, 0.000000, 1.000000),
            (1.000000, 0.000000, 0.000000)
        )
    );
    assert_eq!(
        obj.vertices[2],
        vt!(
            (-0.500000, 0.500000, 0.500000),
            (0.000000, 0.000000, 1.000000),
            (0.000000, 1.000000, 0.000000)
        )
    );
    assert_eq!(
        obj.vertices[3],
        vt!(
            (0.500000, 0.500000, 0.500000),
            (0.000000, 0.000000, 1.000000),
            (1.000000, 1.000000, 0.000000)
        )
    );
    assert_eq!(
        obj.vertices[4],
        vt!(
            (-0.500000, 0.500000, 0.500000),
            (0.000000, 1.000000, 0.000000),
            (0.000000, 0.000000, 0.000000)
        )
    );
    assert_eq!(
        obj.vertices[5],
        vt!(
            (0.500000, 0.500000, 0.500000),
            (0.000000, 1.000000, 0.000000),
            (1.000000, 0.000000, 0.000000)
        )
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
}
