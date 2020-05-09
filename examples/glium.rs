//! Working example of *obj-rs*
//! ========
//!
//! Execute it with the command below
//!
//!     cargo run --example glium --features glium-support

#[cfg(feature = "glium-support")]
fn main() {
    use glium::glutin::dpi::LogicalSize;
    use glium::glutin::event::{Event, WindowEvent};
    use glium::glutin::event_loop::{ControlFlow, EventLoop};
    use glium::glutin::window::WindowBuilder;
    use glium::glutin::ContextBuilder;
    use glium::Program;
    use obj::*;
    use std::fs::File;
    use std::io::BufReader;

    let event_loop = EventLoop::new();

    // building the display, ie. the main object

    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(500.0, 400.0))
        .with_title("obj-rs");

    let context = ContextBuilder::new();

    let display = glium::Display::new(window, context, &event_loop).unwrap();

    let input = BufReader::new(File::open("tests/fixtures/normal-cone.obj").unwrap());
    let obj: Obj = load_obj(input).unwrap();

    let vb = obj.vertex_buffer(&display).unwrap();
    let ib = obj.index_buffer(&display).unwrap();

    let program = Program::from_source(
        &display,
        r#"
        #version 410

        uniform mat4 matrix;

        in vec3 position;
        in vec3 normal;

        smooth out vec3 _normal;

        void main() {
            gl_Position = matrix * vec4(position, 1.0);
            _normal = normalize(normal);
        }
    "#,
        r#"
        #version 410

        uniform vec3 light;

        smooth in vec3 _normal;
        out vec4 result;

        void main() {
            result = vec4(clamp(dot(_normal, -light), 0.0f, 1.0f) * vec3(1.0f, 0.93f, 0.56f), 1.0f);
        }
    "#,
        None,
    )
    .unwrap();

    // drawing a frame
    let uniforms = uniform! {
        matrix: [
            [ 2.356724, 0.000000, -0.217148, -0.216930],
            [ 0.000000, 2.414214,  0.000000,  0.000000],
            [-0.523716, 0.000000, -0.977164, -0.976187],
            [ 0.000000, 0.000000,  9.128673,  9.219544f32]
        ],
        light: (-1.0, -1.0, -1.0f32)
    };

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };

    // Main loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(_) => {
                // draw
                use glium::Surface;

                let mut target = display.draw();
                target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
                target.draw(&vb, &ib, &program, &uniforms, &params).unwrap();
                target.finish().unwrap();
            }
            _ => (),
        }
    });
}

#[cfg(not(feature = "glium-support"))]
fn main() {
    println!(
        "\n\
    Please execute it with \x1b[4m--features glium-support\x1b[24m the option

    $ \x1b[33mcargo run --example glium --features glium-support\x1b[0m\n"
    );
}
