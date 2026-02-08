use glium::Program;
use glium::backend::Facade;
use glium::backend::glutin::SimpleWindowBuilder;
use glium::uniform;
use glium::winit::event::{Event, WindowEvent};
use glium::winit::event_loop::{ControlFlow, EventLoop};
use obj::{Obj, load_obj};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;

    // building the display, ie. the main object
    let (_window, display) = SimpleWindowBuilder::new()
        .with_inner_size(500, 400)
        .with_title("obj-rs")
        .build(&event_loop);

    let input = include_bytes!("../../obj-rs/tests/fixtures/normal-cone.obj");
    let obj: Obj = load_obj(&input[..])?;

    let vb = obj.vertex_buffer(display.get_context())?;
    let ib = obj.index_buffer(display.get_context())?;

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
    )?;

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
    #[allow(deprecated, reason = "TODO: Migrate this into `.run_app()` later")]
    event_loop.run(move |event, active_event_loop| {
        active_event_loop.set_control_flow(ControlFlow::Wait);

        match event {
            Event::LoopExiting => {}
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => active_event_loop.exit(),
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                // draw
                use glium::Surface;

                let mut target = display.draw();
                target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
                target.draw(&vb, &ib, &program, &uniforms, &params).unwrap();
                target.finish().unwrap();
            }
            _ => {}
        }
    })?;

    Ok(())
}
