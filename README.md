obj-rs [![crates-i][]][crates-a] [![travis-i][]][travis-a]
========

[Wavefront obj][obj] parser for **[Rust][]**. It handles both `.obj` and `.mtl`
formats. [Documentation][docs]

```toml
[dependencies]
obj-rs = "0.4"
```
```rust
use std::fs::File;
use std::io::BufReader;
use obj::*;

let input = BufReader::new(File::open("tests/fixtures/dome.obj").unwrap());
let dome: Obj = load_obj(input).unwrap();

// Do whatever you want
dome.vertices;
dome.indices;
```

![img][]

> This sample image is pretty good illustration of current status of **obj-rs**.
**obj-rs** is currently able to load position and normal data of `obj` but not
texture & material data yet.

Glium support
--------

**obj-rs** supports [glium][] out of the box. See [example][] for further details.

```toml
[dependencies]
glium = "0.8"
obj-rs = { version = "0.4", features = ["glium-support"] }
```
```rust
use std::fs::File;
use std::io::BufReader;
use obj::*;

let input = BufReader::new(File::open("rilakkuma.obj").unwrap());
let obj: Obj = load_obj(input).unwrap();

let vb = obj.vertex_buffer(&display).unwrap();
let ib = obj.index_buffer(&display).unwrap();
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
[example]: examples/glium/src/main.rs
