use std::sync::Arc;
use wgpu::{Adapter, BackendOptions, Backends, Device, DeviceDescriptor, ExperimentalFeatures, Features, Instance, InstanceDescriptor, InstanceFlags, Limits, MemoryBudgetThresholds, MemoryHints, PowerPreference, Queue, RequestAdapterOptions, Surface, Trace};
use winit::window::Window;

pub struct GraphicsState {
    surface: Surface<'static>,
    device: Device,
    queue: Queue
}

pub fn create_instance() -> Instance {
    let backend_options = BackendOptions::default();

    let desc = InstanceDescriptor {
        backend_options,
        flags: InstanceFlags::DEBUG,
        backends: Backends::all(),
        memory_budget_thresholds: MemoryBudgetThresholds::default()
    };

    Instance::new(&desc)
}

pub async fn get_adapter(instance: &Instance, surface: &Surface<'_>) -> Adapter {
    let options = RequestAdapterOptions {
        compatible_surface: Some(surface),
        force_fallback_adapter: true,
        power_preference: PowerPreference::HighPerformance
    };

    instance.request_adapter(&options)
        .await
        .expect("Cant get adapter")
}

pub async fn get_device(adapter: &Adapter) -> (Device, Queue) {
    let desc = DeviceDescriptor {
        label: Some("unnamed device"),
        experimental_features: ExperimentalFeatures::disabled(),
        memory_hints: MemoryHints::Performance,
        required_features: Features::empty(),
        required_limits: Limits::default(),
        trace: Trace::Off
    };

    adapter.request_device(&desc)
        .await
        .expect("Cant get device")
}

pub async fn setup_graphics(instance: &Instance, window: Arc<Window>) -> GraphicsState {
    let surface = instance.create_surface(window).expect("Cant create surface");
    let adapter = get_adapter(instance, &surface).await;
    let (device, queue) = get_device(&adapter).await;

    GraphicsState {
        surface,
        device,
        queue
    }
}