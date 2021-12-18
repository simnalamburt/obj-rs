extern crate obj;
extern crate vulkano;

use crate::vulkano::sync::GpuFuture;
use nalgebra::{Isometry3, Perspective3, Point3, Vector3};
use obj::{load_obj, Obj};
use std::sync::Arc;
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer, CpuBufferPool},
    command_buffer::{AutoCommandBufferBuilder, DynamicState},
    descriptor::descriptor_set::PersistentDescriptorSet,
    device::{Device, DeviceExtensions, Features},
    format::Format,
    framebuffer::{Framebuffer, FramebufferAbstract, RenderPassAbstract, Subpass},
    image::{AttachmentImage, ImageUsage, SwapchainImage},
    instance::{Instance, PhysicalDevice},
    pipeline::{viewport::Viewport, GraphicsPipeline, GraphicsPipelineAbstract},
    swapchain::{
        AcquireError, ColorSpace, FullscreenExclusive, PresentMode, SurfaceTransform, Swapchain,
        SwapchainCreationError,
    },
    sync::FlushError,
};
use vulkano_win::VkSurfaceBuild;
use winit::{
    event::Event::WindowEvent, event_loop::ControlFlow, event_loop::EventLoop, window::Window,
    window::WindowBuilder,
};

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: "
            #version 450

            layout(location = 0) in vec3 position;
            layout(location = 1) in vec3 normal;

            layout(location = 0) out vec3 v_normal;
            layout(location = 1) out vec3 light_position;

            layout(set = 0, binding = 0) uniform Data {
                mat4 mvp;
                vec3 light;
            } uniforms;

            void main() {
                v_normal = normalize(normal);
                light_position = uniforms.light;
                gl_Position = uniforms.mvp * vec4(position, 1.0);
            }
        "
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src:"
        #version 450

        layout(location = 0) in vec3 v_normal;
        layout(location = 1) in vec3 light_position;
        layout(location = 0) out vec4 f_color;


        void main() {
            f_color = vec4(clamp(dot(v_normal, -light_position), 0.0f, 1.0f) * vec3(1.0f, 0.93f, 0.56f), 1.0f);
            //f_color = vec4(v_normal, 1.0);
        }
        "
    }
}

