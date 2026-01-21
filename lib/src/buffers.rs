use bytemuck::NoUninit;
use wgpu::util::DeviceExt;

/// create a vertex buffer from the given vertices
pub fn create_vertex_buffer<T>(device: &wgpu::Device, vertices: Vec<T>) -> wgpu::Buffer
where
    T: NoUninit,
{
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
    })
}

/// create an index buffer from the given vertices and indices
pub fn create_index_buffer<T>(
    device: &wgpu::Device,
    vertices: Vec<T>,
    indices: Vec<u16>,
) -> (wgpu::Buffer, wgpu::Buffer)
where
    T: NoUninit,
{
    let vertex_buffer = create_vertex_buffer(device, vertices);

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&indices),
        usage: wgpu::BufferUsages::INDEX,
    });
    (vertex_buffer, index_buffer)
}
