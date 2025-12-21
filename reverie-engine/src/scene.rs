//! シーンに関するモジュール

use crate::{render::RenderingResource, scene::frame::Frame, texture::TextureId};

mod components;
pub mod frame;

pub use components::{
    colored::ColoredComponent, model::ModelComponent, sprite::SpriteComponent,
    transform::TransformComponent,
};

#[derive(Debug, Default)]
pub struct Scene {
    pub meshes: Registry<MeshKey, Mesh>,
    pub materials: Registry<MaterialKey, Material>,
    pub game_objects: DenseRegistry<GameObjectKey, GameObject>,
}

impl Scene {
    pub const fn setup(&mut self, _resource: &RenderingResource<'_>) {}

    pub const fn update(&mut self, _frame: &Frame<'_>, _resource: &RenderingResource<'_>) {}

    pub fn render(&mut self, rp: &mut wgpu::RenderPass<'_>, resource: &RenderingResource<'_>) {
        rp.set_pipeline(&resource.colored_pipeline.pipeline);
        rp.set_bind_group(
            crate::render::colored::GROUP_TRANSFORM,
            &resource.colored_pipeline.uniform_bind_group,
            &[],
        );

        rp.set_pipeline(&resource.sprite_pipeline.pipeline);
        rp.set_bind_group(
            crate::render::sprite::GROUP_TRANSFORM,
            &resource.sprite_pipeline.uniform_bind_group,
            &[],
        );
    }
}

/// 汎用レジストリ
///
/// ## [`DenseRegistry`] との使い分け
///
/// イテレーションよりもランダムアクセスが多い場合は [`Registry`] を使用する。
pub struct Registry<K: slotmap::Key, V> {
    map: slotmap::SlotMap<K, V>,
}

/// 密な汎用レジストリ
///
/// ## [`Registry`] との使い分け
///
/// イテレーションが多くランダムアクセスが少ない場合は [`DenseRegistry`] を使用する。
pub struct DenseRegistry<K: slotmap::Key, V> {
    map: slotmap::DenseSlotMap<K, V>,
}

impl<K: slotmap::Key, V> Default for Registry<K, V> {
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl<K: slotmap::Key, V> Default for DenseRegistry<K, V> {
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

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

#[derive(Debug)]
pub struct GameObject {
    pub name: String,
    pub model_component: ModelComponent,
}

slotmap::new_key_type! { pub struct GameObjectKey; }

impl<K: slotmap::Key, V> std::fmt::Debug for Registry<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { map } = self;
        f.debug_struct("Registry").field("len", &map.len()).finish()
    }
}

impl<K: slotmap::Key, V> std::fmt::Debug for DenseRegistry<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { map } = self;
        f.debug_struct("DenseRegistry")
            .field("len", &map.len())
            .finish()
    }
}

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
