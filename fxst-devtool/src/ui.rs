pub mod error;
pub mod wgpu;

use std::io::Cursor;
use std::ops::Deref;
use std::sync::Arc;
use image::ImageReader;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::platform::windows::EventLoopBuilderExtWindows;
use winit::window::{Window, WindowAttributes, WindowId};
use crate::ui::error::{StartError, StartErrorTask};
use crate::ui::wgpu::{create_instance, setup_graphics, GraphicsState};

struct Ui {
    logo: Vec<u8>
}

impl Ui {
    pub fn start() -> Result<Self, ()> {
        let ui = Self {
            logo: ImageReader::open(r"C:\Users\rayya\Downloads\download.jpg") // TODO: error handling needed
                .map_err(|err| ())?
                .decode()
                .map_err(|err| ())?
                .into_bytes()
        };
        Ok(ui)
    }
}

pub struct EventLoopContext {
    windows: Vec<Arc<dyn Window>>,
    graphics: Option<GraphicsState>
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
            WindowEvent::RedrawRequested if let Some(gfx) = &mut self.graphics => {
                 let output = gfx.surface.get_current_texture().unwrap();
                gfx.d
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
        Self { windows: vec![], graphics: None }
    }
}

pub fn start() -> Result<(), StartError> {
    let event_loop = EventLoop::new().map_err(|err| StartError::EventLoopError(err, StartErrorTask::CreateEventLoop))?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let data = EventLoopContext::new();
    event_loop.run_app(data).map_err(|err| StartError::EventLoopError(err, StartErrorTask::RunApp))?;

    Ok(())
}