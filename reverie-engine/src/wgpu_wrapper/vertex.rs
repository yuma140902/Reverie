use wgpu as w;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
/// UV 座標を持つ頂点
///
/// * `position`: 頂点の位置
/// * `uv`: UV 座標
pub struct UvVertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
}

impl UvVertex {
    pub const fn desc() -> w::VertexBufferLayout<'static> {
        w::VertexBufferLayout {
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
        }
    }
}
