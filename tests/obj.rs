extern crate obj;

use std::io::{BufferedReader, File};

#[test]
fn test_obj() {
    let path = Path::new("tests").join("fixtures").join("cube.obj");
    let mut input = BufferedReader::new(File::open(&path));

    // obj::obj(&mut input);
}
