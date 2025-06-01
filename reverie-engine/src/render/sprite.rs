use std::borrow::Cow;

use wgpu as w;

use crate::{model::sprite::SpriteVertex, render::vertex::Vertex};

use super::{BindingId, texture::WgpuTexture};

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
            label: Some("sprite.wgsl"),
            source: w::ShaderSource::Wgsl(Cow::Borrowed(include_str!("./sprite.wgsl"))),
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

pub fn create_texture_binding_layout(device: &w::Device) -> w::BindGroupLayout {
    WgpuTexture::bind_group_layout(
        device,
        Some("sprite texture bind group layout"),
        BINDING_TEXTURE.binding,
        BINDING_SAMPLER.binding,
    )
}

pub fn create_uniform_bind_group(
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
