#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ColoredVertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}
