pub struct TextureUV {
    pub begin_u: f32,
    pub begin_v: f32,
    pub end_u: f32,
    pub end_v: f32,
}

impl TextureUV {
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
