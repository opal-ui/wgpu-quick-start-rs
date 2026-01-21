use std::sync::{Arc, RwLock};

use log::{debug, info, warn};
use wgpu_quick_start::{MyDevice, create_new_device};
use winit::{event::WindowEvent, window::WindowAttributes};
use winit_app::{AppWindowEvent, Application};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let winit_app = Application::new();
    let mut opt_device: Option<Arc<RwLock<Box<dyn MyDevice>>>> = None;
    winit_app.run(
        WindowAttributes::default().with_title("wgpu quick start app"),
        move |app_window_event| match app_window_event {
            AppWindowEvent::NewWindow(window) => match create_new_device(window) {
                Ok(value) => {
                    opt_device = Some(Arc::new(RwLock::new(value)));
                }
                Err(err) => {
                    warn!("Error creating new surface from the window {:?}", err);
                }
            },
            AppWindowEvent::OnWindowEvent(event, event_loop) => {
                if let Some(local_device) = opt_device.as_mut() {
                    match event {
                        WindowEvent::CloseRequested => {
                            event_loop.exit();
                        }
                        WindowEvent::SurfaceResized(size) => {
                            info!("Resized {:?}", size);
                            match local_device.write() {
                                Ok(mut writer) => {
                                    writer.resize((size.width, size.height));
                                }
                                Err(err) => {
                                    warn!("Unable to acquire write lock for resize {:?}", err);
                                }
                            }
                        }
                        WindowEvent::RedrawRequested => match local_device.read() {
                            Ok(reader) => match reader.get_current_texture() {
                                Ok(output) => {
                                    let texture_view_descriptor =
                                        wgpu::TextureViewDescriptor::default();
                                    let view = output.texture.create_view(&texture_view_descriptor);
                                    let device = reader.get_device();
                                    let mut encoder = device.create_command_encoder(
                                        &wgpu::CommandEncoderDescriptor {
                                            label: Some("Render Encoder"),
                                        },
                                    );
                                    {
                                        let _render_pass =
                                            wgpu_quick_start::create_default_render_pass(
                                                &mut encoder,
                                                "root-render-pass".to_owned(),
                                                wgpu::Color {
                                                    r: 0.9,
                                                    g: 0.9,
                                                    b: 0.9,
                                                    a: 1.0,
                                                },
                                                &view,
                                            );
                                    }
                                    let queue = reader.get_queue();
                                    queue.submit(std::iter::once(encoder.finish()));
                                    output.present();
                                }
                                Err(err) => {
                                    warn!("Error retrieving current texture {:?}", err);
                                }
                            },
                            Err(err) => {
                                warn!("Error acquiring read lock {:?}", err);
                            }
                        },
                        _ => {
                            debug!("Unhandled event {:?}", event);
                        }
                    }
                } else {
                    warn!("No Window set. Ignoring the window event hence");
                }
            }
        },
    )?;
    Ok(())
}
