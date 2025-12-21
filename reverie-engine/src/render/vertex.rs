pub trait VertexLayout {
    /// 頂点バッファのレイアウト
    const DESC: wgpu::VertexBufferLayout<'static>;
}
