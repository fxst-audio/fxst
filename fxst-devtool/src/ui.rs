pub mod error;
pub mod wgpu;

use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::platform::windows::EventLoopBuilderExtWindows;
use winit::window::{Window, WindowAttributes, WindowId};
use crate::ui::error::{StartError, StartErrorTask};
use crate::ui::wgpu::{create_instance, setup_graphics, GraphicsState};

pub struct EventLoopContext {
    windows: Vec<Arc<Window>>,
    graphics: Option<GraphicsState>
}

impl ApplicationHandler for EventLoopContext {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut attrs = WindowAttributes::default();
        attrs.title = String::from("FXST Devtool");
        let window = Arc::new(event_loop.create_window(attrs).expect("Cant create window"));

        let instance = create_instance();
        self.graphics = Some(pollster::block_on(setup_graphics(&instance, window.clone())));

        window.set_visible(true);
        self.windows.push(window.clone());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {

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

    let mut data = EventLoopContext::new();
    event_loop.run_app(&mut data).map_err(|err| StartError::EventLoopError(err, StartErrorTask::RunApp))?;

    Ok(())
}