use std::time::Instant;

use futures::executor::block_on;
use imgui::{self as im, im_str};
use imgui_wgpu::{Renderer, RendererConfig, Texture, TextureConfig};
use scene::Scene;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

mod scene;

const WINDOW_PAD_X: f32 = 16.;
const WINDOW_PAD_Y: f32 = 36.;

fn main() {
    // Set up window and GPU
    let event_loop = EventLoop::new();
    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

    let (window, size, surface) = {
        let window = Window::new(&event_loop).unwrap();
        window.set_title("Tracy UI");
        window.set_inner_size(LogicalSize {
            width: 1280,
            height: 640,
        });

        let size = window.inner_size();
        let surface = unsafe { instance.create_surface(&window) };

        (window, size, surface)
    };

    let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
    }))
    .unwrap();

    let (device, queue) =
        block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None)).unwrap();

    // Set up swap chain
    let sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        width: size.width as u32,
        height: size.height as u32,
        present_mode: wgpu::PresentMode::Mailbox,
    };

    let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

    // Set up dear imgui
    let mut imgui = im::Context::create();
    let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
    platform.attach_window(
        imgui.io_mut(),
        &window,
        imgui_winit_support::HiDpiMode::Default,
    );
    imgui.set_ini_filename(None);

    let hidpi_factor = window.scale_factor();
    let font_size = (13.0 * hidpi_factor) as f32;
    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

    imgui.fonts().add_font(&[im::FontSource::DefaultFontData {
        config: Some(im::FontConfig {
            oversample_h: 1,
            pixel_snap_h: true,
            size_pixels: font_size,
            ..Default::default()
        }),
    }]);

    // Set up dear imgui wgpu renderer
    let clear_color = wgpu::Color {
        r: 0.1,
        g: 0.2,
        b: 0.3,
        a: 1.0,
    };

    let renderer_config = RendererConfig {
        texture_format: sc_desc.format,
        ..Default::default()
    };

    let mut renderer = Renderer::new(&mut imgui, &device, &queue, renderer_config);
    let mut last_frame = Instant::now();
    let mut last_cursor = None;

    // Set up scene
    let (width, height) = (512, 512);
    let texture_id = render_to_texture(
        None,
        &scene::SCENES[0],
        width,
        height,
        &queue,
        &device,
        &mut renderer,
    );

    // Event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                let size = window.inner_size();

                let sc_desc = wgpu::SwapChainDescriptor {
                    usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    width: size.width as u32,
                    height: size.height as u32,
                    present_mode: wgpu::PresentMode::Mailbox,
                };

                swap_chain = device.create_swap_chain(&surface, &sc_desc);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::MainEventsCleared => window.request_redraw(),
            Event::RedrawEventsCleared => {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;

                let frame = match swap_chain.get_current_frame() {
                    Ok(frame) => frame,
                    Err(e) => {
                        eprintln!("dropped frame: {:?}", e);
                        return;
                    }
                };

                platform
                    .prepare_frame(imgui.io_mut(), &window)
                    .expect("Failed to prepare frame");

                let ui = imgui.frame();
                {
                    let window = im::Window::new(im_str!("Canvas"));
                    let size = [width as f32, height as f32];

                    window
                        .size(
                            [size[0] + WINDOW_PAD_X, size[1] + WINDOW_PAD_Y],
                            im::Condition::FirstUseEver,
                        )
                        .position([48., 48.], im::Condition::FirstUseEver)
                        .build(&ui, || im::Image::new(texture_id, size).build(&ui));

                    let window = im::Window::new(im_str!("Scenarios"));

                    window
                        .size([400., 512.], im::Condition::FirstUseEver)
                        .position([832., 48.], im::Condition::FirstUseEver)
                        .build(&ui, || {
                            for (i, scene) in scene::SCENES.iter().enumerate() {
                                if im::CollapsingHeader::new(&im::ImString::new(&scene.name))
                                    .default_open(i == 0)
                                    .build(&ui)
                                {
                                    ui.text(im::ImString::new(&scene.description));

                                    if ui.button(&im_str!("Render it!##{}", i), [0., 0.]) {
                                        render_to_texture(
                                            Some(texture_id),
                                            &scene::SCENES[i],
                                            width,
                                            height,
                                            &queue,
                                            &device,
                                            &mut renderer,
                                        );
                                    }
                                }
                            }
                        });
                }

                let mut encoder: wgpu::CommandEncoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                if last_cursor != Some(ui.mouse_cursor()) {
                    last_cursor = Some(ui.mouse_cursor());
                    platform.prepare_render(&ui, &window);
                }

                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.output.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(clear_color),
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: None,
                });

                renderer
                    .render(ui.render(), &queue, &device, &mut rpass)
                    .expect("Rendering failed");

                drop(rpass);

                queue.submit(Some(encoder.finish()));
            }
            _ => (),
        }

        platform.handle_event(imgui.io_mut(), &window, &event);
    });
}

fn render_to_texture(
    id: Option<im::TextureId>,
    scene: &Scene,
    width: usize,
    height: usize,
    queue: &wgpu::Queue,
    device: &wgpu::Device,
    renderer: &mut imgui_wgpu::Renderer,
) -> im::TextureId {
    let canvas = (scene.render_fn)(width, height);
    let raw_data = canvas
        .iter()
        .flat_map(|c| {
            let (r, g, b) = c.to_rgb888();
            vec![b, g, r, 255]
        })
        .collect::<Vec<_>>();

    let texture_config = TextureConfig {
        size: wgpu::Extent3d {
            width: width as u32,
            height: height as u32,
            ..Default::default()
        },
        label: Some("canvas"),
        ..Default::default()
    };

    let texture = Texture::new(&device, &renderer, texture_config);
    texture.write(&queue, &raw_data, width as u32, height as u32);

    match id {
        Some(id) => {
            renderer.textures.replace(id, texture);
            id
        }
        None => renderer.textures.insert(texture),
    }
}
