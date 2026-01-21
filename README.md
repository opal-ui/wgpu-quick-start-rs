[`wgpu_quick_start`](https://crates.io/crates/wgpu_quick_start) [![crates.io](https://img.shields.io/crates/v/wgpu_quick_start.svg)](https://crates.io/crates/wgpu_quick_start)

# Wgpu library Starter Project

```toml
[dependencies]
wgpu_quick_start = "0.28"
```

To access the synchronous version of creating `MySurface::new`, use the `sync_window` feature as below
```toml
[dependencies]
wgpu_quick_start = { version="0.28", features = ["enable-sync-winit"] }
```

This Rust library `wgpu_quick_start` represents the code to get started with `wgpu` library.

(To see more details about wgpu see here  at - https://github.com/gfx-rs/wgpu )



## Usage

```rust
use wgpu_quick_start::{MyResizableDevice, create_new_device};
use winit::{event::WindowEvent, window::WindowAttributes};
use winit_app::{AppWindowEvent, Application};


fn launch() -> Result<(), Box<dyn std::error::Error>> {
   let winit_app = Application::new();
   let mut opt_device: Option<Box<dyn MyResizableDevice>> = None;
   winit_app.run(
       WindowAttributes::default().with_title("wgpu starter app"),
       move |app_window_event| match app_window_event {
           AppWindowEvent::NewWindow(window) => match create_new_device(window) {
               Ok(value) => {
                   opt_device = Some(value);
               }
               Err(err) => {
                   // warning - Error creating new surface from the window
               }
           },
           AppWindowEvent::OnWindowEvent(event, event_loop) => {
               if let Some(local_device) = opt_device.as_mut() {
                   match event {
                       WindowEvent::CloseRequested => {
                           event_loop.exit();
                       }
                       WindowEvent::SurfaceResized(size) => {
                           // Resized
                           local_device.resize((size.width, size.height));
                       }
                       WindowEvent::RedrawRequested => {
                           match local_device.get_current_texture() {
                               Ok(output) => {
                                   let texture_view_descriptor=  wgpu::TextureViewDescriptor::default();
                                   let view = output.texture.create_view(&texture_view_descriptor);
                                   let device = local_device.get_device();
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
                                       // TODO: Render objects using render pass
                                   }
                                   let queue = local_device.get_queue();
                                   queue.submit(std::iter::once(encoder.finish()));
                                   output.present();
                               }
                               Err(err) => {
                                   // warning - error creating the texture
                               }
                           }
                       }
                       _ => {}
                   }
               }
           }
       },
   )?;
   Ok(())
}
```

## Credits

* If you want to get started with the wgpu basics, please see `learn-wgpu` . 

https://sotrh.github.io/learn-wgpu/ 

 

## Developer

### Documentation

To generate documentation for the package locally, you can use the term

```bash
$ cargo doc --no-deps
```

### Testing

To test code, including the one inside the documentation, just execute

```
$ cargo t
```
