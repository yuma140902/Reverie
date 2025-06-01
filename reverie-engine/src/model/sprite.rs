use wgpu as w;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SpriteVertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
}

impl super::Vertex for SpriteVertex {
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
