#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// [`Scene`]に登録したエンティティのインデックス
pub struct EntityIndex(pub hecs::Entity);
