#![feature(fs, io)]

extern crate obj;

use std::fs::File;
use std::io::BufReader;
use obj::*;

macro_rules! fixture {
    ($name:expr) => (BufReader::new(File::open(concat!("tests/fixtures/", $name)).unwrap()))
}

#[test]
fn dome() {
    let obj: Obj = load_obj(fixture!("dome.obj")).unwrap();

    macro_rules! v {
        ($($x:expr),*) => (Vertex { position: [$(stringify!($x).parse::<f32>().unwrap()),*] })
    }

    assert_eq!(obj.name, Some("Dome".to_string()));
    assert_eq!(obj.vertices[0], v!(-0.382683, 0.923880, 0.000000));
    assert_eq!(obj.vertices[1], v!(-0.707107, 0.707107, 0.000000));
    assert_eq!(obj.indices[0], 3);
    assert_eq!(obj.indices[1], 2);
    assert_eq!(obj.indices[2], 6);
}

#[test]
fn dome_position() {
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