fn main() {
    let event_loop = EventLoop::new();

    let extensions = vulkano_win::required_extensions();

    let vk_instance = match Instance::new(None, &extensions, None) {
        Ok(i) => i,
        Err(err) => panic!("Couldn't build instance: {:?}", err),
    };

    let surface = WindowBuilder::new()
        .build_vk_surface(&event_loop, vk_instance.clone())
        .unwrap();

    let physical_device = PhysicalDevice::enumerate(&vk_instance)
        .next()
        .expect("no physical device available");

    let q_family = physical_device
        .queue_families()
        .find(|&q| q.supports_graphics())
        .expect("no graphics queue available on physical device");

    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::none()
    };

    println!("physical device name: {:?}", &physical_device.name());

    let (logical_device, mut queues) = Device::new(
        physical_device,
        &Features::none(),
        &device_extensions,
        [(q_family, 1.0)].iter().cloned(),
    )
    .expect("failed to create logical device");
    let graphics_queue = queues.next().unwrap();

    let input = include_bytes!("../../obj-rs/tests/fixtures/normal-cone.obj");
    let obj: Obj = load_obj(&input[..]).unwrap();

    let verts = obj.vertices.clone();
    let v_buff = CpuAccessibleBuffer::from_iter(
        logical_device.clone(),
        BufferUsage::vertex_buffer(),
        false,
        verts.into_iter(),
    )
    .unwrap();
    let indices = obj.indices;
    let i_buff = CpuAccessibleBuffer::from_iter(
        logical_device.clone(),
        BufferUsage::index_buffer(),
        false,
        indices.into_iter(),
    )
    .unwrap();
    let (mut swapchain, images) = {
        let caps = surface.capabilities(physical_device).unwrap();
        let alpha = caps.supported_composite_alpha.iter().next().unwrap();
        let format = caps.supported_formats[0].0;
        let dimensions: [u32; 2] = surface.window().inner_size().into();

        Swapchain::new(
            logical_device.clone(),
            surface.clone(),
            caps.min_image_count,
            format,
            dimensions,
            1,
            ImageUsage::color_attachment(),
            &graphics_queue,
            SurfaceTransform::Identity,
            alpha,
            PresentMode::Fifo,
            FullscreenExclusive::Default,
            true,
            ColorSpace::SrgbNonLinear,
        )
        .unwrap()
    };

    let uniform_buffer =
        CpuBufferPool::<vs::ty::Data>::new(logical_device.clone(), BufferUsage::all());

    let vs = vs::Shader::load(logical_device.clone()).unwrap();
    let fs = fs::Shader::load(logical_device.clone()).unwrap();

    let render_pass = Arc::new(
        vulkano::single_pass_renderpass!(logical_device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: Format::B8G8R8A8Unorm,
                    samples: 1,
                },
                depth: {
                    load: Clear,
                    store: DontCare,
                    format: Format::D16Unorm,
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {depth}
            }
        )
        .unwrap(),
    );

    let (mut pipeline, mut framebuffers) = {
        window_size_dependent_setup(
            logical_device.clone(),
            &vs,
            &fs,
            &images,
            render_pass.clone(),
        )
    };

    let mut recreate_swap = false;

    let mut previous_frame_end = Some(vulkano::sync::now(logical_device.clone()).boxed());

    event_loop.run(move |event, _, control_flow| match event {
        WindowEvent {
            event: winit::event::WindowEvent::CloseRequested,
            ..
        } => *control_flow = ControlFlow::Exit,
        WindowEvent {
            event: winit::event::WindowEvent::Resized(_),
            ..
        } => recreate_swap = true,
        winit::event::Event::RedrawEventsCleared => {
            previous_frame_end.as_mut().unwrap().cleanup_finished();

            let dimensions: [u32; 2] = surface.window().inner_size().into();
            if recreate_swap {
                let (new_swap, new_images) = match swapchain.recreate_with_dimensions(dimensions) {
                    Ok(r) => r,
                    Err(SwapchainCreationError::UnsupportedDimensions) => return,
                    Err(e) => panic!("Failed to create the swapchain {:?}", e),
                };
                swapchain = new_swap;
                let (new_pipeline, new_framebuffers) = window_size_dependent_setup(
                    logical_device.clone(),
                    &vs,
                    &fs,
                    &new_images,
                    render_pass.clone(),
                );
                pipeline = new_pipeline;
                framebuffers = new_framebuffers;
                recreate_swap = false;
            }

            let uniform_subbuffer = {
                let eye = Point3::new(0.0, 0.0, -5.0);
                let target = Point3::new(1.0, 0.0, 0.0);
                let view = Isometry3::look_at_rh(&eye, &target, &-Vector3::y());
                let model = Isometry3::new(Vector3::x(), nalgebra::zero());
                let aspect_ratio = dimensions[0] as f32 / dimensions[1] as f32;
                let projection =
                    Perspective3::new(aspect_ratio, std::f32::consts::PI / 2.0, 0.1, 1000.0);

                let mvp = projection.into_inner() * (view * model).to_homogeneous();

                let light_pos = [1.0, 1.0, 1.0f32];

                let uniform_data = vs::ty::Data {
                    mvp: mvp.into(),
                    light: light_pos,
                };

                uniform_buffer.next(uniform_data).unwrap()
            };

            let layout = pipeline.descriptor_set_layout(0).unwrap();

            let set = Arc::new(
                PersistentDescriptorSet::start(layout.clone())
                    .add_buffer(uniform_subbuffer)
                    .unwrap()
                    .build()
                    .unwrap(),
            );

            let (image_num, suboptimal, acquire_future) =
                match vulkano::swapchain::acquire_next_image(swapchain.clone(), None) {
                    Ok(r) => r,
                    Err(AcquireError::OutOfDate) => {
                        recreate_swap = true;
                        return;
                    }
                    Err(e) => panic!("Failed to get next iamge {:?}", e),
                };

            if suboptimal {
                recreate_swap = true;
            }

            let mut builder = AutoCommandBufferBuilder::primary_one_time_submit(
                logical_device.clone(),
                graphics_queue.family(),
            )
            .unwrap();

            builder
                .begin_render_pass(
                    framebuffers[image_num].clone(),
                    false,
                    vec![[0.0, 0.0, 0.0, 1.0].into(), 1f32.into()],
                )
                .unwrap()
                .draw_indexed(
                    pipeline.clone(),
                    &DynamicState::none(),
                    vec![v_buff.clone()],
                    i_buff.clone(),
                    set,
                    (),
                )
                .unwrap()
                .end_render_pass()
                .unwrap();

            let command_buffer = builder.build().unwrap();

            let future = previous_frame_end
                .take()
                .unwrap()
                .join(acquire_future)
                .then_execute(graphics_queue.clone(), command_buffer)
                .unwrap()
                .then_swapchain_present(graphics_queue.clone(), swapchain.clone(), image_num)
                .then_signal_fence_and_flush();

            match future {
                Ok(future) => {
                    previous_frame_end = Some(future.boxed());
                }
                Err(FlushError::OutOfDate) => {
                    recreate_swap = true;
                    previous_frame_end = Some(vulkano::sync::now(logical_device.clone()).boxed());
                }
                Err(e) => {
                    println!("Failed to flush future: {:?}", e);
                    previous_frame_end = Some(vulkano::sync::now(logical_device.clone()).boxed());
                }
            }
        }
        _ => (),
    })
}

fn window_size_dependent_setup(
    device: Arc<Device>,
    vs: &vs::Shader,
    fs: &fs::Shader,
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
) -> (
    Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
    Vec<Arc<dyn FramebufferAbstract + Send + Sync>>,
) {
    let dimensions = images[0].dimensions();

    let depth_buffer =
        AttachmentImage::transient(device.clone(), dimensions, Format::D16Unorm).unwrap();

    let framebuffers = images
        .iter()
        .map(|image| {
            Arc::new(
                Framebuffer::start(render_pass.clone())
                    .add(image.clone())
                    .unwrap()
                    .add(depth_buffer.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            ) as Arc<dyn FramebufferAbstract + Send + Sync>
        })
        .collect::<Vec<_>>();

    let pipeline = Arc::new(
        GraphicsPipeline::start()
            .vertex_input_single_buffer::<obj::Vertex>()
            .vertex_shader(vs.main_entry_point(), ())
            .triangle_list()
            .viewports_dynamic_scissors_irrelevant(1)
            .viewports(std::iter::once(Viewport {
                origin: [0.0, 0.0],
                dimensions: [dimensions[0] as f32, dimensions[1] as f32],
                depth_range: 0.0..1.0,
            }))
            .fragment_shader(fs.main_entry_point(), ())
            .depth_stencil_simple_depth()
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            .build(device)
            .unwrap(),
    );

    (pipeline, framebuffers)
}
