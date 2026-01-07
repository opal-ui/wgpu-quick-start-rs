use log::{info, warn};
use wgpu_quick_start::{my_surface::MySurface, sync_surface::create_new_surface};
use winit::{event::WindowEvent, window::WindowAttributes};
use winit_app::{app_listener::AppWindowEvent, application::Application};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let winit_app = Application::new();
    let mut opt_surface: Option<Box<MySurface<'_>>> = None;
    winit_app.run(
        WindowAttributes::default().with_title("wgpu quick start app"),
        move |app_window_event| match app_window_event {
            AppWindowEvent::NewWindow(window) => match create_new_surface(window) {
                Ok(value) => {
                    let boxed = Box::new(value);
                    opt_surface = Some(boxed);
                }
                Err(err) => {
                    warn!("Error creating new surface from the window {:?}", err);
                }
            },
            AppWindowEvent::OnWindowEvent(event, event_loop) => {
                if let Some(local_surface) = opt_surface.as_mut() {
                    match event {
                        WindowEvent::CloseRequested => {
                            event_loop.exit();
                        }
                        WindowEvent::SurfaceResized(size) => {
                            info!("Resized {:?}", size);
                            local_surface.resize((size.width, size.height));
                        }
                        WindowEvent::RedrawRequested => match local_surface.get_texture() {
                            Ok((output, view)) => {
                                let mut encoder = local_surface.on_device(|device| {
                                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                        label: Some("Render Encoder"),
                                    })
                                });
                                {
                                    let _render_pass =
                                        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                            label: Some("render-pass"),
                                            color_attachments: &[Some(
                                                wgpu::RenderPassColorAttachment {
                                                    view: &view,
                                                    resolve_target: None,
                                                    ops: wgpu::Operations {
                                                        load: wgpu::LoadOp::Clear(wgpu::Color {
                                                            r: 0.9,
                                                            g: 0.9,
                                                            b: 0.9,
                                                            a: 1.0,
                                                        }),
                                                        store: wgpu::StoreOp::Store,
                                                    },
                                                    depth_slice: None,
                                                },
                                            )],
                                            depth_stencil_attachment: None,
                                            occlusion_query_set: None,
                                            timestamp_writes: None,
                                            multiview_mask: None,
                                        });
                                }
                                local_surface.submit_to_queue(std::iter::once(encoder.finish()));
                                output.present();
                            }
                            Err(err) => {
                                warn!("Error retrieving current texture {:?}", err);
                            }
                        },
                        _ => {}
                    }
                } else {
                    warn!("No Window set. Ignoring the window event hence");
                }
            }
        },
    )?;
    Ok(())
}
