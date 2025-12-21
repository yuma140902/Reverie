//! シーンに関するモジュール

use crate::{
    model::{Material, MaterialKey, Mesh, MeshKey},
    render::{RenderingResource, sprite},
    scene::frame::Frame,
    texture::TextureRegistry,
};

mod components;
pub mod frame;

pub use components::{
    model::ModelComponent, sprite::SpriteComponent, transform::TransformComponent,
};

#[derive(Debug, Default)]
pub struct Scene {
    pub meshes: Registry<MeshKey, Mesh>,
    pub materials: Registry<MaterialKey, Material>,
    pub game_objects: DenseRegistry<GameObjectKey, GameObject>,
    pub textures: TextureRegistry,
}

impl Scene {
    pub fn setup(&mut self, r: &RenderingResource<'_>) {
        self.textures.send_all_to_gpu(
            &r.device,
            &r.queue,
            &r.sprite_pipeline.texture_bind_group_layout,
            &r.texture_sampler,
            sprite::BINDING_TEXTURE.binding,
            sprite::BINDING_SAMPLER.binding,
        );
    }

    pub const fn update(&mut self, _frame: &Frame<'_>, _resource: &RenderingResource<'_>) {}

    pub fn render(&mut self, rp: &mut wgpu::RenderPass<'_>, resource: &RenderingResource<'_>) {
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
    pub(crate) map: slotmap::SlotMap<K, V>,
}

/// 密な汎用レジストリ
///
/// ## [`Registry`] との使い分け
///
/// イテレーションが多くランダムアクセスが少ない場合は [`DenseRegistry`] を使用する。
pub struct DenseRegistry<K: slotmap::Key, V> {
    pub(crate) map: slotmap::DenseSlotMap<K, V>,
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
