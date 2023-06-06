obj-rs [![version]][crates.io]
========
[Wavefront .obj] parser for Rust. It handles both `.obj` and `.mtl` formats.
See [Documentation] for the further details.

```toml
[dependencies]
obj-rs = "0.7"
```
```rust
use std::fs::File;
use std::io::BufReader;
use obj::{load_obj, Obj};

let input = BufReader::new(File::open("tests/fixtures/dome.obj"))?;
let model: Obj = load_obj(input)?;

// Do whatever you want
model.vertices;
model.indices;
```

![Rendered image of cute Rilakkuma](https://simnalamburt.github.io/obj-rs/screenshot.png)

<br>

Glium support
--------
**obj-rs** supports [glium] out of the box.

```toml
[dependencies]
glium = "0.26"
obj-rs = { version = "0.6", features = ["glium"] }
```
```rust
use std::fs::File;
use std::io::BufReader;
use obj::{load_obj, Obj};

let input = BufReader::new(File::open("rilakkuma.obj")?);
let obj: Obj = load_obj(input)?;

let vb = obj.vertex_buffer(&display)?;
let ib = obj.index_buffer(&display)?;
```

Please see the [working sample] for the further details. Use can execute it with
the command below.
```bash
cargo run -p sampleapp
```

<br>

--------
*obj-rs* is primarily distributed under the terms of both the [Apache License
(Version 2.0)] and the [MIT license]. See [COPYRIGHT] for details.

[version]: https://badgen.net/crates/v/obj-rs
[crates.io]: https://crates.io/crates/obj-rs

[Wavefront .obj]: https://en.wikipedia.org/wiki/Wavefront_.obj_file
[Documentation]: https://docs.rs/obj-rs/
[glium]: https://github.com/tomaka/glium
[working sample]: sampleapp/src/main.rs

[MIT license]: LICENSE-MIT
[Apache License (Version 2.0)]: LICENSE-APACHE
[COPYRIGHT]: COPYRIGHT
