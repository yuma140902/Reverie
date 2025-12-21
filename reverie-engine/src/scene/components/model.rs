use crate::scene::{MaterialKey, MeshKey};

#[derive(Debug)]
pub struct ModelComponent {
    pub meshes: Vec<(MeshKey, MaterialKey)>,
}
