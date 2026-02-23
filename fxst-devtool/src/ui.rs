pub mod error;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::platform::windows::EventLoopBuilderExtWindows;
use winit::window::{Window, WindowId};
use crate::ui::error::{StartError, StartErrorTask};

pub struct EventLoopData {

}

impl ApplicationHandler for EventLoopData {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        todo!()
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        todo!()
    }
}

impl EventLoopData {
    pub fn new() -> Self {
        EventLoopData {}
    }
}

fn start() -> Result<(), StartError> {
    let event_loop = EventLoop::new().map_err(|err| StartError::EventLoopError(err, StartErrorTask::CreateEventLoop))?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut data = EventLoopData::new();
    event_loop.run_app(&mut data).map_err(|err| StartError::EventLoopError(err, StartErrorTask::RunApp))?;

    Ok(())
}