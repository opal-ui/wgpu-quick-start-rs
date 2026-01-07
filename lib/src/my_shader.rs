//! Module to create shaders and render pipelines to be used for rendering shapes and objects
//!
//!

use std::borrow::Cow;

use wgpu::VertexBufferLayout;

pub struct MyShader {
    /// Entry-point of the function in the shader wgsl file. Default is `vs_main`
    vertex_entry_point: String,

    /// Name of the function in the shader wgsl file. Default is `fs_main`
    fragment_entry_point: String,

    /// Label assigned to the pipeline layout
    label_pipeline_layout: String,

    /// Label assigned to the render pipeline
    label_render_pipeline: String,

    /// Topology of the points/vertices. Default is `wgpu::PrimitiveTopology::TriangleList`
    topology: wgpu::PrimitiveTopology,

    /// Name of the shader module
    shader: wgpu::ShaderModule,
}

impl MyShader {
    /// Create a new shader from the given `device`, giving it a given `label`, sourcing it from `source`
    pub fn new<'b>(device: &wgpu::Device, label: String, source: Cow<'b, str>) -> Self {
        let descriptor = wgpu::ShaderModuleDescriptor {
            label: Some(label.as_str()),
            source: wgpu::ShaderSource::Wgsl(source),
        };
        let shader = device.create_shader_module(descriptor);
        Self {
            vertex_entry_point: "vs_main".to_owned(),
            fragment_entry_point: "fs_main".to_owned(),
            label_pipeline_layout: "Render Pipeline Layout".to_owned(),
            label_render_pipeline: "Render Pipeline".to_owned(),
            topology: wgpu::PrimitiveTopology::TriangleList,
            shader,
        }
    }

    pub fn with_label_pipeline_layout(&mut self, label_pipeline_layout: String) -> &Self {
        self.label_pipeline_layout = label_pipeline_layout;
        self
    }

    pub fn with_label_render_pipeline(&mut self, label_render_pipeline: String) -> &Self {
        self.label_render_pipeline = label_render_pipeline;
        self
    }

    pub fn with_vertex_entry_point(&mut self, vertex_entry_point: String) -> &Self {
        self.vertex_entry_point = vertex_entry_point;
        self
    }

    pub fn with_fragment_entry_point(&mut self, fragment_entry_point: String) -> &Self {
        self.fragment_entry_point = fragment_entry_point;
        self
    }

    pub fn with_topology(mut self, topology: wgpu::PrimitiveTopology) -> Self {
        self.topology = topology;
        self
    }

    pub fn create_render_pipeline<'a>(
        &self,
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        buffers: &'a [VertexBufferLayout<'a>],
    ) -> wgpu::RenderPipeline {
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some(&self.label_pipeline_layout),
                bind_group_layouts: &[],
                immediate_size: 0,
            });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(&self.label_render_pipeline),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &self.shader,
                entry_point: Some(self.vertex_entry_point.as_str()),
                buffers,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &self.shader,
                entry_point: Some(self.fragment_entry_point.as_str()),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: self.topology, // wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
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
}
