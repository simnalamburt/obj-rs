#![feature(fs, io)]

extern crate obj;

use std::fs::File;
use std::io::BufReader;

macro_rules! v {
    ($($x:expr),*) => ( ($(stringify!($x).parse::<f32>().unwrap()),*) )
}

#[test]
fn dome() {
    let file = File::open("tests/fixtures/dome.obj").unwrap();
    let obj = obj::load_obj(BufReader::new(file)).unwrap();

    assert_eq!(obj.name, Some("Dome".to_string()));
    assert_eq!(obj.vertices[0], v!(-0.382683, 0.923880, 0.000000));
    assert_eq!(obj.vertices[1], v!(-0.707107, 0.707107, 0.000000));
    assert_eq!(obj.indices[0], 3);
    assert_eq!(obj.indices[1], 2);
    assert_eq!(obj.indices[2], 6);
}
