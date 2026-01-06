pub mod gpu_instance;
pub mod my_shader;
pub mod my_surface;

#[cfg(feature = "sync")]
pub mod sync_surface;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GPUStarterError {
    #[error(transparent)]
    SurfaceError(#[from] wgpu::SurfaceError),
    #[error(transparent)]
    CreateSurfaceError(#[from] wgpu::CreateSurfaceError),
    #[error(transparent)]
    RequestAdapterError(#[from] wgpu::RequestAdapterError),
    #[error(transparent)]
    RequestDeviceError(#[from] wgpu::RequestDeviceError),
}

pub type GPUStarterResult<T> = Result<T, GPUStarterError>;
