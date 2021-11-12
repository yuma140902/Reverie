//! テクスチャアトラスの仕組みを実現するためのモジュール

use std::marker::PhantomData;

use crate::gui::layout::Rect;
use crate::types::*;

/// テクスチャアトラス内での、始点と終点のUV座標を表す
pub struct TextureUV<Width, Height, AtlasWidth, AtlasHeight> {
    pub begin_u: f32,
    pub begin_v: f32,
    pub end_u: f32,
    pub end_v: f32,
    _pd: PhantomData<fn() -> (Width, Height, AtlasWidth, AtlasHeight)>,
}

impl<Width, Height, AtlasWidth, AtlasHeight> TextureUV<Width, Height, AtlasWidth, AtlasHeight> {
    fn new_internal(rect: &Rect<i32, u32>, texture_width: u32, texture_height: u32) -> Self {
        Self {
            begin_u: *rect.origin_x() as f32 / texture_width as f32,
            begin_v: *rect.origin_y() as f32 / texture_height as f32,
            end_u: (*rect.origin_x() + *rect.width() as i32) as f32 / texture_width as f32,
            end_v: (*rect.origin_y() + *rect.height() as i32) as f32 / texture_height as f32,
            _pd: PhantomData,
        }
    }
}

impl<const W: u32, const H: u32, const ATLAS_W: u32, const ATLAS_H: u32>
    TextureUV<Const<W>, Const<H>, Const<ATLAS_W>, Const<ATLAS_H>>
{
    /// `ATLAS_W`×`ATLAS_H`のテクスチャアトラスを`W`×`H`のサイズに分割した時、`pos.row`行目`pos.column`列目のパーツの始点と終点のUV座標
    pub fn of_atlas(pos: &TextureAtlasPos) -> Self {
        TextureUV {
            begin_u: (pos.column * W) as f32 / ATLAS_W as f32,
            begin_v: 1.0f32 - ((pos.row + 1) * H) as f32 / ATLAS_H as f32,
            end_u: ((pos.column + 1) * W) as f32 / ATLAS_W as f32,
            end_v: 1.0f32 - (pos.row * H) as f32 / ATLAS_H as f32,
            _pd: PhantomData,
        }
    }
}

impl<const ATLAS_W: u32, const ATLAS_H: u32>
    TextureUV<Dynamic, Dynamic, Const<ATLAS_W>, Const<ATLAS_H>>
{
    pub fn new(rect: &Rect<i32, u32>) -> Self {
        Self::new_internal(rect, ATLAS_W, ATLAS_H)
    }
}

impl TextureUV<Dynamic, Dynamic, Dynamic, Dynamic> {
    pub fn new(rect: &Rect<i32, u32>, texture_width: u32, texture_height: u32) -> Self {
        Self::new_internal(rect, texture_width, texture_height)
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
