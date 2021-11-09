use crate::gui::layout::Rect;

pub struct DynamicTextureUV {
    pub begin_u: f32,
    pub begin_v: f32,
    pub end_u: f32,
    pub end_v: f32,
}

impl DynamicTextureUV {
    pub fn new(rect: &Rect<i32, u32>, texture_width: u32, texture_height: u32) -> Self {
        Self {
            begin_u: *rect.origin_x() as f32 / texture_width as f32,
            begin_v: *rect.origin_y() as f32 / texture_height as f32,
            end_u: (*rect.origin_x() + *rect.width() as i32) as f32 / texture_width as f32,
            end_v: (*rect.origin_y() + *rect.height() as i32) as f32 / texture_height as f32,
        }
    }
}
