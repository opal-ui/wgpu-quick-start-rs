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
#[cfg(feature = "sync")]
pub fn create_new_surface<'a>(window: Box<dyn Window>) -> GPUStarterResult<MySurface<'a>> {
    let size = window.surface_size();
    let surface = pollster::block_on(MySurface::new(window, (size.width, size.height)))?;
    info!("Created a new surface context. Passing down");
    Ok(surface)
}
