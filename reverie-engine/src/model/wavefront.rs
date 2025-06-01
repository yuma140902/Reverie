use std::path::Path;
use wgpu as w;

use crate::{render::buffer::VertexIndexBuffer, texture::TextureId};

#[derive(Debug)]
pub struct Model {
    meshes: Vec<Mesh>,
    materials: slotmap::SlotMap<slotmap::DefaultKey, Material>,
}

#[derive(Debug)]
pub struct Mesh {
    name: String,
    buffer: VertexIndexBuffer<WavefrontVertex>,
    num_elements: u32,
    material: slotmap::DefaultKey,
}

#[derive(Debug)]
pub struct Material {
    name: String,
    diffuse_texture: TextureId,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct WavefrontVertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub normal: [f32; 3],
}

pub fn load_from_file(
    path: impl AsRef<Path>,
    device: &w::Device,
    queue: &w::Queue,
) -> anyhow::Result<Model> {
    todo!()
}
