use thiserror::Error;
use winit::error::EventLoopError;

#[derive(Debug)]
pub enum StartErrorTask {
    CreateEventLoop,
    RunApp
}

#[derive(Debug, Error)]
pub enum StartError {
    #[error("the event loop cannot continue to run")]
    EventLoopError(EventLoopError, StartErrorTask)
}