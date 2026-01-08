//! This is a module that contains a synchronous function to create a new `MySurface`.
//!
//! Available with `sync` feature. (not the default one)
//!
//! Can be enabled in `dependencies` in `Cargo.toml` as below.
//!
//! ```toml
//! wgpu_quick_start = { version="0.28.3", features = ["sync"] }
//! ```
//!
//!
//! ## Usage
//!
//! Following is an example of using `create_new_surface` from a "winit" `Window`.
//!
//!
//! ```rust
//! use wgpu_quick_start::{my_surface::MySurface, sync_surface::create_new_surface};
//! use winit::{event::WindowEvent, window::WindowAttributes};
//! use winit_app::{app_listener::AppWindowEvent, application::Application};
//!
//!
//! fn launch() -> Result<(), Box<dyn std::error::Error>> {
//!    let winit_app = Application::new();
//!    let mut opt_surface: Option<Box<MySurface<'_>>> = None;
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
//!                     // Handle those events
//!                }
//!            }
//!        },
//!    )?;
//!    Ok(())
//! }
//! ```
use super::GPUStarterResult;
use super::my_surface::MySurface;
use log::info;
use winit::window::Window;

/// This synchronous function helps create a new surface from the given window.
///
/// Available with `sync` feature. (Hence, not available by default).
///
/// Can be enabled as below in the crate when being used.
///
/// ```toml
/// wgpu_quick_start = { version="0.28.3", features = ["sync"] }
/// ```
///
/// ## Usage
///
/// Following is an example of using `create_new_surface` from a "winit" `Window`.
///
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
#[cfg(feature = "sync")]
pub fn create_new_surface<'a>(window: Box<dyn Window>) -> GPUStarterResult<MySurface<'a>> {
    let size = window.surface_size();
    let surface = pollster::block_on(MySurface::new(window, (size.width, size.height)))?;
    info!("Created a new surface context. Passing down");
    Ok(surface)
}
