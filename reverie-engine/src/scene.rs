//! シーンに関するモジュール

use crate::{
    camera::{Camera, OrthographicCamera, PerspectiveCamera},
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
use nalgebra::{Point3, Vector3};

#[derive(Debug)]
pub struct Scene {
    pub meshes: Registry<MeshKey, Mesh>,
    pub materials: Registry<MaterialKey, Material>,
    pub game_objects: DenseRegistry<GameObjectKey, GameObject>,
    pub textures: TextureRegistry,
    /// Skybox color
    pub skybox: wgpu::Color,
    /// Main (and the only for now) camera
    pub camera: Camera,
}

impl Default for Scene {
    fn default() -> Self {
        let camera = if true {
            PerspectiveCamera::new(
                &Point3::new(0.0, 0.0, -0.5),
                &Point3::new(0.0, 0.0, 0.0),
                &Vector3::new(0.0, 1.0, 0.0),
                90.0_f32.to_radians(),
                0.1,
                100.0,
            )
            .into()
        } else {
            OrthographicCamera {
                eye: Point3::new(0.0, 0.0, -0.5),
                target: Point3::new(0.0, 0.0, 0.0),
                up: Vector3::new(0.0, 1.0, 0.0),
                size: 0.5,
                z_near: 0.1,
                z_far: 100.0,
            }
            .into()
        };
        Self {
            meshes: Default::default(),
            materials: Default::default(),
            game_objects: Default::default(),
            textures: Default::default(),
            skybox: wgpu::Color {
                r: 54.0 / 255.0,
                g: 77.0 / 255.0,
                b: 118.0 / 255.0,
                a: 1.0,
            },
            camera,
        }
    }
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

    pub fn new_game_object(
        &mut self,
        name: String,
        parent: Option<GameObjectKey>,
    ) -> GameObjectKey {
        let game_object = GameObject { name, parent };
        self.game_objects.map.insert(game_object)
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
    pub parent: Option<GameObjectKey>,
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
