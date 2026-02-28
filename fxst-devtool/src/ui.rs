pub mod error;

use std::sync::Arc;
use wgpu::{BackendOptions, Backends, Instance, InstanceDescriptor, InstanceFlags, MemoryBudgetThresholds, Surface};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::platform::windows::EventLoopBuilderExtWindows;
use winit::window::{Window, WindowAttributes, WindowId};
use crate::ui::error::{StartError, StartErrorTask};

pub struct EventLoopContext {
    windows: Vec<Arc<Window>>,
    surface: Surface
}

fn create_instance() -> Instance {
    let backend_options = BackendOptions::default();

    let desc = InstanceDescriptor {
        backend_options,
        flags: InstanceFlags::DEBUG,
        backends: Backends::VULKAN,
        memory_budget_thresholds: MemoryBudgetThresholds::default()
    };

    Instance::new(&desc)
}

impl ApplicationHandler for EventLoopContext {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut attrs = WindowAttributes::default();
        attrs.title = String::from("FXST Devtool");
        let window = Arc::new(event_loop.create_window(attrs).expect("Cant create window"));

        let instance = create_instance();
        let surface = instance.create_surface(window.clone()).expect("Cant create surface");

        window.set_visible(true);
        self.windows.push(window.clone());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {

    }
}

impl EventLoopContext {
    pub fn new() -> Self {
        EventLoopContext { windows: vec![] }
    }
}

pub fn start() -> Result<(), StartError> {
    let event_loop = EventLoop::new().map_err(|err| StartError::EventLoopError(err, StartErrorTask::CreateEventLoop))?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut data = EventLoopContext::new();
    event_loop.run_app(&mut data).map_err(|err| StartError::EventLoopError(err, StartErrorTask::RunApp))?;

    Ok(())
}