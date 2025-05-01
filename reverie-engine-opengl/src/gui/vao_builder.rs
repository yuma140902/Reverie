use crate::gui::layout::{Origin, Position, Rect};
use crate::texture::TextureUV;
use crate::vao::{VaoBuffer, VertexWithNormUv};

/// [`VaoBuffer`]上にGUIを追加する
///
/// * `T` - テクスチャの型
pub trait VaoBuilder2DGui<T> {
    fn add_rectangle(&mut self, texture: &T, dst: &Rect<i32, u32>);
    #[allow(clippy::too_many_arguments)]
    fn add_layout_rectangle(
        &mut self,
        texture: &T,
        parent_width: u32,
        parent_height: u32,
        origin: &Origin,
        position_x: &Position<i32>,
        position_y: &Position<i32>,
        inner_width: u32,
        inner_height: u32,
    );
    fn add_biggest_rectangle(
        &mut self,
        texture: &T,
        parent_width: u32,
        parent_height: u32,
        inner_width: u32,
        inner_height: u32,
    );
}

impl<Width, Height, AtlasWidth, AtlasHeight>
    VaoBuilder2DGui<TextureUV<Width, Height, AtlasWidth, AtlasHeight>>
    for VaoBuffer<VertexWithNormUv>
{
    fn add_rectangle(
        &mut self,
        texture: &TextureUV<Width, Height, AtlasWidth, AtlasHeight>,
        dst: &Rect<i32, u32>,
    ) {
        let x = *dst.origin_x() as f32;
        let y = *dst.origin_y() as f32;
        let w = *dst.width() as f32;
        let h = *dst.height() as f32;

        #[rustfmt::skip]
        let mut vert: Vec<f32> = vec![
            x  , y  , 0.0,  0.0, 0.0, 1.0,  texture.begin_u, texture.begin_v,
            x  , y+h, 0.0,  0.0, 0.0, 1.0,  texture.begin_u, texture.end_v  ,
            x+w, y+h, 0.0,  0.0, 0.0, 1.0,  texture.end_u  , texture.end_v  ,

            x  , y  , 0.0,  0.0, 0.0, 1.0,  texture.begin_u, texture.begin_v,
            x+w, y+h, 0.0,  0.0, 0.0, 1.0,  texture.end_u  , texture.end_v  ,
            x+w, y  , 0.0,  0.0, 0.0, 1.0,  texture.end_u  , texture.begin_v,
        ];
        self.append(&mut vert);
    }

    fn add_layout_rectangle(
        &mut self,
        texture: &TextureUV<Width, Height, AtlasWidth, AtlasHeight>,
        parent_width: u32,
        parent_height: u32,
        origin: &Origin,
        position_x: &Position<i32>,
        position_y: &Position<i32>,
        inner_width: u32,
        inner_height: u32,
    ) {
        let dst = Rect::new_in_rect(
            &Rect::new(0_i32, 0_i32, parent_width, parent_height),
            origin,
            position_x,
            position_y,
            inner_width,
            inner_height,
        );
        self.add_rectangle(texture, &dst);
    }

    fn add_biggest_rectangle(
        &mut self,
        texture: &TextureUV<Width, Height, AtlasWidth, AtlasHeight>,
        parent_width: u32,
        parent_height: u32,
        inner_width: u32,
        inner_height: u32,
    ) {
        let dst = Rect::new_biggest_in_rect(
            &Rect::new(0_i32, 0_i32, parent_width, parent_height),
            inner_width,
            inner_height,
        );
        self.add_rectangle(texture, &dst);
    }
}
