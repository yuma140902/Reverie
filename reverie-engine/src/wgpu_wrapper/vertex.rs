pub trait Vertex: bytemuck::Pod + bytemuck::Zeroable {
    /// 頂点バッファのレイアウト
    const DESC: wgpu::VertexBufferLayout<'static>;
}
