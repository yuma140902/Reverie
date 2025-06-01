use std::ops::Range;

use wgpu as w;

use crate::model::Vertex;

#[derive(Debug)]
/// 頂点バッファとインデックスバッファをまとめた構造体
pub struct VertexIndexBuffer<V> {
    pub(crate) vertex_buffer: w::Buffer,
    vertex_array: Vec<V>,
    pub(crate) index_buffer: w::Buffer,
    index_array: Vec<u16>,
    pub(crate) index_buffer_range: Range<u32>,
}

impl<V: Vertex> VertexIndexBuffer<V> {
    pub fn new(
        device: &w::Device,
        max_vertices: usize,
        max_indices: usize,
        label: Option<&str>,
    ) -> anyhow::Result<Self> {
        let name_v = label.map(|label| format!("{label} [vertex part]"));
        let name_i = label.map(|label| format!("{label} [index part]"));

        let vertex_buffer = device.create_buffer(&w::BufferDescriptor {
            label: name_v.as_deref(),
            usage: w::BufferUsages::VERTEX | w::BufferUsages::COPY_DST,
            size: (max_vertices * size_of::<V>()) as u64,
            mapped_at_creation: false,
        });
        let index_buffer = device.create_buffer(&w::BufferDescriptor {
            label: name_i.as_deref(),
            usage: w::BufferUsages::INDEX | w::BufferUsages::COPY_DST,
            size: (max_indices * size_of::<u16>()) as u64,
            mapped_at_creation: false,
        });

        Ok(Self {
            vertex_buffer,
            vertex_array: Vec::with_capacity(max_vertices),
            index_buffer,
            index_array: Vec::with_capacity(max_indices),
            index_buffer_range: 0..0,
        })
    }

    pub const fn start_update<'a>(
        &'a mut self,
        queue: &'a w::Queue,
    ) -> VertexIndexBufferUpdater<'a, V> {
        VertexIndexBufferUpdater {
            buffer: self,
            queue,
            vertex_update: 0..0,
            index_update: 0..0,
        }
    }

    fn send_to_gpu(
        &self,
        queue: &w::Queue,
        vertex_update: Range<usize>,
        index_update: Range<usize>,
    ) {
        if !vertex_update.is_empty() {
            queue.write_buffer(
                &self.vertex_buffer,
                vertex_update.start as u64,
                bytemuck::cast_slice(&self.vertex_array[vertex_update]),
            );
        }
        if !index_update.is_empty() {
            queue.write_buffer(
                &self.index_buffer,
                index_update.start as u64,
                bytemuck::cast_slice::<u16, u8>(&self.index_array[index_update]),
            );
        }
    }
}

#[derive(Debug)]
/// [`VertexIndexBuffer`] の更新を行うための構造体。Drop されると GPU にデータを送信する
pub struct VertexIndexBufferUpdater<'a, V: Vertex> {
    buffer: &'a mut VertexIndexBuffer<V>,
    queue: &'a w::Queue,
    vertex_update: Range<usize>,
    index_update: Range<usize>,
}

impl<V: Vertex> VertexIndexBufferUpdater<'_, V> {
    pub const fn vertex_mut(&mut self) -> &mut Vec<V> {
        &mut self.buffer.vertex_array
    }

    pub const fn index_mut(&mut self) -> &mut Vec<u16> {
        &mut self.buffer.index_array
    }

    /// 更新した頂点バッファの範囲を設定する
    pub const fn set_vertex_update(&mut self, range: Range<usize>) {
        self.vertex_update = range;
    }

    /// 更新したインデックスバッファの範囲を設定する
    pub const fn set_index_update(&mut self, range: Range<usize>) {
        self.index_update = range;
    }

    /// インデックスバッファの描画範囲を設定する
    pub const fn set_render_range(&mut self, range: Range<u32>) {
        self.buffer.index_buffer_range = range;
    }
}

impl<V: Vertex> std::ops::Drop for VertexIndexBufferUpdater<'_, V> {
    fn drop(&mut self) {
        self.buffer.send_to_gpu(
            self.queue,
            self.vertex_update.clone(),
            self.index_update.clone(),
        )
    }
}
