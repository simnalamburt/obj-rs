//! Type the command below to your console to run this example.
//!
//!     cargo run --example glium --features glium-support

#![feature(fs, io, plugin, std_misc, old_io)]
#![plugin(glium_macros)]

extern crate glutin;
#[macro_use] extern crate glium;
extern crate obj;

use std::default::Default;
use std::old_io::timer;
use std::time::Duration;
use std::fs::File;
use std::io::BufReader;

use obj::*;

fn main() {
    use glium::DisplayBuild;

    // building the display, ie. the main object
    let display = glutin::WindowBuilder::new()
        .with_dimensions(500, 500)
        .with_depth_buffer(32)
        .build_glium().unwrap();

    let input = BufReader::new(File::open("tests/fixtures/normal-cone.obj").unwrap());
    let obj: Obj = load_obj(input).unwrap();

    let vertex_buffer = glium::VertexBuffer::new(&display, obj.vertices);
    let index_buffer = glium::IndexBuffer::new(&display, glium::index::TrianglesList(obj.indices));

    let program = glium::Program::from_source(&display, r#"
        #version 410

        uniform mat4 matrix;

        in vec3 position;
        in vec3 normal;

        smooth out vec3 _normal;

        void main() {
            gl_Position = matrix * vec4(position, 1.0);
            _normal = normalize(normal);
        }
    "#, r#"
        #version 410

        uniform vec3 light;

        smooth in vec3 _normal;
        out vec4 result;

        void main() {
            result = vec4(clamp(dot(_normal, -light), 0.0f, 1.0f) * vec3(1.0f, 0.93f, 0.56f), 1.0f);
        }
    "#, None).unwrap();

    // drawing a frame
    let uniforms = uniform! {
        matrix: [
            [ 2.356724, 0.000000, -0.217148, -0.216930],
            [ 0.000000, 2.414214,  0.000000,  0.000000],
            [-0.523716, 0.000000, -0.977164, -0.976187],
            [ 0.000000, 0.000000,  9.128673,  9.219544]
        ],
        light: (-1.0, -1.0, -1.0)
    };

    let params = glium::DrawParameters {
        depth_write: true,
        depth_test: glium::DepthTest::IfLess,
        .. Default::default()
    };

    // the main loop
    // each cycle will draw once
    'main: loop {
        use glium::Surface;

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &params).unwrap();
        target.finish();

        // sleeping for some time in order not to use up too much CPU
        timer::sleep(Duration::milliseconds(17));

        // polling and handling the events received by the window
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => break 'main,
                _ => ()
            }
        }
    }
}
