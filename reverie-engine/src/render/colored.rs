use std::borrow::Cow;

use wgpu as w;

use crate::model::colored::ColoredVertex;

use super::{BindingId, vertex::Vertex};

pub static LOC_VERTEX: u32 = 0;
pub static LOC_COLOR: u32 = 1;
pub static GROUP_TRANSFORM: u32 = 0;
pub static BINDING_TRANSFORM: BindingId = BindingId::new(GROUP_TRANSFORM, 0);

impl Vertex for ColoredVertex {
    const DESC: wgpu::VertexBufferLayout<'static> = w::VertexBufferLayout {
        array_stride: size_of::<Self>() as w::BufferAddress,
        step_mode: w::VertexStepMode::Vertex,
        attributes: &[
            w::VertexAttribute {
                offset: 0,
                shader_location: LOC_VERTEX,
                format: w::VertexFormat::Float32x3,
            },
            w::VertexAttribute {
                offset: size_of::<[f32; 3]>() as w::BufferAddress,
                shader_location: LOC_COLOR,
                format: w::VertexFormat::Float32x4,
            },
        ],
    };
}

#[derive(Debug)]
pub struct ColoredRenderPipeline {
    pub pipeline: w::RenderPipeline,
    pub uniform_bind_group: w::BindGroup,
}

impl ColoredRenderPipeline {
    pub fn new(
        device: &w::Device,
        surface_format: w::TextureFormat,
        transform_uniform_buffer: &w::Buffer,
    ) -> Self {
        let shader = device.create_shader_module(w::ShaderModuleDescriptor {
            label: Some("colored.wgsl"),
            source: w::ShaderSource::Wgsl(Cow::Borrowed(include_str!("./colored.wgsl"))),
        });

        let (uniform_bind_group_layout, uniform_bind_group) =
            create_uniform_bind_group(device, transform_uniform_buffer);

        let pipeline_layout = device.create_pipeline_layout(&w::PipelineLayoutDescriptor {
            label: Some("sprite model render pipeline layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&w::RenderPipelineDescriptor {
            label: Some("sprite model render pipeline"),
            layout: Some(&pipeline_layout),
            vertex: w::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[ColoredVertex::DESC],
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

        Self {
            pipeline,
            uniform_bind_group,
        }
    }
}

fn create_uniform_bind_group(
    device: &w::Device,
    transform_uniform_buffer: &w::Buffer,
) -> (w::BindGroupLayout, w::BindGroup) {
    let layout = device.create_bind_group_layout(&w::BindGroupLayoutDescriptor {
        label: Some("Main Bind Group Layout"),
        entries: &[w::BindGroupLayoutEntry {
            binding: BINDING_TRANSFORM.binding,
            visibility: w::ShaderStages::VERTEX,
            ty: w::BindingType::Buffer {
                ty: w::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: w::BufferSize::new(size_of::<[f32; 4 * 4]>() as u64),
            },
            count: None,
        }],
    });

    let bind_group = device.create_bind_group(&w::BindGroupDescriptor {
        label: Some("Main Bind Group"),
        layout: &layout,
        entries: &[w::BindGroupEntry {
            binding: BINDING_TRANSFORM.binding,
            resource: transform_uniform_buffer.as_entire_binding(),
        }],
    });

    (layout, bind_group)
}
