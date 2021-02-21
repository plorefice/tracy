use std::{path::Path, time::Instant};

use futures::executor::block_on;
use image::{ImageBuffer, Rgb};
use imgui::{self as im, im_str};
use imgui_wgpu::{Renderer, RendererConfig, Texture, TextureConfig};
use imgui_winit_support::WinitPlatform;
use scene::Streamer;
use tracy::rendering::{Canvas, Stream};
use winit::{
    dpi::{LogicalPosition, LogicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::Window,
};

use crate::scene::{self, Scene};

const DEFAULT_WIDTH: u32 = 512;
const DEFAULT_HEIGHT: u32 = 512;

pub struct TracyUi {
    event_loop: EventLoop<()>,
    scene_mgr: SceneManager,
    ctx: UiContext,
    gfx: GfxBackend,
}

struct UiContext {
    imgui: im::Context,
    window: Window,
    platform: WinitPlatform,
}

#[derive(Default)]
struct UiState {
    render_scene: Option<usize>,
    save_scene: Option<usize>,
    canvas_width: u32,
    canvas_height: u32,
}

struct GfxBackend {
    queue: wgpu::Queue,
    device: wgpu::Device,
    surface: wgpu::Surface,
    renderer: imgui_wgpu::Renderer,
    swap_chain: wgpu::SwapChain,
    texture_id: Option<im::TextureId>,
}

struct SceneManager {
    scenes: Vec<Box<dyn Scene>>,
    current_scene_id: usize,
}

impl TracyUi {
    /// Creates a new user interface instance.
    pub fn new<S: AsRef<str>>(title: S, width: u32, height: u32) -> Self {
        // Set up window and GPU
        let event_loop = EventLoop::new();
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

        let (window, size, surface) = {
            let window = Window::new(&event_loop).unwrap();
            window.set_title(title.as_ref());
            window.set_inner_size(LogicalSize { width, height });
            window.set_outer_position(LogicalPosition::new(0, 0));

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

        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        // Set up dear imgui
        let mut imgui = im::Context::create();
        let mut platform = WinitPlatform::init(&mut imgui);
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
        let renderer_config = RendererConfig {
            texture_format: sc_desc.format,
            ..Default::default()
        };

        let renderer = Renderer::new(&mut imgui, &device, &queue, renderer_config);

        // Build UI structure
        Self {
            event_loop,
            scene_mgr: SceneManager {
                scenes: scene::get_scene_list(),
                current_scene_id: 0,
            },
            ctx: UiContext {
                platform,
                window,
                imgui,
            },
            gfx: GfxBackend {
                queue,
                device,
                surface,
                renderer,
                swap_chain,
                texture_id: None,
            },
        }
    }

    /// Loops forever or until the user closes the window.
    pub fn run(self) {
        let TracyUi {
            mut event_loop,
            mut scene_mgr,
            mut ctx,
            mut gfx,
        } = self;

        let mut last_frame = Instant::now();
        let mut last_cursor = None;

        let clear_color = wgpu::Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        };

        let mut current_stream: Option<Stream> = None;
        let mut streamers: Vec<Box<dyn Streamer>> =
            vec![Box::new(scene::ch11::Reflections::new().unwrap())];

        // Event loop
        event_loop.run_return(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    ..
                } => {
                    let size = ctx.window.inner_size();

                    let sc_desc = wgpu::SwapChainDescriptor {
                        usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
                        format: wgpu::TextureFormat::Bgra8UnormSrgb,
                        width: size.width,
                        height: size.height,
                        present_mode: wgpu::PresentMode::Mailbox,
                    };

                    gfx.swap_chain = gfx.device.create_swap_chain(&gfx.surface, &sc_desc);
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                Event::MainEventsCleared => ctx.window.request_redraw(),
                Event::RedrawEventsCleared => {
                    let now = Instant::now();
                    ctx.imgui.io_mut().update_delta_time(now - last_frame);
                    last_frame = now;

                    let frame = match gfx.swap_chain.get_current_frame() {
                        Ok(frame) => frame,
                        Err(e) => {
                            eprintln!("dropped frame: {:?}", e);
                            return;
                        }
                    };

                    ctx.platform
                        .prepare_frame(ctx.imgui.io_mut(), &ctx.window)
                        .expect("Failed to prepare frame");

                    let ui = ctx.imgui.frame();

                    let mut state = UiState::default();
                    state.draw_ui(&ui, &mut scene_mgr, gfx.texture_id);

                    // Render next frame if a rendering is in progress
                    if let Some(ref mut stream) = current_stream {
                        if stream.advance() {
                            gfx.render_to_texture(
                                state.canvas_width,
                                state.canvas_height,
                                stream.canvas(),
                            )
                        } else {
                            current_stream = None;
                        }
                    }

                    // New render triggered/forced
                    if let Some(_id) = state.render_scene {
                        // SAFETY: because of some lifetime fuckery that I don't fully understand,
                        // the closure we are in has an anonymous lifetime different from the one
                        // on `Stream`, and AFAIK it cannot be changed. As a result of this, items
                        // borrowed from `streamers` cannot fulfill both lifetimes, despite them
                        // living (supposedly) longer than this closure. Anyway, make the borrow
                        // checker happy by transmuting the lifetime to one it can accept.
                        let stream = unsafe {
                            std::mem::transmute::<
                                &'_ mut Box<dyn Streamer>,
                                &'_ mut Box<dyn Streamer>,
                            >(&mut streamers[0])
                        };

                        current_stream =
                            Some(stream.stream(state.canvas_width, state.canvas_height));
                    }

                    // Image save requested
                    if let Some(id) = state.save_scene {
                        scene_mgr.save_current_scene(
                            state.canvas_width,
                            state.canvas_height,
                            &format!("{}.png", scene_mgr.scenes.get(id).unwrap().name()),
                        );
                    }

                    let mut encoder: wgpu::CommandEncoder = gfx
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                    if last_cursor != Some(ui.mouse_cursor()) {
                        last_cursor = Some(ui.mouse_cursor());
                        ctx.platform.prepare_render(&ui, &ctx.window);
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

                    gfx.renderer
                        .render(ui.render(), &gfx.queue, &gfx.device, &mut rpass)
                        .expect("Rendering failed");

                    drop(rpass);

                    gfx.queue.submit(Some(encoder.finish()));
                }
                _ => (),
            }

            ctx.platform
                .handle_event(ctx.imgui.io_mut(), &ctx.window, &event);
        });
    }
}

