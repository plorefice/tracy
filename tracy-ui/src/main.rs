//! Visualization of scenes rendered by Tracy using the `imgui-rs` crate.

#![deny(missing_debug_implementations)]
#![warn(missing_docs)]

use std::{path::Path, time::Instant};

use futures::executor::block_on;
use image::{ImageBuffer, Rgb};
use imgui::{self as im, im_str};
use imgui_wgpu::{Renderer, RendererConfig, Texture, TextureConfig};
use scene::{get_scene_list, Scene};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

mod scene;

const CANVAS_WIDTH: u32 = 512;
const CANVAS_HEIGHT: u32 = 512;

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

    // Set up a default scene
    let mut scenes = get_scene_list();
    let mut texture_id = Some(render_scene(
        None,
        scenes.last().unwrap().as_ref(),
        CANVAS_WIDTH,
        CANVAS_HEIGHT,
        &queue,
        &device,
        &mut renderer,
    ));

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
                    width: size.width,
                    height: size.height,
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

                draw_ui(
                    &ui,
                    &mut scenes,
                    &mut texture_id,
                    &queue,
                    &device,
                    &mut renderer,
                );

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

fn draw_ui(
    ui: &im::Ui,
    scenes: &mut [Box<dyn Scene>],
    texture_id: &mut Option<im::TextureId>,
    queue: &wgpu::Queue,
    device: &wgpu::Device,
    renderer: &mut imgui_wgpu::Renderer,
) {
    draw_canvas(ui, texture_id);
    draw_scene_picker(ui, scenes, texture_id, queue, device, renderer);
}

fn draw_scene_picker(
    ui: &im::Ui,
    scenes: &mut [Box<dyn Scene>],
    texture_id: &mut Option<im::TextureId>,
    queue: &wgpu::Queue,
    device: &wgpu::Device,
    renderer: &mut imgui_wgpu::Renderer,
) {
    let window = im::Window::new(im_str!("Scenarios"));

    window
        .size([432., 512.], im::Condition::FirstUseEver)
        .position([800., 48.], im::Condition::FirstUseEver)
        .build(&ui, || {
            for scene in scenes.iter_mut() {
                draw_scene_entry(ui, scene, texture_id, queue, device, renderer);
            }
        });
}

fn draw_canvas(ui: &im::Ui, texture_id: &Option<im::TextureId>) {
    let window = im::Window::new(im_str!("Canvas"));
    let size = [CANVAS_WIDTH as f32, CANVAS_HEIGHT as f32];

    window
        .position([48., 48.], im::Condition::FirstUseEver)
        .build(&ui, || {
            if let Some(ref id) = texture_id {
                im::Image::new(*id, size).build(&ui);
            }
        });
}

fn draw_scene_entry(
    ui: &im::Ui,
    scene: &mut Box<dyn Scene>,
    texture_id: &mut Option<im::TextureId>,
    queue: &wgpu::Queue,
    device: &wgpu::Device,
    renderer: &mut imgui_wgpu::Renderer,
) {
    if im::CollapsingHeader::new(&im::ImString::new(&scene.name())).build(&ui) {
        ui.text(im::ImString::new(&scene.description()));
        ui.separator();
        let redraw = scene.draw(&ui);
        ui.separator();
        let force = ui.button(&im_str!("Render it!##{}", scene.name()), [0., 0.]);
        ui.same_line(0.);
        let save = ui.button(&im_str!("Save as PNG##{}", scene.name()), [0., 0.]);

        if redraw || force {
            *texture_id = Some(render_scene(
                *texture_id,
                scene.as_ref(),
                CANVAS_WIDTH,
                CANVAS_HEIGHT,
                queue,
                device,
                renderer,
            ));
        }

        if save {
            save_scene(
                scene.as_ref(),
                CANVAS_WIDTH,
                CANVAS_HEIGHT,
                &format!("{}.png", scene.name()),
            );
        }
    }
}

fn render_scene<S>(
    texture_id: Option<im::TextureId>,
    scene: &S,
    width: u32,
    height: u32,
    queue: &wgpu::Queue,
    device: &wgpu::Device,
    renderer: &mut imgui_wgpu::Renderer,
) -> im::TextureId
where
    S: Scene + ?Sized,
{
    let canvas = scene.render(width, height);
    let raw_data = canvas
        .iter()
        .flat_map(|c| {
            let (r, g, b) = c.to_rgb888();
            vec![b, g, r, 255]
        })
        .collect::<Vec<_>>();

    let texture_config = TextureConfig {
        size: wgpu::Extent3d {
            width,
            height,
            ..Default::default()
        },
        label: Some("canvas"),
        ..Default::default()
    };

    let texture = Texture::new(&device, &renderer, texture_config);
    texture.write(&queue, &raw_data, width, height);

    match texture_id {
        Some(texture_id) => {
            renderer.textures.replace(texture_id, texture);
            texture_id
        }
        None => renderer.textures.insert(texture),
    }
}

fn save_scene<S, P>(scene: &S, width: u32, height: u32, path: P)
where
    S: Scene + ?Sized,
    P: AsRef<Path>,
{
    let buf = scene
        .render(width, height)
        .iter()
        .flat_map(|p| {
            let (r, g, b) = p.to_rgb888();
            vec![r, g, b]
        })
        .collect::<Vec<u8>>();

    ImageBuffer::<Rgb<u8>, _>::from_vec(width, height, buf)
        .unwrap()
        .save(path)
        .unwrap();
}
