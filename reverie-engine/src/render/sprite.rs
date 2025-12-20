use std::borrow::Cow;

use wgpu as w;

use crate::{model::sprite::SpriteVertex, render::vertex::Vertex};

use super::{BindingId, texture::WgpuTexture, uniform};

pub static LOC_VERTEX: u32 = 0;
pub static LOC_UV: u32 = 1;
pub static GROUP_TEXTURE: u32 = 0;
pub static BINDING_TEXTURE: BindingId = BindingId::new(GROUP_TEXTURE, 0);
pub static BINDING_SAMPLER: BindingId = BindingId::new(GROUP_TEXTURE, 1);
pub static GROUP_TRANSFORM: u32 = 1;
pub static BINDING_TRANSFORM: BindingId = BindingId::new(GROUP_TRANSFORM, 0);

impl Vertex for SpriteVertex {
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
                shader_location: LOC_UV,
                format: w::VertexFormat::Float32x2,
            },
        ],
    };
}

#[derive(Debug)]
pub struct SpriteRenderPipeline {
    pub pipeline: w::RenderPipeline,
    pub texture_bind_group_layout: w::BindGroupLayout,
    pub uniform_bind_group: w::BindGroup,
}

impl SpriteRenderPipeline {
    pub fn new(
        device: &w::Device,
        surface_format: w::TextureFormat,
        transform_uniform_buffer: &w::Buffer,
    ) -> Self {
        let shader = device.create_shader_module(w::ShaderModuleDescriptor {
            label: Some("sprite.wgsl"),
            source: w::ShaderSource::Wgsl(Cow::Borrowed(include_str!("./sprite.wgsl"))),
        });

        let texture_bind_group_layout = create_texture_binding_layout(device);
        let (uniform_bind_group_layout, uniform_bind_group) = uniform::create_mat4_bind_group(
            device,
            transform_uniform_buffer,
            BINDING_TRANSFORM.binding,
        );

        let pipeline_layout = device.create_pipeline_layout(&w::PipelineLayoutDescriptor {
            label: Some("sprite model render pipeline layout"),
            bind_group_layouts: &[&texture_bind_group_layout, &uniform_bind_group_layout],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&w::RenderPipelineDescriptor {
            label: Some("sprite model render pipeline"),
            layout: Some(&pipeline_layout),
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
            cache: None,
            multiview_mask: None,
        });

        Self {
            pipeline,
            texture_bind_group_layout,
            uniform_bind_group,
        }
    }
}

fn create_texture_binding_layout(device: &w::Device) -> w::BindGroupLayout {
    WgpuTexture::bind_group_layout(
        device,
        Some("sprite texture bind group layout"),
        BINDING_TEXTURE.binding,
        BINDING_SAMPLER.binding,
    )
}
