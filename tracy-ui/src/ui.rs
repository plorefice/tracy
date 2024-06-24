use std::{
    path::Path,
    time::{Duration, Instant},
};

use futures::executor::block_on;
use image::{ImageBuffer, Rgb};
use imgui::{self as im};
use imgui_wgpu::{Renderer, RendererConfig, Texture, TextureConfig};
use imgui_winit_support::WinitPlatform;
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

const MAX_RENDER_BATCH_DURATION: Duration = Duration::from_millis(50);

pub struct TracyUi {
    event_loop: EventLoop<()>,
    ctx: UiContext,
    gfx: GfxBackend,
}

struct UiContext {
    imgui: im::Context,
    window: Window,
    platform: WinitPlatform,
}

struct GfxBackend {
    queue: wgpu::Queue,
    device: wgpu::Device,
    surface: wgpu::Surface,
    renderer: imgui_wgpu::Renderer,
    texture_id: Option<im::TextureId>,
}

#[derive(Default)]
struct UiState {
    render_scene: Option<usize>,
    save_scene: Option<usize>,
    canvas_width: u32,
    canvas_height: u32,
    stop_rendering: bool,
    freeze_canvas_size: bool,
}

impl TracyUi {
    /// Creates a new user interface instance.
    pub fn new<S: AsRef<str>>(title: S, width: u32, height: u32) -> Self {
        // Set up window and GPU
        let event_loop = EventLoop::new();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let (window, size, surface) = {
            let window = Window::new(&event_loop).unwrap();
            window.set_title(title.as_ref());
            window.set_inner_size(LogicalSize { width, height });
            window.set_outer_position(LogicalPosition::new(0, 0));

            let size = window.inner_size();
            let surface = unsafe { instance.create_surface(&window) }.unwrap();

            (window, size, surface)
        };

        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            ..Default::default()
        }))
        .unwrap();

        let (device, queue) =
            block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None)).unwrap();

        // Set up swap chain
        let surface_desc = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![wgpu::TextureFormat::Bgra8Unorm],
        };

        surface.configure(&device, &surface_desc);

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
            texture_format: surface_desc.format,
            ..Default::default()
        };

        let renderer = Renderer::new(&mut imgui, &device, &queue, renderer_config);

        // Build UI structure
        Self {
            event_loop,
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
                texture_id: None,
            },
        }
    }

    /// Loops forever or until the user closes the window.
    pub fn run(self) {
        let TracyUi {
            mut event_loop,
            mut ctx,
            mut gfx,
        } = self;

        let mut scenes = scene::get_scene_list().unwrap();
        let mut current_render: Option<Stream> = None;

        let mut last_frame = Instant::now();
        let mut last_cursor = None;

        let clear_color = wgpu::Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        };

        // Event loop
        event_loop.run_return(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::Resized(size),
                    ..
                } => {
                    let surface_desc = wgpu::SurfaceConfiguration {
                        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                        format: wgpu::TextureFormat::Bgra8UnormSrgb,
                        width: size.width,
                        height: size.height,
                        present_mode: wgpu::PresentMode::Mailbox,
                        alpha_mode: wgpu::CompositeAlphaMode::Auto,
                        view_formats: vec![wgpu::TextureFormat::Bgra8Unorm],
                    };

                    gfx.surface.configure(&gfx.device, &surface_desc);
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

                    // Prepare next frame
                    let frame = match gfx.surface.get_current_texture() {
                        Ok(frame) => frame,
                        Err(e) => {
                            eprintln!("dropped frame: {:?}", e);
                            return;
                        }
                    };

                    ctx.platform
                        .prepare_frame(ctx.imgui.io_mut(), &ctx.window)
                        .expect("Failed to prepare frame");

                    // Draw UI and capture user's input
                    let ui = ctx.imgui.frame();
                    let mut state = UiState {
                        freeze_canvas_size: current_render.is_some(),
                        ..UiState::default()
                    };
                    state.draw_ui(ui, &mut scenes[..], gfx.texture_id);

                    // User has stopped the rendering
                    if state.stop_rendering {
                        current_render = None;
                    }

                    // New render triggered/forced
                    if let Some(id) = state.render_scene {
                        // SAFETY: because of some lifetime fuckery that I don't fully understand,
                        // the closure we are in has an anonymous lifetime different from the one
                        // on `Stream`, and AFAIK it cannot be changed. As a result of this, items
                        // borrowed from `scenes` cannot fulfill both lifetimes, despite them
                        // living (supposedly) longer than this closure. Anyway, make the borrow
                        // checker happy by transmuting the lifetime to one it can accept.
                        let scene = unsafe {
                            std::mem::transmute::<&'_ mut Box<dyn Scene>, &'_ mut Box<dyn Scene>>(
                                &mut scenes[id],
                            )
                        };

                        current_render =
                            Some(scene.render(state.canvas_width, state.canvas_height));
                    }

                    // Render next batch of frames if a rendering is in progress
                    if let Some(ref mut stream) = current_render {
                        let mut render = false;

                        // Render for MAX_RENDER_BATCH_DURATION or until we are done
                        let start = Instant::now();
                        while start.elapsed() <= MAX_RENDER_BATCH_DURATION && stream.advance() {
                            render = true;
                        }

                        if render {
                            gfx.render_to_texture(
                                state.canvas_width,
                                state.canvas_height,
                                stream.canvas(),
                            )
                        } else {
                            current_render = None;
                        }
                    }

                    // Image save requested
                    if let Some(id) = state.save_scene {
                        let path = format!("{}.png", scenes.get(id).unwrap().name());

                        save_current_scene(
                            &mut scenes[id],
                            state.canvas_width,
                            state.canvas_height,
                            &path,
                        );
                    }

                    // Finalize frame rendering
                    let mut encoder: wgpu::CommandEncoder = gfx
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                    if last_cursor != Some(ui.mouse_cursor()) {
                        last_cursor = Some(ui.mouse_cursor());
                        ctx.platform.prepare_render(ui, &ctx.window);
                    }

                    let view = frame
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());

                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(clear_color),
                                store: true,
                            },
                        })],
                        depth_stencil_attachment: None,
                    });

                    gfx.renderer
                        .render(ctx.imgui.render(), &gfx.queue, &gfx.device, &mut rpass)
                        .expect("Rendering failed");

                    drop(rpass);

                    gfx.queue.submit(Some(encoder.finish()));

                    frame.present();
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
        scenes: &mut [Box<dyn Scene>],
        texture: Option<im::TextureId>,
    ) {
        self.draw_canvas(ui, texture);
        self.draw_scene_picker(ui, scenes);
    }

    fn draw_canvas(&mut self, ui: &im::Ui, texture: Option<im::TextureId>) {
        ui.window("Canvas")
            .size(
                [DEFAULT_WIDTH as f32, DEFAULT_HEIGHT as f32],
                im::Condition::FirstUseEver,
            )
            .resizable(!self.freeze_canvas_size)
            .position([48., 48.], im::Condition::FirstUseEver)
            .build(|| {
                self.stop_rendering = ui.button("Stop rendering");
                ui.separator();

                // Track canvas size changes
                let size = ui.content_region_avail();
                self.canvas_width = size[0] as u32;
                self.canvas_height = size[1] as u32;

                if let Some(tid) = texture {
                    im::Image::new(tid, size).build(ui);
                }
            });
    }

    fn draw_scene_picker(&mut self, ui: &im::Ui, scenes: &mut [Box<dyn Scene>]) {
        ui.window("Scenarios")
            .size([432., 512.], im::Condition::FirstUseEver)
            .position([800., 48.], im::Condition::FirstUseEver)
            .build(|| {
                for scene_id in 0..scenes.len() {
                    self.draw_scene_entry(ui, scenes, scene_id);
                }
            });
    }

    fn draw_scene_entry(&mut self, ui: &im::Ui, scenes: &mut [Box<dyn Scene>], scene_id: usize) {
        let scene = scenes.get_mut(scene_id).unwrap();
        let name = scene.name();

        if im::CollapsingHeader::new(&im::ImString::new(&name)).build(ui) {
            ui.text(im::ImString::new(scene.description()));
            ui.separator();
            let redraw = scene.draw(ui);
            ui.separator();
            let force = ui.button(format!("Render it!##{name}"));
            ui.same_line();
            let save = ui.button(format!("Save as PNG##{name}"));

            if redraw || force {
                self.render_scene = Some(scene_id);
            }
            if save {
                self.save_scene = Some(scene_id);
            }
        }
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

fn save_current_scene<P>(scene: &mut Box<dyn Scene>, width: u32, height: u32, path: P)
where
    P: AsRef<Path>,
{
    let canvas = scene.render(width, height).finalize();

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
