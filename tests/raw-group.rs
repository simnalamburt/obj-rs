#![feature(fs, io, old_io)]

extern crate obj;

use std::fs::File;
use std::io::BufReader;
use obj::raw::*;
use obj::raw::object::Range;

macro_rules! test {
    ($($lhs:expr, $rhs:expr)*) => ({
        $(eq!($lhs, $rhs);)*
    });
}

macro_rules! eq {
    ($lhs:expr, $rhs:expr) => (eq!($lhs, $rhs, stringify!($lhs)));

    ($lhs:expr, $rhs:expr, $exp:expr) => ({
        let left = &($lhs);
        let right = &($rhs);

        if !((*left == *right) && (*right == *left)) {
            stderr!("");
            stderr!("{w}{}{c} should be {o}{:?}{c}, but it was {o}{:?}{c}. See {b}{}:{}{c}.",
                    $exp, *right, *left, line!(), column!(),
                    w="\x1b[97m", b="\x1b[34m", o="\x1b[33m", c="\x1b[0m");
            stderr!("");
            panic!($exp);
        }
    });
}

macro_rules! stderr {
    ($($arg:tt)*) => ({
        use std::old_io::stdio::stderr;

        writeln!(&mut stderr(), $($arg)*).unwrap()
    })
}

#[test]
fn dup_groupnames() {
    let input = BufReader::new(File::open("tests/fixtures/group.obj").unwrap());
    let raw = match parse_obj(input) {
        Ok(raw) => raw,
        Err(e) => panic!(e)
    };

    test! {
        raw.smoothing_groups.len(),                 2
        raw.smoothing_groups[1].points.len(),       0
        raw.smoothing_groups[1].lines.len(),        0
        raw.smoothing_groups[1].polygons.len(),     2
        raw.smoothing_groups[1].polygons[0],        Range { start: 0, end: 3 }
        raw.smoothing_groups[1].polygons[1],        Range { start: 6, end: 9 }
        raw.smoothing_groups[2].points.len(),       0
        raw.smoothing_groups[2].lines.len(),        0
        raw.smoothing_groups[2].polygons.len(),     1
        raw.smoothing_groups[2].polygons[0],        Range { start: 3, end: 6 }
    }
}
