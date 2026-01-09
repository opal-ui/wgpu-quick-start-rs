//! A module to create `wgpu::Surface` and manage the same.
//!
//!

use wgpu::SurfaceTarget;

use crate::GPUInstance;

use super::GPUStarterResult;

/// An abstraction of the wgpu::Surface<'lifetime>
/// to create from a given `winit` Window.
///
/// Following is an example to create the surface using a given `Window`
///
/// ```rust
/// use wgpu_quick_start::{my_surface::MySurface, sync_surface::create_new_surface};
/// use winit::{event::WindowEvent, window::WindowAttributes};
/// use winit_app::{app_listener::AppWindowEvent, application::Application};
///
///
/// fn launch() -> Result<(), Box<dyn std::error::Error>> {
///    let winit_app = Application::new();
///    let mut opt_surface: Option<Box<dyn MySurface>> = None;
///    winit_app.run(
///        WindowAttributes::default().with_title("wgpu starter app"),
///        move |app_window_event| match app_window_event {
///            AppWindowEvent::NewWindow(window) => match create_new_surface(window) {
///                Ok(value) => {
///                    opt_surface = Some(Box::new(value));
///                }
///                Err(err) => {
///                    // warning - Error creating new surface from the window
///                }
///            },
///            AppWindowEvent::OnWindowEvent(event, event_loop) => {
///                if let Some(local_surface) = opt_surface.as_mut() {
///                     // Handle those events
///                }
///            }
///        },
///    )?;
///    Ok(())
/// }
/// ```
pub struct MySurfaceImpl<'a> {
    surface: wgpu::Surface<'a>,

    device: wgpu::Device,

    queue: wgpu::Queue,

    adapter: wgpu::Adapter,

    config: wgpu::SurfaceConfiguration,
}

pub trait MySurface {
    /// Retrieve the device associated with this surface
    fn get_device(&self) -> &wgpu::Device;

    /// Retrieve the underlying queue present
    fn get_queue(&self) -> &wgpu::Queue;

    fn get_config(&self) -> &wgpu::SurfaceConfiguration;

    /// Reconfigure the underlying surface based on the new configuration
    fn reconfigure(&self, new_config: &wgpu::SurfaceConfiguration);

    fn resize(&mut self, size: (u32, u32));

    /// Retrieve a new texture for the current surface
    fn get_default_texture(&self) -> GPUStarterResult<(wgpu::SurfaceTexture, wgpu::TextureView)>;
}

impl<'a> MySurfaceImpl<'a> {
    /// Create a new surface from the given window
    /// for the given dimensions (width, height) tuple
    ///
    pub async fn new(
        window: impl Into<SurfaceTarget<'a>>,
        dimensions: (u32, u32),
    ) -> GPUStarterResult<Self> {
        let instance = GPUInstance::new();
        let surface = instance.create_surface(window)?;
        let adapter = instance
            .instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),

                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;

        let surface_capabilities = surface.get_capabilities(&adapter);

        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_capabilities
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_capabilities.formats[0]);
        let (width, height) = dimensions;
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width,
            height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        Ok(Self {
            surface,
            device,
            queue,
            adapter,
            config,
        })
    }
}

impl<'a> MySurface for MySurfaceImpl<'a> {
    fn get_device(&self) -> &wgpu::Device {
        &self.device
    }

    fn get_queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    fn get_config(&self) -> &wgpu::SurfaceConfiguration {
        &self.config
    }

    /// Reconfigure the surface based on updated configuration
    fn reconfigure(&self, new_config: &wgpu::SurfaceConfiguration) {
        self.surface.configure(&self.device, new_config)
    }

    fn get_default_texture(&self) -> GPUStarterResult<(wgpu::SurfaceTexture, wgpu::TextureView)> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        Ok((output, view))
    }

    fn resize(&mut self, dimensions: (u32, u32)) {
        let (width, height) = dimensions;
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
        }
    }
}
