use super::GPUStarterResult;
use super::my_surface::MySurface;
use log::info;
use winit::window::Window;

#[cfg(feature = "sync")]
pub fn create_new_surface<'a>(window: Box<dyn Window>) -> GPUStarterResult<MySurface<'a>> {
    let size = window.surface_size();
    let surface = pollster::block_on(MySurface::new(window, (size.width, size.height)))?;
    info!("Created a new surface context. Passing down");
    Ok(surface)
}
