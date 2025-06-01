use wgpu as w;

use crate::model::wavefront::WavefrontVertex;

use super::{BindingId, vertex::Vertex};

pub static LOC_VERTEX: u32 = 0;
pub static LOC_UV: u32 = 1;
pub static LOC_NORMAL: u32 = 2;
pub static GROUP_TRANSFORM: u32 = 0;
pub static BINDING_TRANSFORM: BindingId = BindingId::new(GROUP_TRANSFORM, 0);

impl Vertex for WavefrontVertex {
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
            w::VertexAttribute {
                offset: size_of::<[f32; 3 + 2]>() as w::BufferAddress,
                shader_location: LOC_NORMAL,
                format: w::VertexFormat::Float32x3,
            },
        ],
    };
}
