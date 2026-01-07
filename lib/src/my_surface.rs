//! A module to create `wgpu::Surface` and manage the same.
//! 
//! 

use wgpu::{CommandBuffer, SubmissionIndex, SurfaceTarget};

use super::GPUStarterResult;
use super::gpu_instance::GPUInstance;

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
///    let mut opt_surface: Option<Box<MySurface<'_>>> = None;
///    winit_app.run(
///        WindowAttributes::default().with_title("wgpu starter app"),
///        move |app_window_event| match app_window_event {
///            AppWindowEvent::NewWindow(window) => match create_new_surface(window) {
///                Ok(value) => {
///                    let boxed = Box::new(value);
///                    opt_surface = Some(boxed);
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
pub struct MySurface<'a> {
    config: wgpu::SurfaceConfiguration,

    surface: wgpu::Surface<'a>,

    device: wgpu::Device,

    queue: wgpu::Queue,

    adapter: wgpu::Adapter,
}

impl<'a> MySurface<'a> {
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
        //
        Ok(Self {
            surface,
            config,
            device,
            queue,
            adapter,
        })
    }

    /// Resizes the given surface with the given dimensions
    pub fn resize(&mut self, (width, height): (u32, u32)) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    /// Retrieve the current size of the underlying configuration
    pub fn get_size(&self) -> (u32, u32) {
        (self.config.width, self.config.height)
    }

    /// Reconfigure the surface based on updated configuration
    pub fn reconfigure(&self, new_config: &wgpu::SurfaceConfiguration) {
        self.surface.configure(&self.device, new_config)
    }

    /// Retrieve a new texture for the current surface
    pub fn get_texture(&self) -> GPUStarterResult<(wgpu::SurfaceTexture, wgpu::TextureView)> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        Ok((output, view))
    }

    /// Execute a function on the `device` reference stored internally
    pub fn on_device_mut<R>(&self, mut fn_on_device: impl FnMut(&wgpu::Device) -> R) -> R {
        (fn_on_device)(&self.device)
    }

    /// Execute a function on the `device` reference stored internally
    pub fn on_device<R>(&self, fn_on_device: impl Fn(&wgpu::Device) -> R) -> R {
        (fn_on_device)(&self.device)
    }

    /// Execute a function on 'device' and texture stored internally
    pub fn on_device_and_texture_mut<R>(
        &self,
        fn_device_and_texture: impl Fn(&wgpu::Device, wgpu::TextureFormat) -> R,
    ) -> R {
        (fn_device_and_texture)(&self.device, self.config.format)
    }

    /// Execute a function on 'device' and texture stored internally    
    pub fn on_device_and_texture<R>(
        &self,
        mut fn_device_and_texture: impl FnMut(&wgpu::Device, wgpu::TextureFormat) -> R,
    ) -> R {
        (fn_device_and_texture)(&self.device, self.config.format)
    }

    /// Execute a function on the `queue` reference stored internally
    pub fn on_queue_mut<R>(&self, mut fn_on_queue: impl FnMut(&wgpu::Queue) -> R) -> R {
        (fn_on_queue)(&self.queue)
    }

    /// Execute a function on the `queue` reference stored internally
    pub fn on_queue<R>(&self, fn_on_queue: impl Fn(&wgpu::Queue) -> R) -> R {
        (fn_on_queue)(&self.queue)
    }

    /// Execute a function on the `adapter` reference stored internally
    pub fn on_adapter_mut<R>(&self, mut fn_on_adapter: impl FnMut(&wgpu::Adapter) -> R) -> R {
        (fn_on_adapter)(&self.adapter)
    }

    /// Execute a function on the `adapter` reference stored internally
    pub fn on_adapter<R>(&self, fn_on_adapter: impl Fn(&wgpu::Adapter) -> R) -> R {
        (fn_on_adapter)(&self.adapter)
    }

    /// submit the operations on the encoder to the queue created internally.
    ///
    /// Queue submission is quite expensive. Should not be used frequently.
    /// But, rather batched at the end !
    pub fn submit_to_queue<I>(&self, command_buffer: I) -> SubmissionIndex
    where
        I: IntoIterator<Item = CommandBuffer>,
    {
        self.queue.submit(command_buffer)
    }
}
