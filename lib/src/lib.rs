//!
//!
//! A starter crate to get started with wgpu
//!
//! ```rust
//! use wgpu_quick_start::{my_surface::MySurface, sync_surface::create_new_surface};
//! use winit::{event::WindowEvent, window::WindowAttributes};
//! use winit_app::{app_listener::AppWindowEvent, application::Application};
//!
//!
//! fn launch() -> Result<(), Box<dyn std::error::Error>> {
//!    let winit_app = Application::new();
//!    let mut opt_surface: Option<Box<dyn MySurface>> = None;
//!    winit_app.run(
//!        WindowAttributes::default().with_title("wgpu starter app"),
//!        move |app_window_event| match app_window_event {
//!            AppWindowEvent::NewWindow(window) => match create_new_surface(window) {
//!                Ok(value) => {
//!                    opt_surface = Some(Box::new(value));
//!                }
//!                Err(err) => {
//!                    // warning - Error creating new surface from the window
//!                }
//!            },
//!            AppWindowEvent::OnWindowEvent(event, event_loop) => {
//!                if let Some(local_surface) = opt_surface.as_mut() {
//!                    match event {
//!                        WindowEvent::CloseRequested => {
//!                            event_loop.exit();
//!                        }
//!                        WindowEvent::SurfaceResized(size) => {
//!                            // Resized
//!                            local_surface.resize((size.width, size.height));
//!                        }
//!                        WindowEvent::RedrawRequested => {
//!                            match local_surface.get_default_texture() {
//!                                Ok((output, view)) => {
//!                                    let device = local_surface.get_device();
//!                                    let mut encoder = device.create_command_encoder(
//!                                         &wgpu::CommandEncoderDescriptor {
//!                                             label: Some("Render Encoder"),
//!                                         },
//!                                    );
//!                                    {
//!                                     let _render_pass =
//!                                        wgpu_quick_start::render_pass_factory::create_render_pass(
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
//!                                    let queue = local_surface.get_queue();
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
pub mod my_shader;
pub mod my_surface;
pub mod render_pass_factory;

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
