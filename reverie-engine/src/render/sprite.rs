use std::borrow::Cow;

use wgpu as w;

use crate::{model::sprite::SpriteVertex, render::vertex::Vertex};

impl Vertex for SpriteVertex {
    const DESC: wgpu::VertexBufferLayout<'static> = w::VertexBufferLayout {
        array_stride: size_of::<Self>() as w::BufferAddress,
        step_mode: w::VertexStepMode::Vertex,
        attributes: &[
            w::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: w::VertexFormat::Float32x3,
            },
            w::VertexAttribute {
                offset: size_of::<[f32; 3]>() as w::BufferAddress,
                shader_location: 1,
                format: w::VertexFormat::Float32x2,
            },
        ],
    };
}

#[derive(Debug)]
pub struct SpriteRenderPipeline(w::RenderPipeline);

impl SpriteRenderPipeline {
    pub const fn get(&self) -> &w::RenderPipeline {
        &self.0
    }

    pub const fn get_mut(&mut self) -> &mut w::RenderPipeline {
        &mut self.0
    }

    pub fn new(
        device: &w::Device,
        bind_group_layouts: &[&w::BindGroupLayout],
        surface_format: w::TextureFormat,
    ) -> Self {
        let shader = device.create_shader_module(w::ShaderModuleDescriptor {
            label: Some("shader.wgsl for sprite model"),
            source: w::ShaderSource::Wgsl(Cow::Borrowed(include_str!("../shader.wgsl"))),
        });

        let layout = device.create_pipeline_layout(&w::PipelineLayoutDescriptor {
            label: Some("sprite model render pipeline layout"),
            bind_group_layouts,
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&w::RenderPipelineDescriptor {
            label: Some("sprite model render pipeline"),
            layout: Some(&layout),
            vertex: w::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[SpriteVertex::DESC],
            },
            fragment: Some(w::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(w::ColorTargetState {
                    format: surface_format,
                    blend: Some(w::BlendState {
                        color: w::BlendComponent {
                            src_factor: w::BlendFactor::SrcAlpha,
                            dst_factor: w::BlendFactor::OneMinusSrcAlpha,
                            operation: w::BlendOperation::Add,
                        },
                        alpha: w::BlendComponent::OVER,
                    }),
                    write_mask: w::ColorWrites::ALL,
                })],
            }),
            primitive: w::PrimitiveState {
                topology: w::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: w::FrontFace::Ccw,
                cull_mode: Some(w::Face::Back),
                unclipped_depth: false,
                polygon_mode: w::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(w::DepthStencilState {
                format: w::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: w::CompareFunction::Less,
                stencil: w::StencilState::default(),
                bias: w::DepthBiasState::default(),
            }),
            multisample: w::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: true,
            },
            multiview: None,
            cache: None,
        });
        Self(pipeline)
    }
}
