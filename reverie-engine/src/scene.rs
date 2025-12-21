//! シーンに関するモジュール

use crate::{
    model::{Material, MaterialKey, Mesh, MeshKey},
    render::{RenderingResource, sprite},
    scene::frame::Frame,
    texture::TextureRegistry,
};

mod components;
pub mod frame;

use anyhow::Context;
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

pub struct TreeNode<K, V> {
    pub value: V,
    parent: Option<K>,
    children: Vec<K>,
}

impl<K, V> TreeNode<K, V> {
    pub const fn new(value: V) -> Self {
        Self {
            value,
            parent: None,
            children: Vec::new(),
        }
    }
}

impl<K: slotmap::Key, V> TreeNode<K, V> {
    /// 親子関係を設定する
    ///
    /// `child_key` が既に別の親を持つ場合、その親子関係は解除される
    ///
    /// # Arguments
    ///
    /// - `map`: ノードの実体を管理するマップ
    /// - `parent_key`: 親ノードのキー
    /// - `child_key`: 子ノードのキー
    ///
    /// # Returns
    ///
    /// 成功時は Ok(())、サイクルが発生する場合や無効なキーの場合は Err
    pub fn link_nodes(
        map: &mut slotmap::DenseSlotMap<K, Self>,
        parent_key: K,
        child_key: K,
    ) -> anyhow::Result<()> {
        if Self::has_ancestor(map, parent_key, child_key) {
            anyhow::bail!(
                "Cannot link nodes: would create a cycle (parent_key={:?}, child_key={:?})",
                parent_key,
                child_key
            );
        }

        Self::unlink_parent(map, child_key).context("Failed to unlink existing parent")?;

        let [parent, child] = map
            .get_disjoint_mut([parent_key, child_key])
            .with_context(|| {
                format!(
                    "Failed to get parent and child mutable references, keys={:?}",
                    [parent_key, child_key]
                )
            })?;

        parent.children.push(child_key);
        child.parent = Some(parent_key);

        Ok(())
    }

    /// 親子関係を解除し、元の親のキーを返す
    ///
    /// # Returns
    ///
    /// - `Ok(Some(parent_key))` 親子関係が解除され、元の親のキーが返される
    /// - `Ok(None)` 親子関係が存在しなかった
    pub fn unlink_parent(
        map: &mut slotmap::DenseSlotMap<K, Self>,
        child_key: K,
    ) -> anyhow::Result<Option<K>> {
        let child = map
            .get(child_key)
            .with_context(|| format!("Child node not found (key={:?})", child_key))?;
        let Some(parent_key) = child.parent else {
            return Ok(None);
        };

        let [parent, child] = map
            .get_disjoint_mut([parent_key, child_key])
            .with_context(|| {
                format!(
                    "Failed to get parent and child mutable references, keys={:?}",
                    [parent_key, child_key]
                )
            })?;

        let index = parent
            .children
            .iter()
            .position(|&k| k == child_key)
            .with_context(|| {
            format!(
                "Child key not found in parent's children list, parent_key={:?}, child_key={:?}",
                parent_key, child_key
            )
            })?;
        parent.children.swap_remove(index);

        child.parent = None;

        Ok(Some(parent_key))
    }

    /// 指定したノードが指定した祖先ノードを持つかどうかを確認する
    ///
    /// 2 つのキーが同じ場合も `true` を返す
    fn has_ancestor(map: &slotmap::DenseSlotMap<K, Self>, node_key: K, ancestor_key: K) -> bool {
        if node_key == ancestor_key {
            return true;
        }
        let mut current_key = node_key;
        while let Some(node) = map.get(current_key) {
            if let Some(parent_key) = node.parent {
                if parent_key == ancestor_key {
                    return true;
                }
                current_key = parent_key;
            } else {
                break;
            }
        }
        false
    }
}

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

#[cfg(test)]
mod tree_pbt {
    use std::collections::HashSet;

    use super::TreeNode;
    use proptest::{prelude::*, prop_oneof, proptest};

    slotmap::new_key_type! { struct TestKey; }

    #[derive(Debug, Clone)]
    enum Action {
        CreateNode,
        Link {
            parent_index: usize,
            child_index: usize,
        },
        Unlink {
            child_index: usize,
        },
    }

    fn execute_actions(actions: &[Action]) {
        let mut map: slotmap::DenseSlotMap<TestKey, TreeNode<TestKey, ()>> =
            slotmap::DenseSlotMap::default();

        for action in actions {
            match action {
                Action::CreateNode => {
                    map.insert(TreeNode::new(()));
                }
                Action::Link {
                    parent_index,
                    child_index,
                } => {
                    let keys = map.keys_as_slice();
                    if *parent_index >= keys.len() || *child_index >= keys.len() {
                        continue;
                    }
                    let parent_key = keys[*parent_index];
                    let child_key = keys[*child_index];

                    if let Err(e) = TreeNode::link_nodes(&mut map, parent_key, child_key) {
                        let msg = format!("{:?}", e);
                        if msg.contains("would create a cycle") {
                            // サイクルが発生する場合は無視
                            continue;
                        } else {
                            panic!("Unexpected error during linking: {}", msg);
                        }
                    }
                }
                Action::Unlink { child_index } => {
                    let keys = map.keys_as_slice();
                    if *child_index >= keys.len() {
                        continue;
                    }
                    let child_key = keys[*child_index];
                    TreeNode::unlink_parent(&mut map, child_key).unwrap();
                }
            }
        }

        check_tree_integrity(&map);
    }

