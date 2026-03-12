pub mod error;
pub mod wgpu;

use std::num::{NonZeroU32, NonZeroU64};
use crate::ui::error::{StartError, StartErrorTask};
use crate::ui::wgpu::{create_instance, get_surface_config, get_texture_format, setup_graphics, GraphicsState};
use image::ImageReader;
use std::ops::Deref;
use std::sync::Arc;
use ::wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, Buffer, BufferBindingType, BufferSize, BufferUsages, CommandEncoder, CommandEncoderDescriptor, Device, Extent3d, Operations, Origin3d, Queue, RenderPass, RenderPassColorAttachment, RenderPassDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages, TexelCopyBufferLayout, TexelCopyTextureInfo, Texture, TextureAspect, TextureDescriptor, TextureDimension, TextureFormat, TextureSampleType, TextureUsages, TextureView, TextureViewDescriptor, TextureViewDimension};
use ::wgpu::util::{BufferInitDescriptor, DeviceExt};
use egui::{epaint, CentralPanel, RawInput};
use egui_wgpu::{RendererOptions, ScreenDescriptor};
use pollster::FutureExt;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::platform::windows::EventLoopBuilderExtWindows;
use winit::window::{Window, WindowAttributes, WindowId};

macro_rules! nz {
    ($val:expr) => {{
        const CHECK: NonZeroU32 = match NonZeroU32::new($val) {
            Some(v) => v,
            None => panic!("nz! macro got zero!"),
        };
        CHECK
    }};
}

struct Ui {
    logo: Vec<u8>,
    size: (u32, u32),
    loaded: bool,
    texture: Option<Texture>,
    egui_texture: Option<Texture>,
    egui_ctx: egui::Context,
    egui_renderer: egui_wgpu::Renderer,
    screen: Option<ScreenDescriptor>,
    quad_options: Option<Buffer>,
    output_view: Option<TextureView>,
    bind_group: Option<BindGroup>
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct QuadOptions {
    size: [f32; 2],
    origin: [f32; 4],
}

impl Ui {
    pub fn start(device: &Device) -> Result<Self, ()> {
        let reader = ImageReader::open(r"C:\Users\rayya\Downloads\Untitled.png") // TODO: error handling needed
            .map_err(|err| {
                panic!("{}", err.to_string());
                ()
            })?
            .decode()
            .map_err(|err| {
                panic!("{}", err.to_string());
                ()
            })?
            .into_rgba8();

        let ctx = egui::Context::default();
        let renderer = egui_wgpu::Renderer::new(device, get_texture_format(), RendererOptions {
            depth_stencil_format: None,
            dithering: true,
            msaa_samples: 1,
            predictable_texture_filtering: false
        });

        let ui = Self {
            size: (reader.width(), reader.height()),
            logo: reader.to_vec(),
            loaded: false,
            texture: None,
            egui_ctx: ctx,
            egui_texture: None,
            egui_renderer: renderer,
            screen: None,
            quad_options: None,
            output_view: None,
            bind_group: None
        };
        Ok(ui)
    }

    pub fn prepare(&mut self, size: (u32, u32)) {
        self.screen_descriptor(size);
    }

    pub fn ui_clipped(&mut self, device: &Device, queue: &Queue) -> Vec<epaint::ClippedPrimitive> {
        let output = self.egui_ctx.run(RawInput::default(), |ctx| {
            CentralPanel::default().show(ctx, |ui| {
                let _ = ui.button("Hello World");
            });
        });

        let clipped = self.egui_ctx.tessellate(output.shapes, output.pixels_per_point);
        for (id, delta) in &output.textures_delta.set {
            println!("Uppdating buffer");
            self.egui_renderer.update_texture(device, queue, *id, delta);
        }

        for entry in &output.textures_delta.free {
            self.egui_renderer.free_texture(entry);
        }

        clipped
    }

    fn screen_descriptor(&mut self, size: (u32, u32)) {
        self.screen = Some(ScreenDescriptor {
            size_in_pixels: [size.0, size.1],
            pixels_per_point: 1.0
        });
    }

    pub fn buffer_clipped(&mut self, device: &Device, queue: &Queue, encoder: &mut CommandEncoder, clipped_primitives: &[epaint::ClippedPrimitive]) {
        self.egui_renderer.update_buffers(device, queue, encoder, &clipped_primitives, &self.screen.as_ref().unwrap());
    }

