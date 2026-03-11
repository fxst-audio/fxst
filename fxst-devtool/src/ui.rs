pub mod error;
pub mod wgpu;

use std::io::Cursor;
use std::ops::Deref;
use std::sync::Arc;
use ::wgpu::{BufferAddress, Extent3d, Origin3d, TexelCopyBufferLayout, TexelCopyTextureInfo, TextureAspect, TextureDescriptor};
use image::ImageReader;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::platform::windows::EventLoopBuilderExtWindows;
use winit::window::{Window, WindowAttributes, WindowId};
use crate::ui::error::{StartError, StartErrorTask};
use crate::ui::wgpu::{create_instance, setup_graphics, GraphicsState};

struct Ui {
    logo: Vec<u8>,
    size: (u32, u32),
    loaded: bool,
    texture: Option<Texture>
}

impl Ui {
    pub fn start() -> Result<Self, ()> {
        let reader = ImageReader::open(r"C:\Users\Tetra\Downloads\download.jpg") // TODO: error handling needed
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

        let ui = Self {
            size: (reader.width(), reader.height()),
            logo: reader.to_vec(),
            loaded: false
        };
        Ok(ui)
    }
}

pub struct EventLoopContext {
    windows: Vec<Arc<dyn Window>>,
    graphics: Option<GraphicsState>,
    ui: Ui
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
        self.graphics = Some(pollster::block_on(setup_graphics(&instance, window.clone())));

        window.set_visible(true);
    }

    fn window_event(&mut self, event_loop: &dyn ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::RedrawRequested if let Some(gfx) = &mut self.graphics && !self.ui.loaded => {
                let texture_info =
                let texture = gfx.device.create_texture(&TextureDescriptor {

                });

                const PIXEL_BYTES: u32 = 4;
                let bytes_per_row = PIXEL_BYTES * self.ui.size.0;
                let total_bytes = self.ui.size.0 * self.ui.size.1 * PIXEL_BYTES;

                gfx.queue.write_texture(TexelCopyTextureInfo {
                    texture,
                    aspect: TextureAspect::All,
                    mip_level: 0,
                    origin: Origin3d::ZERO
                }, &self.ui.logo, TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(bytes_per_row),
                    rows_per_image: Some(total_bytes / bytes_per_row)
                }, Extent3d {
                    depth_or_array_layers: 1,
                    height: self.ui.size.1,
                    width: self.ui.size.0
                });

                self.ui.loaded = true;
                gfx.queue.submit([]);
                let surface_texture = gfx.surface.get_current_texture().unwrap();
                surface_texture.present();
            },
            WindowEvent::RedrawRequested if let Some(gfx) = &mut self.graphics => {
                let surface_texture = gfx.surface.get_current_texture().unwrap();
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
        Self { windows: vec![], graphics: None, ui: Ui::start().expect("UI fail") }
    }
}

pub fn start() -> Result<(), StartError> {
    let event_loop = EventLoop::new().map_err(|err| StartError::EventLoopError(err, StartErrorTask::CreateEventLoop))?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let data = EventLoopContext::new();
    event_loop.run_app(data).map_err(|err| StartError::EventLoopError(err, StartErrorTask::RunApp))?;

    Ok(())
}