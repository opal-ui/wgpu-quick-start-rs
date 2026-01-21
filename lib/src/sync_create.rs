//! This is a module that contains a synchronous function to create a new `MySurface`.
//!

use super::MyDevice;

use super::GPUStarterResult;

#[cfg(feature = "enable-sync-winit")]
use winit::window::Window;

/// This synchronous function helps create a new surface from the given window.
///
/// Available with `sync` feature. (Hence, not available by default).
///
/// Can be enabled as below in the crate when being used.
///
/// ```toml
/// wgpu_quick_start = { version="0.28.3", features = ["enable-sync-winit"] }
/// ```
///
/// ## Usage
///
/// Following is an example of using `create_new_device` from a "winit" `Window`.
///
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
pub fn create_new_device(window: Box<dyn Window>) -> GPUStarterResult<Box<dyn MyDevice>> {
    use crate::my_device::MyDeviceImpl;

    let size = window.surface_size();
    let device = pollster::block_on(MyDeviceImpl::new(window, (size.width, size.height)))?;
    Ok(Box::new(device))
}
