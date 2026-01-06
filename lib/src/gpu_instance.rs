use wgpu::{Surface, SurfaceTarget};

use crate::GPUStarterResult;

pub struct GPUInstance {
    pub instance: wgpu::Instance,
}

impl Default for GPUInstance {
    fn default() -> Self {
        Self::new()
    }
}

impl GPUInstance {
    pub fn new() -> Self {
        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });
        Self { instance }
    }

    pub fn create_surface<'a>(
        &self,
        window: impl Into<SurfaceTarget<'a>>,
    ) -> GPUStarterResult<Surface<'a>> {
        Ok(self.instance.create_surface(window)?)
    }
}