impl UiState {
    fn draw_ui(
        &mut self,
        ui: &im::Ui,
        scene_mgr: &mut SceneManager,
        texture: Option<im::TextureId>,
    ) {
        self.draw_canvas(ui, texture);
        self.draw_scene_picker(ui, scene_mgr);
    }

    fn draw_canvas(&mut self, ui: &im::Ui, texture: Option<im::TextureId>) {
        im::Window::new(im_str!("Canvas"))
            .size(
                [DEFAULT_WIDTH as f32, DEFAULT_HEIGHT as f32],
                im::Condition::FirstUseEver,
            )
            .position([48., 48.], im::Condition::FirstUseEver)
            .build(&ui, || {
                // Track canvas size changes
                let size = ui.content_region_avail();
                self.canvas_width = size[0] as u32;
                self.canvas_height = size[1] as u32;

                if let Some(tid) = texture {
                    im::Image::new(tid, size).build(&ui);
                }
            });
    }

    fn draw_scene_picker(&mut self, ui: &im::Ui, scene_mgr: &mut SceneManager) {
        im::Window::new(im_str!("Scenarios"))
            .size([432., 512.], im::Condition::FirstUseEver)
            .position([800., 48.], im::Condition::FirstUseEver)
            .build(&ui, || {
                for scene_id in 0..scene_mgr.scenes.len() {
                    self.draw_scene_entry(ui, scene_mgr, scene_id);
                }
            });
    }

    fn draw_scene_entry(&mut self, ui: &im::Ui, scene_mgr: &mut SceneManager, scene_id: usize) {
        let scene = scene_mgr.scenes.get_mut(scene_id).unwrap();
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
                self.render_scene = Some(scene_id);
            }
            if save {
                self.save_scene = Some(scene_id);
            }
        }
    }
}

impl SceneManager {
    fn save_current_scene<P>(&self, width: u32, height: u32, path: P)
    where
        P: AsRef<Path>,
    {
        let scene = self.scenes.get(self.current_scene_id).unwrap();

        let canvas = scene
            .render(width, height)
            .unwrap_or_else(|e| panic!("Could not render scene \"{}\": {}", scene.name(), e));

        let buf = canvas
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

impl GfxBackend {
    fn render_to_texture(&mut self, width: u32, height: u32, canvas: &Canvas) {
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

        let texture = Texture::new(&self.device, &self.renderer, texture_config);
        texture.write(&self.queue, &raw_data, width, height);

        if let Some(id) = self.texture_id {
            self.renderer.textures.replace(id, texture);
        } else {
            self.texture_id = Some(self.renderer.textures.insert(texture));
        }
    }
}
