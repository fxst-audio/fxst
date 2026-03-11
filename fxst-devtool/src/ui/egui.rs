use egui_wgpu::{Renderer, RendererOptions};
use wgpu::{Device, TextureFormat};

pub fn setup_egui_renderer(device: &Device) -> Renderer {
    let options = RendererOptions::default();
    let output_color_format = TextureFormat::Rgba8Uint;
    Renderer::new(device, output_color_format, options)
}