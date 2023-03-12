use lazy_static::*;
use parking_lot::Mutex;
use winit::{event_loop::EventLoop, window::Window};

lazy_static! {
    pub static ref PLATFORM: Mutex<Platform> = Mutex::new(Platform {
        context_wrapper: None,
        wgpu: None,
    });
}

/// Defines the WGPU platform
pub struct Platform {
    /// Wrapper for the winit context
    pub context_wrapper: Option<WrappedContext>,
    /// Contains the WGPU back-end (device, etc.)
    pub wgpu: Option<Wgpu>,
}

unsafe impl Send for Platform {}
unsafe impl Sync for Platform {}

pub struct Wgpu {
    pub instance: wgpu::Instance,
    pub surface: wgpu::Surface,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
}

pub struct WrappedContext {
    pub el: EventLoop<()>,
    pub window: Window,
}
