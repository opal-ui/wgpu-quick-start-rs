//! Module used to create `wgpu::RenderPass` to be used for rendering.
//! 
//! 

/// Create a new render pass for the given encoder
/// with a given label and clear with a particular color
pub fn create_render_pass<'b>(
    encoder: &'b mut wgpu::CommandEncoder,
    label: String,
    color: wgpu::Color,
    view: &'b wgpu::TextureView,
) -> wgpu::RenderPass<'b> {
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some(label.as_str()),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(color),
                store: wgpu::StoreOp::Store,
            },
            depth_slice: None,
        })],
        depth_stencil_attachment: None,
        occlusion_query_set: None,
        timestamp_writes: None,
        multiview_mask: None,
    })
}
