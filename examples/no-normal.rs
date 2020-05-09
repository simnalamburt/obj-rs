//! Working example of *obj-rs*
//! ========
//!
//! Execute it with the command below
//!
//!     cargo run --example no-normal

extern crate obj;

use obj::{load_obj, Obj, ObjError, Position};
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), ObjError> {
    let input = BufReader::new(File::open("tests/fixtures/dome.obj")?);

    // NOTE: This will fail since dome.obj does not have normal data
    // let obj: Obj = load_obj(input)?;

    let obj: Obj<Position> = load_obj(input)?;
    println!("name:     {:?}", obj.name);
    println!();
    println!("vertices: {:?}", obj.vertices);
    println!();
    println!("indices:  {:?}", obj.indices);
    Ok(())
}
