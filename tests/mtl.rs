extern crate obj;

use std::io::{BufferedReader, File};

#[test]
fn test_obj() {
    let path = Path::new("tests").join("fixtures").join("cube.mtl");
    let mut input = BufferedReader::new(File::open(&path));

    // obj::mtl(&mut input);
}
