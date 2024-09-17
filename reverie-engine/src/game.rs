//! Game トレイト
use crate::{scene::Scene, texture::TextureRegistry};

/// ゲームが実装すべきトレイト
pub trait Game {
    /// シーンを生成する
    ///
    /// ゲームが開始されたときに呼ばれる。
    fn generate_scene(&mut self, registry: &mut TextureRegistry) -> anyhow::Result<Scene>;
}
