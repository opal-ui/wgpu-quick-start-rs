use wgpu::SurfaceTarget;

use super::GPUStarterResult;
use super::gpu_instance::GPUInstance;

/// An abstraction of the wgpu::Surface<'lifetime>
/// that gets created.
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
///                    match event {
///                        WindowEvent::CloseRequested => {
///                            event_loop.exit();
///                        }
///                        WindowEvent::SurfaceResized(size) => {
///                            // Resized
///                            local_surface.resize((size.width, size.height));
///                        }
///                        WindowEvent::RedrawRequested => {
///                            match local_surface.get_texture() {
///                                Ok((output, view)) => {
///                                     let mut encoder = local_surface.on_device(|device| {
///                                        device.create_command_encoder(
///                                            &wgpu::CommandEncoderDescriptor {
///                                                label: Some("Render Encoder"),
///                                            },
///                                        )
///                                    });
///                                    {
///                                        let _render_pass = encoder.begin_render_pass(
///                                            &wgpu::RenderPassDescriptor {
///                                                label: Some("render-pass"),
///                                                color_attachments: &[Some(
///                                                    wgpu::RenderPassColorAttachment {
///                                                        view: &view,
///                                                        resolve_target: None,
///                                                        ops: wgpu::Operations {
///                                                            load: wgpu::LoadOp::Clear(
///                                                                wgpu::Color {
///                                                                    r: 0.2,
///                                                                    g: 0.2,
///                                                                    b: 0.2,
///                                                                    a: 1.0,
///                                                                },
///                                                            ),
///                                                            store: wgpu::StoreOp::Store,
///                                                        },
///                                                        depth_slice: None,
///                                                    },
///                                                )],
///                                                depth_stencil_attachment: None,
///                                                occlusion_query_set: None,
///                                                timestamp_writes: None,
///                                                multiview_mask: None,
///                                            },
///                                        );
///                                    }
///                                    local_surface.submit_queue(encoder);
///                                    output.present();
///                                }
///                                Err(err) => {
///                                    // warning - error creating the texture
///                                }
///                            }
///                        }
///                        _ => {}
///                    }
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

    pub fn on_device<R>(&self, fn_on_device: fn(&wgpu::Device) -> R) -> R {
        (fn_on_device)(&self.device)
    }

    /// submit the operations on the encoder to the queue created internally.
    ///
    /// Queue submission is quite expensive. Should not be used frequently.
    /// But, rather batched at the end !
    pub fn submit_queue(&self, encoder: wgpu::CommandEncoder) {
        self.queue.submit(std::iter::once(encoder.finish()));
    }
}
