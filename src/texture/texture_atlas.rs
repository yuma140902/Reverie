//! テクスチャアトラスの仕組みを実現するためのモジュール

/// テクスチャアトラス内での、始点と終点のUV座標を表す
pub struct TextureUV<const W: u32, const H: u32, const ATLAS_W: u32, const ATLAS_H: u32> {
    pub begin_u: f32,
    pub begin_v: f32,
    pub end_u: f32,
    pub end_v: f32,
}

impl<const W: u32, const H: u32, const ATLAS_W: u32, const ATLAS_H: u32>
    TextureUV<W, H, ATLAS_W, ATLAS_H>
{
    /// `ATLAS_W`×`ATLAS_H`のテクスチャアトラスを`W`×`H`のサイズに分割した時、`pos.row`行目`pos.column`列目のパーツの始点と終点のUV座標
    pub fn of_atlas(pos: &TextureAtlasPos) -> TextureUV<W, H, ATLAS_W, ATLAS_H> {
        TextureUV {
            begin_u: (pos.column * W) as f32 / ATLAS_W as f32,
            begin_v: 1.0f32 - ((pos.row + 1) * H) as f32 / ATLAS_H as f32,
            end_u: ((pos.column + 1) * W) as f32 / ATLAS_W as f32,
            end_v: 1.0f32 - (pos.row * H) as f32 / ATLAS_H as f32,
        }
    }
}

/// テクスチャアトラス内で何行目何列目かを表す
pub struct TextureAtlasPos {
    pub row: u32,
    pub column: u32,
}

impl TextureAtlasPos {
    pub const fn new(row: u32, column: u32) -> Self {
        Self { row, column }
    }
}
