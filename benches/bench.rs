// Copyright 2014-2017 Hyeon Kim
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(test)]

extern crate obj;
extern crate test;

use test::Bencher;

use obj::raw::parse_obj;

fn load_file(filename: &str) -> Vec<u8> {
    use std::path::Path;
    use std::fs::File;
    use std::io::Read;

    let path = Path::new("tests").join("fixtures").join(filename);
    let mut file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => panic!("Failed to open \"{}\". \x1b[31m{}\x1b[0m", path.to_string_lossy(), e)
    };

    let mut data = Vec::new();
    match file.read_to_end(&mut data) {
        Err(e) => panic!("Failed to read \"{}\". \x1b[31m{}\x1b[0m", path.to_string_lossy(), e),
        Ok(_) => ()
    }

    data
}

#[bench]
fn sponza(b: &mut Bencher) {
    let data = load_file("sponza.obj");
    let mut data = std::io::Cursor::new(&data[..]);
    b.iter(|| {
        parse_obj(&mut data).unwrap()
    });
}