    pub fn render_wgpu(&mut self, clipped: &[epaint::ClippedPrimitive], pass: &mut RenderPass<'static>) {
        self.egui_renderer.render(pass, clipped, &self.screen.as_ref().unwrap());
    }

    pub fn setup_vertex_buffer(&mut self, device: &Device) {
        self.quad_options = Some(device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Output frame quads"),
            usage: BufferUsages::VERTEX,
            contents: &bytemuck::bytes_of(&QuadOptions {
                origin: [0.2, 0.2, 0.0, 1.0],
                size: [0.5, 0.5]
            })
        }));
    }

    pub fn make_sampler(&mut self, device: &Device) -> Sampler {
        device.create_sampler(&SamplerDescriptor {
            label: Some("Output sampler"),
            ..Default::default()
        })
    }

    pub fn setup_bind_group(&mut self, device: &Device) {
        let layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Output data layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::all(),
                    count: None,
                    ty: BindingType::Texture {
                        multisampled: false,
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                    }
                },
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::all(),
                    count: None,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering)
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::all(),
                    count: None,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: Some(NonZeroU64::new(size_of::<QuadOptions>() as u64).unwrap())
                    }
                }
            ]
        });

        let sampler = self.make_sampler(device);

        let binding = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Output resources"),
            layout: &layout,
            entries: &[
                BindGroupEntry { binding: 0, resource: BindingResource::Sampler(&sampler) },
                BindGroupEntry { binding: 1, resource: BindingResource::TextureView(self.output_view.as_ref().unwrap()) },
                BindGroupEntry { binding: 2, resource: self.quad_options.as_ref().unwrap().as_entire_binding() }
            ]
        });

        self.bind_group = Some(binding);
    }

    pub fn output_view(&mut self) {
        self.output_view = Some(self.texture.as_ref().expect("Texture shouldv been loaded").create_view(&TextureViewDescriptor {
            label: Some("EGUI output area"),
            aspect: TextureAspect::All,
            format: None,
            usage: Some(TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING),
            dimension: Some(TextureViewDimension::D2),
            array_layer_count: None,
            mip_level_count: Some(1),
            base_mip_level: 0,
            base_array_layer: 0
        }));
    }
}

pub struct EventLoopContext {
    windows: Vec<Arc<dyn Window>>,
    graphics: Option<GraphicsState>,
    ui: Option<Ui>
}

impl ApplicationHandler for EventLoopContext {
    fn resumed(&mut self, event_loop: &dyn ActiveEventLoop) {
        println!("Foreground app");
    }

    fn can_create_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {
        let mut attrs = WindowAttributes::default();
        attrs.title = String::from("FXST Devtool");
        let window: Arc<dyn Window> = Arc::from(event_loop.create_window(attrs).expect("Cant create window"));
        self.windows.push(window.clone());

        let instance = create_instance();
        let graphics = setup_graphics(&instance, window.clone()).block_on();
        self.ui = Some(Ui::start(&graphics.device).expect("UI fail"));
        self.graphics = Some(graphics);

        window.set_visible(true);
    }

    fn window_event(&mut self, event_loop: &dyn ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::RedrawRequested if let Some(gfx) = &mut self.graphics && let Some(ui) = &mut self.ui && !ui.loaded => {
                let size = Extent3d {
                    depth_or_array_layers: 1,
                    height: ui.size.1,
                    width: ui.size.0
                };

                let texture = gfx.device.create_texture(&TextureDescriptor {
                    label: Some("Logo"),
                    size,
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: TextureDimension::D2,
                    format: TextureFormat::Rgba8UnormSrgb,
                    usage: TextureUsages::RENDER_ATTACHMENT,
                    view_formats: &[],
                });

                const PIXEL_BYTES: u32 = 4;
                let bytes_per_row = PIXEL_BYTES * ui.size.0;

                // (L * W * B) / (B * W)

                // gfx.queue.write_texture(TexelCopyTextureInfo {
                //     texture: &texture,
                //     aspect: TextureAspect::All,
                //     mip_level: 0,
                //     origin: Origin3d::ZERO
                // }, &ui.logo, TexelCopyBufferLayout {
                //     offset: 0,
                //     bytes_per_row: Some(bytes_per_row),
                //     rows_per_image: Some(ui.size.1),
                // }, Extent3d {
                //     width: ui.size.0,
                //     height: ui.size.1,
                //     depth_or_array_layers: 1
                // });

                let mut encoder = gfx.device.create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("main")
                });

