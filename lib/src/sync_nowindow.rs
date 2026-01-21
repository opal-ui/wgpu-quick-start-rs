//! A module to create `wgpu::Device` not necessarily from a window
//!
//!

/// This synchronous function helps create a new device without any window
///
/// Available with `enable-sync` feature. (Hence, not available by default).
///
/// Can be enabled as below in the crate when being used.
///
/// ```toml
/// wgpu_quick_start = { version="0.28.3", features = ["enable-sync"] }
/// ```
///
/// ## Usage
///
/// Following is an example of using `create_new_windowless_device`.
///
///
/// ```rust
/// use wgpu_quick_start::{MyDevice, create_new_windowless_device};
///
///
/// fn launch() -> Result<(), Box<dyn std::error::Error>> {
///    let mut opt_device: Option<Box<dyn MyDevice>> = None;
///    match create_new_windowless_device() {
///        Ok(value) => {
///            opt_device = Some(value);
///        }
///        Err(err) => {
///            // warning - Error creating new device
///        }
///    }
///    Ok(())
/// }
/// ```
#[cfg(feature = "enable-sync")]
pub fn create_new_windowless_device() -> super::GPUStarterResult<Box<dyn super::MyDevice>> {
    use crate::my_device::MyDeviceImpl;

    let device = pollster::block_on(MyDeviceImpl::new_without_window())?;
    Ok(Box::new(device))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    #[cfg(feature = "enable-sync")]
    fn test_create_new_windowless_device() -> super::super::GPUStarterResult<()> {
        let my_device = create_new_windowless_device()?;
        assert!(my_device.get_texture_format().is_ok());
        Ok(())
    }
}
