use wgpu::VertexBufferLayout;

/// Descriptor to configure creating a render pipeline
pub struct MyRenderPipelineDescriptor {
    /// Entry-point of the function in the shader wgsl file. Default is `vs_main`
    vertex_entry_point: String,

    /// Name of the function in the shader wgsl file. Default is `fs_main`
    fragment_entry_point: String,

    /// Label assigned to the render pipeline
    label_render_pipeline: String,

    /// Topology of the points/vertices. Default is `wgpu::PrimitiveTopology::TriangleList`
    topology: wgpu::PrimitiveTopology,
}

impl Default for MyRenderPipelineDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl MyRenderPipelineDescriptor {
    pub fn new() -> Self {
        Self {
            vertex_entry_point: "vs_main".to_owned(),
            fragment_entry_point: "fs_main".to_owned(),
            label_render_pipeline: "Render Pipeline".to_owned(),
            topology: wgpu::PrimitiveTopology::TriangleList,
        }
    }

    pub fn with_label_render_pipeline(mut self, label_render_pipeline: String) -> Self {
        self.label_render_pipeline = label_render_pipeline;
        self
    }

    pub fn with_vertex_entry_point(mut self, vertex_entry_point: String) -> Self {
        self.vertex_entry_point = vertex_entry_point;
        self
    }

    pub fn with_fragment_entry_point(mut self, fragment_entry_point: String) -> Self {
        self.fragment_entry_point = fragment_entry_point;
        self
    }

    pub fn with_topology(mut self, topology: wgpu::PrimitiveTopology) -> Self {
        self.topology = topology;
        self
    }
}

/// Create a pipeline layout with default values
pub fn create_default_pipeline_layout(
    device: &wgpu::Device,
    label_pipeline_layout: Option<&str>,
) -> wgpu::PipelineLayout {
    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: label_pipeline_layout,
        bind_group_layouts: &[],
        immediate_size: 0,
    })
}

/// Create a new render pipeline with default values.
///
///
pub fn create_default_render_pipeline<'a>(
    device: &wgpu::Device,
    shader: &'a wgpu::ShaderModule,
    format: wgpu::TextureFormat,
    render_pipeline_layout: &wgpu::PipelineLayout,
    buffers: &'a [VertexBufferLayout<'a>],
    descriptor: &MyRenderPipelineDescriptor,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(&descriptor.label_render_pipeline),
        layout: Some(render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some(&descriptor.vertex_entry_point),
            buffers,
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some(&descriptor.fragment_entry_point),
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: descriptor.topology,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview_mask: None,
        cache: None,
    })
}
