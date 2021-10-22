//! テクスチャアトラスの仕組みを実現するためのモジュール

/// テクスチャアトラス内での、始点と終点のUV座標を表す
pub struct TextureUV {
    pub begin_u: f32,
    pub begin_v: f32,
    pub end_u: f32,
    pub end_v: f32,
}

impl TextureUV {
    /// `atlas_width`×`atlas_height`のテクスチャアトラスを`width`×`height`のサイズに分割した時、`row`行目`column`列目のパーツの始点と終点のUV座標
    pub fn of_atlas(
        row: u32,
        column: u32,
        width: u32,
        height: u32,
        atlas_width: u32,
        atlas_height: u32,
    ) -> TextureUV {
        TextureUV {
            begin_u: (column * width) as f32 / atlas_width as f32,
            begin_v: 1.0f32 - ((row + 1) * height) as f32 / atlas_height as f32,
            end_u: ((column + 1) * width) as f32 / atlas_width as f32,
            end_v: 1.0f32 - (row * height) as f32 / atlas_height as f32,
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
