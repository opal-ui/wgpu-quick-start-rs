//! A module to create `wgpu::Surface` and manage the same.
//!
//!

use super::MyDevice;

use super::GPUStarterResult;

/// An abstraction of the wgpu::Surface<'lifetime>
/// to create from a given `winit` Window.
///
/// Following is an example to create the surface using a given `Window`
///
/// # Examples
///
/// ```rust
/// use wgpu_quick_start::{MyDevice, create_new_device};
/// use winit::{event::WindowEvent, window::WindowAttributes};
/// use winit_app::{AppWindowEvent, Application};
///
///
/// fn launch() -> Result<(), Box<dyn std::error::Error>> {
///    let winit_app = Application::new();
///    let mut opt_device: Option<Box<dyn MyDevice>> = None;
///    winit_app.run(
///        WindowAttributes::default().with_title("wgpu starter app"),
///        move |app_window_event| match app_window_event {
///            AppWindowEvent::NewWindow(window) => match create_new_device(window) {
///                Ok(value) => {
///                    opt_device = Some(value);
///                }
///                Err(err) => {
///                    // warning - Error creating new surface from the window
///                }
///            },
///            AppWindowEvent::OnWindowEvent(event, event_loop) => {
///                if let Some(local_device) = opt_device.as_mut() {
///                     // Handle those events
///                }
///            }
///        },
///    )?;
///    Ok(())
/// }
/// ```
///
/// # Lifetimes
///
/// The lifetime of this struct refers to the underlying lifetime of the `wgpu::Surface<'lifetime>` inside the same.
///
pub struct MyDeviceImpl<'a> {
    surface: Option<wgpu::Surface<'a>>,

    device: wgpu::Device,

    queue: wgpu::Queue,

    adapter: wgpu::Adapter,

    config: Option<wgpu::SurfaceConfiguration>,

    texture_format: wgpu::TextureFormat,
}

impl<'a> MyDeviceImpl<'a> {
    /// Create a new surface from the given window
    /// for the given dimensions (width, height) tuple
    ///
    #[cfg(feature = "enable-sync-winit")]
    pub async fn new(
        window: impl Into<wgpu::SurfaceTarget<'a>>,
        dimensions: (u32, u32),
    ) -> GPUStarterResult<Self> {
        use crate::GPUInstance;
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

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(wgpu::TextureFormat::is_srgb)
            .unwrap_or(surface_caps.formats[0]);

        Ok(Self {
            surface: Some(surface),
            device,
            queue,
            adapter,
            config: Some(config),
            texture_format: surface_format,
        })
    }

    /// Create a new device without a 'window' reference.
    /// Useful for unit testing
    #[cfg(feature = "enable-sync")]
    pub async fn new_without_window() -> crate::GPUStarterResult<Self> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
                    #[cfg(not(target_arch = "wasm32"))]
                    backends: wgpu::Backends::VULKAN,
                    #[cfg(target_arch = "wasm32")]
                    backends: wgpu::Backends::GL,
                    ..Default::default()
                });        
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await?;
        let (device, queue) = adapter.request_device(&Default::default()).await?;
        let texture_size = 256u32;

        let texture_desc = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: texture_size,
                height: texture_size,
                depth_or_array_layers: 1,
            },
            view_formats: &[],
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
        };
        let texture = device.create_texture(&texture_desc);
        let _texture_view = texture.create_view(&Default::default());
        Ok(Self {
            surface: None,
            device,
            queue,
            adapter,
            config: None,
            texture_format: texture.format(),
        })
    }
}

impl<'a> MyDevice for MyDeviceImpl<'a> {
    fn get_device(&self) -> &wgpu::Device {
        &self.device
    }

    fn get_queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    fn get_adapter(&self) -> &wgpu::Adapter {
        &self.adapter
    }

    fn get_current_texture(&self) -> GPUStarterResult<wgpu::SurfaceTexture> {
        if let Some(surface) = self.surface.as_ref() {
            let output = surface.get_current_texture()?;
            Ok(output)
        } else {
            Err(crate::GPUStarterError::UnsupportedError)
        }
    }

    fn get_texture_format(&self) -> GPUStarterResult<wgpu::TextureFormat> {
        Ok(self.texture_format)
    }

    fn resize(&mut self, dimensions: (u32, u32)) {
        // Resize the same
        let (width, height) = dimensions;
        if let Some(surface) = self.surface.as_mut()
            && let Some(config) = self.config.as_mut()
            && width > 0
            && height > 0
        {
            config.width = width;
            config.height = height;
            surface.configure(&self.device, config);
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    #[cfg(feature = "enable-sync")]
    fn test_windowless_device() -> super::super::GPUStarterResult<()> {
        use super::*;
        let my_device = pollster::block_on(MyDeviceImpl::new_without_window())?;
        assert!(my_device.get_texture_format().is_ok());
        Ok(())
    }
}
