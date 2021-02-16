use std::{path::Path, time::Instant};

use futures::executor::block_on;
use image::{ImageBuffer, Rgb};
use imgui::{self as im, im_str};
use imgui_wgpu::{Renderer, RendererConfig, Texture, TextureConfig};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use crate::scene::{self, Scene};

pub struct TracyUi {
    scenes: Vec<Box<dyn Scene>>,
    current_scene_id: usize,

    canvas_size: [f32; 2],
    texture_id: Option<im::TextureId>,
}

impl TracyUi {
    /// Creates a new user interface instance.
    pub fn new() -> Self {
        Self {
            scenes: scene::get_scene_list(),
            current_scene_id: 0,
            canvas_size: [512.0, 512.0],
            texture_id: None,
        }
    }

    /// Loops forever or until the user closes the window.
    pub fn run(mut self) {
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
        self.render_current_scene(&queue, &device, &mut renderer);

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

                    self.draw_ui(&ui, &queue, &device, &mut renderer);

                    let mut encoder: wgpu::CommandEncoder = device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

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
        &mut self,
        ui: &im::Ui,
        queue: &wgpu::Queue,
        device: &wgpu::Device,
        renderer: &mut imgui_wgpu::Renderer,
    ) {
        self.draw_canvas(ui);
        self.draw_scene_picker(ui, queue, device, renderer);
    }

    fn draw_scene_picker(
        &mut self,
        ui: &im::Ui,
        queue: &wgpu::Queue,
        device: &wgpu::Device,
        renderer: &mut imgui_wgpu::Renderer,
    ) {
        let window = im::Window::new(im_str!("Scenarios"));

        window
            .size([432., 512.], im::Condition::FirstUseEver)
            .position([800., 48.], im::Condition::FirstUseEver)
            .build(&ui, || {
                for scene_id in 0..self.scenes.len() {
                    self.draw_scene_entry(ui, scene_id, queue, device, renderer);
                }
            });
    }

    fn draw_canvas(&mut self, ui: &im::Ui) {
        im::Window::new(im_str!("Canvas"))
            .position([48., 48.], im::Condition::FirstUseEver)
            .build(&ui, || {
                if let Some(ref id) = self.texture_id {
                    // Adapt image to window size (or default to 512x512)
                    let mut size = ui.content_region_avail();
                    if size[0] == 0.0 || size[1] == 0.0 {
                        size = self.canvas_size;
                    } else {
                        self.canvas_size = size;
                    }

                    im::Image::new(*id, size).build(&ui);
                }
            });
    }

    fn draw_scene_entry(
        &mut self,
        ui: &im::Ui,
        id: usize,
        queue: &wgpu::Queue,
        device: &wgpu::Device,
        renderer: &mut imgui_wgpu::Renderer,
    ) {
        let scene = self.scenes.get_mut(id).unwrap();
        let name = scene.name();

        if im::CollapsingHeader::new(&im::ImString::new(&name)).build(&ui) {
            ui.text(im::ImString::new(&scene.description()));
            ui.separator();
            let redraw = scene.draw(&ui);
            ui.separator();
            let force = ui.button(&im_str!("Render it!##{}", name), [0., 0.]);
            ui.same_line(0.);
            let save = ui.button(&im_str!("Save as PNG##{}", name), [0., 0.]);

            if redraw || force {
                self.current_scene_id = id;
                self.render_current_scene(queue, device, renderer);
            }

            if save {
                self.save_current_scene(&format!("{}.png", name));
            }
        }
    }

    fn render_current_scene(
        &mut self,
        queue: &wgpu::Queue,
        device: &wgpu::Device,
        renderer: &mut imgui_wgpu::Renderer,
    ) {
        let scene = self.scenes.get(self.current_scene_id).unwrap();
        let width = self.canvas_size[0] as u32;
        let height = self.canvas_size[1] as u32;

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

        if let Some(id) = self.texture_id {
            renderer.textures.replace(id, texture);
        } else {
            self.texture_id = Some(renderer.textures.insert(texture));
        }
    }

    fn save_current_scene<P>(&self, path: P)
    where
        P: AsRef<Path>,
    {
        let scene = self.scenes.get(self.current_scene_id).unwrap();
        let width = self.canvas_size[0] as u32;
        let height = self.canvas_size[1] as u32;

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
}
