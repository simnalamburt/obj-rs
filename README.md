obj-rs [![Travis Build Status]][travis] [![AppVeyor Build Status]][appveyor]
========
[Wavefront obj] parser for [Rust]. It handles both `.obj` and `.mtl` formats.

- [API Documentation](https://simnalamburt.github.io/obj-rs/)
- [See `obj-rs` in crates.io](https://crates.io/crates/obj-rs)

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

<br>

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

<br>

--------
*obj-rs* is primarily distributed under the terms of both the [Apache License
(Version 2.0)] and the [MIT license]. See [COPYRIGHT] for details.

[Travis Build Status]: https://travis-ci.org/simnalamburt/obj-rs.svg?branch=master
[travis]: https://travis-ci.org/simnalamburt/obj-rs
[AppVeyor Build Status]: https://ci.appveyor.com/api/projects/status/281kjgy7oxaa120s/branch/master?svg=true
[appveyor]: https://ci.appveyor.com/project/simnalamburt/obj-rs/branch/master

[Wavefront obj]: https://en.wikipedia.org/wiki/Wavefront_.obj_file
[Rust]: http://rust-lang.org
[img]: https://simnalamburt.github.io/obj-rs/screenshot.png
[glium]: https://github.com/tomaka/glium
[working sample]: examples/glium.rs
[MIT license]: LICENSE-MIT
[Apache License (Version 2.0)]: LICENSE-APACHE
[COPYRIGHT]: COPYRIGHT
