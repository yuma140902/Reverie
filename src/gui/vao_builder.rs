use crate::gui::layout::Rect;
use crate::{texture::dynamic_texture_atlas::DynamicTextureUV, vao::VaoBuffer};

pub trait VaoBuilder2DGui {
    fn add_rectangle(&mut self, texture: &DynamicTextureUV, dst: &Rect<i32, u32>);
}

impl VaoBuilder2DGui for VaoBuffer {
    fn add_rectangle(&mut self, texture: &DynamicTextureUV, dst: &Rect<i32, u32>) {
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
}