    fn check_tree_integrity(map: &slotmap::DenseSlotMap<TestKey, TreeNode<TestKey, ()>>) {
        // 双方向のリンクが正しいことを確認する
        //
        // forall c, p in map: c.parent == p <=> c in p.children
        for (key, node) in map {
            if let Some(parent_key) = node.parent {
                let parent = &map[parent_key];
                assert!(
                    parent.children.contains(&key),
                    "Bidirectional linking failed: parent does not reference child"
                );
            }
            for &child_key in &node.children {
                let child = &map[child_key];
                assert!(
                    child.parent == Some(key),
                    "Bidirectional linking failed: child does not reference parent"
                );
            }
        }

        // 非循環性
        for (key, _) in map {
            let mut visited = std::collections::HashSet::new();
            let mut current_key = key;
            while let Some(node) = map.get(current_key) {
                if !visited.insert(current_key) {
                    panic!(
                        "Acyclicity failed: cycle detected starting from key {:?}",
                        key
                    );
                }
                if let Some(parent_key) = node.parent {
                    current_key = parent_key;
                } else {
                    break;
                }
            }
        }

        // children リスト内の重複がないことを確認する
        for (key, node) in map {
            let unique_children_len = node.children.iter().collect::<HashSet<_>>().len();
            assert_eq!(
                unique_children_len,
                node.children.len(),
                "Children uniqueness failed: duplicate children found for key {:?}",
                key
            );
        }
    }

    fn action_strategy(tree_size: usize) -> impl Strategy<Value = Action> {
        prop_oneof![
            30 => Just(Action::CreateNode),
            50 => (0..tree_size, 0..tree_size).prop_map(|(p, c)| Action::Link { parent_index: p, child_index: c }),
            20 => (0..tree_size).prop_map(|c| Action::Unlink { child_index: c }),
        ]
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn test_tree_operations_with_small_tree_many_actions(
            actions in proptest::collection::vec(action_strategy(10), 1..5000)
        ) {
            execute_actions(&actions);
        }

        #[test]
        fn test_tree_operations_with_large_tree_few_actions(
            actions in proptest::collection::vec(action_strategy(100), 1..500)
        ) {
            execute_actions(&actions);
        }
    }
}

#[cfg(test)]
mod tree_test {
    use super::TreeNode;

    slotmap::new_key_type! { struct TestKey; }

    #[test]
    fn error_on_linking_same_node() {
        let mut map: slotmap::DenseSlotMap<TestKey, TreeNode<TestKey, ()>> =
            slotmap::DenseSlotMap::default();
        let a = map.insert(TreeNode::new(()));

        let result = TreeNode::link_nodes(&mut map, a, a);
        assert!(result.is_err());
    }

    #[test]
    fn unlink_no_parent() {
        let mut map: slotmap::DenseSlotMap<TestKey, TreeNode<TestKey, ()>> =
            slotmap::DenseSlotMap::default();
        let a = map.insert(TreeNode::new(()));
        let result = TreeNode::unlink_parent(&mut map, a);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn error_on_introducing_cycle() {
        let mut map: slotmap::DenseSlotMap<TestKey, TreeNode<TestKey, ()>> =
            slotmap::DenseSlotMap::default();
        let a = map.insert(TreeNode::new(()));
        let b = map.insert(TreeNode::new(()));
        let c = map.insert(TreeNode::new(()));
        TreeNode::link_nodes(&mut map, a, b).unwrap();
        TreeNode::link_nodes(&mut map, b, c).unwrap();
        let result = TreeNode::link_nodes(&mut map, c, a);
        assert!(result.is_err());
    }

    #[test]
    fn error_on_linking_invalid_child_key() {
        let mut map: slotmap::DenseSlotMap<TestKey, TreeNode<TestKey, ()>> =
            slotmap::DenseSlotMap::default();
        let a = map.insert(TreeNode::new(()));
        let b = map.insert(TreeNode::new(()));
        map.remove(b);
        let result = TreeNode::link_nodes(&mut map, a, b);
        assert!(result.is_err());
    }

    #[test]
    fn error_on_linking_invalid_parent_key() {
        let mut map: slotmap::DenseSlotMap<TestKey, TreeNode<TestKey, ()>> =
            slotmap::DenseSlotMap::default();
        let a = map.insert(TreeNode::new(()));
        let b = map.insert(TreeNode::new(()));
        map.remove(a);
        let result = TreeNode::link_nodes(&mut map, a, b);
        assert!(result.is_err());
    }

    #[test]
    fn error_on_unlinking_invalid_key() {
        let mut map: slotmap::DenseSlotMap<TestKey, TreeNode<TestKey, ()>> =
            slotmap::DenseSlotMap::default();
        let a = map.insert(TreeNode::new(()));
        map.remove(a);
        let result = TreeNode::unlink_parent(&mut map, a);
        assert!(result.is_err());
    }
}
