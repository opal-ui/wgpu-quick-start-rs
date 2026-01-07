use wgpu::{Surface, SurfaceTarget};

use crate::GPUStarterResult;

/// Abstraction of `wgpu::Instance`
///
///
///
/// ## Usage
///
/// To create a new instance of the same, use the following example.
///
/// ```rust
/// use wgpu_quick_start::gpu_instance::GPUInstance;
/// fn create_new_gpu_instance() {
///    let instance = GPUInstance::new();
/// }
/// ```
///
pub struct GPUInstance {
    pub instance: wgpu::Instance,
}

impl Default for GPUInstance {
    fn default() -> Self {
        Self::new()
    }
}

impl GPUInstance {
    /// Create a new instance based on the platform
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