                let surface_texture = gfx.surface.get_current_texture().unwrap();
                let output = &surface_texture.texture;

                // encoder.copy_texture_to_texture(TexelCopyTextureInfo {
                //     texture: &texture,
                //     aspect: TextureAspect::All,
                //     mip_level: 0,
                //     origin: Origin3d::ZERO
                // }, TexelCopyTextureInfo {
                //     texture: output,
                //     aspect: TextureAspect::All,
                //     mip_level: 0,
                //     origin: Origin3d::ZERO
                // }, Extent3d {
                //     width: ui.size.0,
                //     height: ui.size.1,
                //     depth_or_array_layers: 1
                // });


                ui.loaded = true;
                ui.texture = Some(texture);
                ui.output_view();
                gfx.queue.submit([ encoder.finish() ]);
                ui.setup_vertex_buffer(&gfx.device);
                ui.setup_bind_group(&gfx.device);
                surface_texture.present();

                let window = self.windows.iter_mut().find(|win| win.id() == window_id);
                if let Some(real_window) = window {
                    real_window.request_redraw();
                    let size = real_window.surface_size();
                    ui.prepare((size.width, size.height))
                } else {
                    eprintln!("Window not stored. this is a bug");
                }
            },
            WindowEvent::RedrawRequested if let Some(gfx) = &mut self.graphics && let Some(ui) = &mut self.ui => {
                let surface_texture = gfx.surface.get_current_texture().unwrap();
                let mut encoder = gfx.device.create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("main")
                });

                let size = self.windows.iter().find(|win| win.id() == window_id).unwrap().surface_size();
                let clipped = ui.ui_clipped(&gfx.device, &gfx.queue);
                ui.buffer_clipped(&gfx.device, &gfx.queue, &mut encoder, &clipped);

                let output_view = ui.output_view.as_ref().unwrap();

                let color_attachments = RenderPassColorAttachment {
                    ops: Operations::default(),
                    depth_slice: None,
                    resolve_target: None,
                    view: output_view
                };

                let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                    label: Some("EGUI pass"),
                    color_attachments: &[ Some(color_attachments) ],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None
                })
                    .forget_lifetime();
                ui.render_wgpu(&clipped, &mut pass);
                drop(pass);

                // encoder.copy_texture_to_texture(TexelCopyTextureInfo {
                //     texture: &ui.texture.as_ref().unwrap(),
                //     aspect: TextureAspect::All,
                //     mip_level: 0,
                //     origin: Origin3d::ZERO
                // }, TexelCopyTextureInfo {
                //     texture: &surface_texture.texture,
                //     aspect: TextureAspect::All,
                //     mip_level: 0,
                //     origin: Origin3d::ZERO
                // }, Extent3d {
                //     width: ui.size.0,
                //     height: ui.size.1,
                //     depth_or_array_layers: 1
                // });

                let output_view = surface_texture.texture.create_view(&TextureViewDescriptor {
                    label: Some("Output surface"),
                    base_array_layer: 0,
                    base_mip_level: 0,
                    mip_level_count: None,
                    array_layer_count: None,
                    dimension: None,
                    usage: None,
                    format: None,
                    aspect: TextureAspect::All
                });

                let mut frame_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                    label: Some("Final output frame"),
                    depth_stencil_attachment: None,
                    color_attachments: &[ Some(RenderPassColorAttachment {
                        ops: Operations::default(),
                        resolve_target: None,
                        depth_slice: None,
                        view: &output_view
                    }) ],
                    timestamp_writes: None,
                    occlusion_query_set: None
                });

                frame_pass.set_bind_group(0, ui.bind_group.as_ref().unwrap(), &[]);
                frame_pass.draw(0..6, 0..0);

                drop(frame_pass);

                gfx.queue.submit([ encoder.finish() ]);
                surface_texture.present();
            },
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            _ => {}
        }
    }
}

impl EventLoopContext {
    pub fn new() -> Self {
        Self { windows: vec![], graphics: None, ui: None }
    }
}

pub fn start() -> Result<(), StartError> {
    let event_loop = EventLoop::new().map_err(|err| StartError::EventLoopError(err, StartErrorTask::CreateEventLoop))?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let data = EventLoopContext::new();
    event_loop.run_app(data).map_err(|err| StartError::EventLoopError(err, StartErrorTask::RunApp))?;

    Ok(())
}