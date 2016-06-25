obj-rs [![crates-i]][crates-a] [![travis-i]][travis-a]
========
[Wavefront obj][obj] parser for **[Rust]**. It handles both `.obj` and `.mtl`
formats. [Documentation][docs]

```toml
[dependencies]
obj-rs = "0.4"
```
```rust
use std::fs::File;
use std::io::BufReader;
use obj::*;

let input = try!(BufReader::new(File::open("tests/fixtures/dome.obj")));
let dome: Obj = try!(load_obj(input));

// Do whatever you want
dome.vertices;
dome.indices;
```

![img]

> This sample image is pretty good illustration of current status of **obj-rs**.
**obj-rs** is currently able to load position and normal data of `obj` but not
texture & material data yet.

Glium support
--------
**obj-rs** supports [glium] out of the box.

```toml
[dependencies]
glium = "0.14"
obj-rs = { version = "0.4", features = ["glium-support"] }
```
```rust
use std::fs::File;
use std::io::BufReader;
use obj::*;

let input = BufReader::new(try!(File::open("rilakkuma.obj")));
let obj: Obj = try!(load_obj(input));

let vb = try!(obj.vertex_buffer(&display));
let ib = try!(obj.index_buffer(&display));
```

Please see the [working sample] for the further details. Use can execute it with
the command below.
```bash
cargo run --example glium --features glium-support
```

--------

[BSD 2-Clause](LICENSE.md)

[crates-i]: https://img.shields.io/crates/v/obj-rs.svg
[crates-a]: https://crates.io/crates/obj-rs
[travis-i]: https://travis-ci.org/simnalamburt/obj-rs.svg?branch=master
[travis-a]: https://travis-ci.org/simnalamburt/obj-rs
[obj]: https://en.wikipedia.org/wiki/Wavefront_.obj_file
[Rust]: http://rust-lang.org
[docs]: https://simnalamburt.github.io/obj-rs/
[img]: https://simnalamburt.github.io/obj-rs/screenshot.png
[glium]: https://github.com/tomaka/glium
[working sample]: examples/glium.rs
