//! シーンに関するモジュール

use std::collections::{HashSet, VecDeque};

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
    pub game_objects: DenseRegistry<GameObjectKey, GOCell<GameObject>>,
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

    pub fn new_game_object(
        &mut self,
        name: String,
        parent: Option<GameObjectKey>,
    ) -> GameObjectKey {
        let game_object = GameObject { name, parent };
        self.game_objects.map.insert(GOCell::new(game_object))
    }

    /**
    ゲームオブジェクトツリーを探索する

    子よりも先に親が訪問されることを保証する。

    # Arguments

    * `visit` - 各ゲームオブジェクトに対して呼び出されるクロージャ。引数として、`GOCellOwner`、現在のゲームオブジェクト、親のゲームオブジェクト（存在する場合）を受け取る。

    # Example

    ```rust
    use reverie_engine::scene::{Scene, GameObject, GameObjectKey, GOCellOwner, GOCell};

    let mut scene = Scene::default();

    let a = scene.new_game_object("A".to_string(), None);
    let b = scene.new_game_object("B".to_string(), Some(a));
    let c = scene.new_game_object("C".to_string(), Some(a));
    let d = scene.new_game_object("D".to_string(), Some(b));
    let e = scene.new_game_object("E".to_string(), Some(b));

    scene.traverse_tree(|owner: &mut GOCellOwner, obj: &GOCell<GameObject>, parent: Option<&GOCell<GameObject>>| {
        let Some(parent) = parent else {
            return;
        };

        let parent_name = {
            let parent_ref = owner.ro(parent);
            parent_ref.name.clone()
        };

        let obj_ref = owner.rw(obj);
        obj_ref.name = format!("{} -> {}", parent_name, obj_ref.name);
    });

    let mut owner = GOCellOwner::new();
    let mut names = Vec::new();
    for obj in scene.get_game_objects() {
        let obj_ref = owner.ro(obj);
        names.push(obj_ref.name.clone());
    }

    names.sort();
    assert_eq!(names, vec![
        "A",
        "A -> B",
        "A -> B -> D",
        "A -> B -> E",
        "A -> C",
    ]);
    ```
    */
    pub fn traverse_tree<F>(&mut self, mut visit: F)
    where
        F: FnMut(&mut GOCellOwner, &GOCell<GameObject>, Option<&GOCell<GameObject>>),
    {
        let mut visited = HashSet::new();
        let mut queue: VecDeque<GameObjectKey> = VecDeque::new();
        queue.extend(self.game_objects.map.keys_as_slice());

        while let Some(key) = queue.pop_front() {
            if visited.contains(&key) {
                continue;
            }

            let mut owner = GOCellOwner::new();
            let Some(obj) = self.game_objects.map.get(key) else {
                tracing::warn!("GameObject with key {:?} not found", key);
                visited.insert(key);
                continue;
            };

            let Some(parent_key) = owner.ro(obj).parent else {
                visit(&mut owner, obj, None);
                visited.insert(key);
                continue;
            };

            if !visited.contains(&parent_key) {
                queue.push_back(parent_key);
            }

            let Some(parent) = self.game_objects.map.get(parent_key) else {
                tracing::warn!("Parent GameObject with key {:?} not found", parent_key);
                continue;
            };

            visit(&mut owner, obj, Some(parent));
            visited.insert(key);
        }
    }

    pub fn get_game_objects(&self) -> &[GOCell<GameObject>] {
        self.game_objects.map.values_as_slice()
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

pub struct GOMarker;
pub type GOCell<T> = qcell::TCell<GOMarker, T>;
pub type GOCellOwner = qcell::TCellOwner<GOMarker>;

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
