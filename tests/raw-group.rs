#![feature(fs, io)]

extern crate obj;

use std::fs::File;
use std::io::BufReader;

#[test]
fn dup_groupnames() {
    let file = File::open("tests/fixtures/dup_groupnames.obj").unwrap();
    let result = obj::raw::parse_obj(BufReader::new(file));
    assert!(result.is_ok());

    let obj = result.unwrap();
    for i in obj.smoothing_groups.iter() {
        println!("{:?}", i);
    }
}
