//!
//!
//!
//!
//! A starter crate to get started with wgpu
//!
//! # Features
#![cfg_attr(doc, doc = document_features::document_features!())]
//!
//!
//! # Examples
//!
//! An example to launch a `winit` window and use this library to get started.
//!
//! ```rust
//! use wgpu_quick_start::{MyDevice};
//!
//! use wgpu_quick_start::create_new_device;
//! use winit::{event::WindowEvent, window::WindowAttributes};
//! use winit_app::{AppWindowEvent, Application};
//!
//!
//! fn launch() -> Result<(), Box<dyn std::error::Error>> {
//!    let winit_app = Application::new();
//!    let mut opt_device: Option<Box<dyn MyDevice>> = None;
//!    winit_app.run(
//!        WindowAttributes::default().with_title("wgpu starter app"),
//!        move |app_window_event| match app_window_event {
//!            AppWindowEvent::NewWindow(window) => match create_new_device(window) {
//!                Ok(value) => {
//!                    opt_device = Some(value);
//!                }
//!                Err(err) => {
//!                    // warning - Error creating new surface from the window
//!                }
//!            },
//!            AppWindowEvent::OnWindowEvent(event, event_loop) => {
//!                if let Some(local_device) = opt_device.as_mut() {
//!                    match event {
//!                        WindowEvent::CloseRequested => {
//!                            event_loop.exit();
//!                        }
//!                        WindowEvent::SurfaceResized(size) => {
//!                            // Resized
//!                            local_device.resize((size.width, size.height));
//!                        }
//!                        WindowEvent::RedrawRequested => {
//!                            match local_device.get_current_texture() {
//!                                Ok(output) => {
//!                                    let texture_view_descriptor=  wgpu::TextureViewDescriptor::default();
//!                                    let view = output.texture.create_view(&texture_view_descriptor);
//!                                    let device = local_device.get_device();
//!                                    let mut encoder = device.create_command_encoder(
//!                                         &wgpu::CommandEncoderDescriptor {
//!                                             label: Some("Render Encoder"),
//!                                         },
//!                                    );
//!                                    {
//!                                     let _render_pass =
//!                                        wgpu_quick_start::create_default_render_pass(
//!                                            &mut encoder,
//!                                            "root-render-pass".to_owned(),
//!                                            wgpu::Color {
//!                                                r: 0.9,
//!                                                g: 0.9,
//!                                                b: 0.9,
//!                                                a: 1.0,
//!                                            },
//!                                            &view,
//!                                        );
//!                                        // TODO: Render objects using render pass
//!                                    }
//!                                    let queue = local_device.get_queue();
//!                                    queue.submit(std::iter::once(encoder.finish()));
//!                                    output.present();
//!                                }
//!                                Err(err) => {
//!                                    // warning - error creating the texture
//!                                }
//!                            }
//!                        }
//!                        _ => {}
//!                    }
//!                }
//!            }
//!        },
//!    )?;
//!    Ok(())
//! }
//! ```
//!

mod my_render_pipeline;
pub use my_render_pipeline::{
    MyRenderPipelineDescriptor, create_default_pipeline_layout, create_default_render_pipeline,
};

mod my_shader;
pub use my_shader::create_shader;

mod render_pass_factory;
pub use render_pass_factory::create_default_render_pass;

mod my_device;
pub use my_device::MyDeviceImpl;

mod buffers;
pub use buffers::{create_index_buffer, create_vertex_buffer};

#[cfg(feature = "enable-sync-winit")]
mod sync_create;

#[cfg(feature = "enable-sync-winit")]
pub use sync_create::create_new_device;

#[cfg(feature = "enable-sync")]
mod sync_nowindow;

#[cfg(feature = "enable-sync")]
pub use sync_nowindow::create_new_windowless_device;

use thiserror::Error;

/// GPUStarterError indicates the kind of error thrown by the `wgpu-quick-start-rs` project
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
    #[error("Unsupported error")]
    UnsupportedError,
}

/// Result type that goes with error type [`GPUStarterError`]
pub type GPUStarterResult<T> = Result<T, GPUStarterError>;

/// GPUInstance is a abstraction of [`wgpu::Instance`].
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
        window: impl Into<wgpu::SurfaceTarget<'a>>,
    ) -> GPUStarterResult<wgpu::Surface<'a>> {
        Ok(self.instance.create_surface(window)?)
    }
}

/// Trait as an abstraction of the underlying device
pub trait MyDevice: Send + Sync {
    /// Retrieve the adapter
    fn get_adapter(&self) -> &wgpu::Adapter;

    /// Retrieve the device associated with this surface
    fn get_device(&self) -> &wgpu::Device;

    /// Retrieve the underlying queue present
    fn get_queue(&self) -> &wgpu::Queue;

    /// Retrieve the current texture
    fn get_current_texture(&self) -> GPUStarterResult<wgpu::SurfaceTexture>;

    /// Retrieve the texture format
    fn get_texture_format(&self) -> GPUStarterResult<wgpu::TextureFormat>;

    /// Resize the window
    fn resize(&mut self, sizes: (u32, u32));
}
