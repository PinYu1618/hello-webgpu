use crate::CResult;

#[derive(Debug)]
pub struct Gpu {
    /*pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,*/
}

impl Gpu {
    pub fn new() -> CResult<Self> {
        Ok(Self {})
    }
}
