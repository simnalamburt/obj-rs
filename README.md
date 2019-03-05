obj-rs [![version]][crates.io]
========
[Wavefront .obj] parser for Rust. It handles both `.obj` and `.mtl` formats.
See [Documentation] for the further details.

```toml
[dependencies]
obj-rs = "0.4"
```
```rust
use std::fs::File;
use std::io::BufReader;
use obj::*;

let input = BufReader::new(File::open("tests/fixtures/dome.obj"))?;
let mobel: Obj = load_obj(input)?;

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

CI Status
--------
Linux/macOS Build | Windows Build
:----------------:|:-------------:
[![Travis Build Status]][travis] | [![AppVeyor Build Status]][appveyor]

<br>

--------
*obj-rs* is primarily distributed under the terms of both the [Apache License
(Version 2.0)] and the [MIT license]. See [COPYRIGHT] for details.

[version]: https://badgen.net/crates/v/obj-rs
[crates.io]: https://crates.io/crates/obj-rs

[Wavefront .obj]: https://en.wikipedia.org/wiki/Wavefront_.obj_file
[Documentation]: https://docs.rs/obj-rs/
[glium]: https://github.com/tomaka/glium
[working sample]: examples/glium.rs

[Travis Build Status]: https://badgen.net/travis/simnalamburt/obj-rs/master?icon=travis
[travis]: https://travis-ci.org/simnalamburt/obj-rs
[AppVeyor Build Status]: https://badgen.net/appveyor/ci/simnalamburt/obj-rs/master?icon=appveyor
[appveyor]: https://ci.appveyor.com/project/simnalamburt/obj-rs/branch/master

[MIT license]: LICENSE-MIT
[Apache License (Version 2.0)]: LICENSE-APACHE
[COPYRIGHT]: COPYRIGHT
