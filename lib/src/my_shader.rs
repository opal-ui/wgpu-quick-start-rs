//! Module to create shaders and render pipelines to be used for rendering shapes and objects
//!
//!

use std::borrow::Cow;

/// create a shader from the given source, with the given label
pub fn create_shader<'b>(
    device: &wgpu::Device,
    label: String,
    source: Cow<'b, str>,
) -> wgpu::ShaderModule {
    let descriptor = wgpu::ShaderModuleDescriptor {
        label: Some(label.as_str()),
        source: wgpu::ShaderSource::Wgsl(source),
    };

    device.create_shader_module(descriptor)
}
