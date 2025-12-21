use crate::texture::TextureId;

pub mod colored;
pub mod sprite;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub normal: [f32; 3],
}

pub struct Mesh {
    pub name: String,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub vertex_count: u32,
    pub index_count: u32,
}

slotmap::new_key_type! { pub struct MeshKey; }

#[derive(Debug)]
pub struct Material {
    pub texture: TextureId,
}

slotmap::new_key_type! { pub struct MaterialKey; }

impl std::fmt::Debug for Mesh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            name,
            vertices: _,
            indices: _,
            vertex_count,
            index_count,
        } = self;
        f.debug_struct("Mesh")
            .field("name", name)
            .field("vertex_count", vertex_count)
            .field("index_count", index_count)
            .finish()
    }
}
