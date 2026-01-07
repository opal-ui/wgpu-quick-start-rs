[`wgpu_quick_start`](https://crates.io/crates/wgpu_quick_start) [![crates.io](https://img.shields.io/crates/v/wgpu_quick_start.svg)](https://crates.io/crates/wgpu_quick_start)

# Wgpu library Starter Project

```toml
[dependencies]
wgpu_quick_start = "0.28.5"
```

To access the synchronous version of creating `MySurface::new`, use the `sync` feature as below
```toml
[dependencies]
wgpu_quick_start = { version="0.28.5", features = ["sync"] }
```

This exposes a new `fn create_new_surface()` to create a `MySurface` synchronously (using the crate `pollster` behind the scenes).

This Rust library `wgpu_quick_start` represents the code to get started with `wgpu` library.

(To see more details about wgpu see here  at - https://github.com/gfx-rs/wgpu )



## Usage

```rust

use wgpu_quick_start::{my_surface::MySurface, sync_surface::create_new_surface};
use winit::{event::WindowEvent, window::WindowAttributes};
use winit_app::{app_listener::AppWindowEvent, application::Application};


fn launch() -> Result<(), Box<dyn std::error::Error>> {
   let winit_app = Application::new();
   let mut opt_surface: Option<Box<MySurface<'_>>> = None;
   winit_app.run(
       WindowAttributes::default().with_title("wgpu starter app"),
       move |app_window_event| match app_window_event {
           AppWindowEvent::NewWindow(window) => match create_new_surface(window) {
               Ok(value) => {
                   let boxed = Box::new(value);
                   opt_surface = Some(boxed);
               }
               Err(err) => {
                   // warning - Error creating new surface from the window
               }
           },
           AppWindowEvent::OnWindowEvent(event, event_loop) => {
               if let Some(local_surface) = opt_surface.as_mut() {
                   match event {
                       WindowEvent::CloseRequested => {
                           event_loop.exit();
                       }
                       WindowEvent::SurfaceResized(size) => {
                           // Resized
                           local_surface.resize((size.width, size.height));
                       }
                       WindowEvent::RedrawRequested => {
                           match local_surface.get_texture() {
                               Ok((output, view)) => {
                                    let mut encoder = local_surface.on_device(|device| {
                                       device.create_command_encoder(
                                           &wgpu::CommandEncoderDescriptor {
                                               label: Some("Render Encoder"),
                                           },
                                       )
                                   });
                                   {
                                       let _render_pass = encoder.begin_render_pass(
                                           &wgpu::RenderPassDescriptor {
                                               label: Some("render-pass"),
                                               color_attachments: &[Some(
                                                   wgpu::RenderPassColorAttachment {
                                                       view: &view,
                                                       resolve_target: None,
                                                       ops: wgpu::Operations {
                                                           load: wgpu::LoadOp::Clear(
                                                               wgpu::Color {
                                                                   r: 0.2,
                                                                   g: 0.2,
                                                                   b: 0.2,
                                                                   a: 1.0,
                                                               },
                                                           ),
                                                           store: wgpu::StoreOp::Store,
                                                       },
                                                       depth_slice: None,
                                                   },
                                               )],
                                               depth_stencil_attachment: None,
                                               occlusion_query_set: None,
                                               timestamp_writes: None,
                                               multiview_mask: None,
                                           },
                                       );
                                   }
                                   local_surface.submit_to_queue(std::iter::once(encoder.finish()));
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

## Developers



### Testing

To test code, including the one inside the documentation, just execute

```
$ cargo t --features sync
```


## Credits

* If you want to get started with the wgpu basics, please see `learn-wgpu` . 

https://sotrh.github.io/learn-wgpu/ 

 