#![feature(fs, io)]

extern crate obj;

use std::fs::File;
use std::io::BufReader;

#[test]
fn cube() {
    let file = File::open("tests/fixtures/cube.obj").unwrap();
    let obj = obj::load_obj(BufReader::new(file)).unwrap();

    assert_eq!(obj.name, "Cube");
}
