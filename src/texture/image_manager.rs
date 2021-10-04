use std::collections::HashMap;
use std::os::raw::c_void;
use std::path::Path;

use image::{GenericImageView, ImageError};

use gl::Gl;

pub struct ImageManager {
    gl: Gl,
    image_map: HashMap<String, u32>,
}

impl ImageManager {
    pub fn new(gl: Gl) -> ImageManager {
        let image_manager = ImageManager {
            gl,
            image_map: HashMap::new(),
        };
        image_manager
    }

    pub fn load_image<'a>(
        &mut self,
        path: &Path,
        id: &'a str,
        vflip: bool,
    ) -> Result<ImageLoadInfo<'a>, ImageError> {
        let mut image = image::open(path)?;
        let format = match image {
            image::DynamicImage::ImageLuma8(_) => gl::RED,
            image::DynamicImage::ImageLumaA8(_) => gl::RG,
            image::DynamicImage::ImageRgb8(_) => gl::RGB,
            image::DynamicImage::ImageRgba8(_) => gl::RGBA,
            image::DynamicImage::ImageBgr8(_) => gl::RGB,
            image::DynamicImage::ImageBgra8(_) => gl::RGBA,
            image::DynamicImage::ImageLuma16(_) => todo!(),
            image::DynamicImage::ImageLumaA16(_) => todo!(),
            image::DynamicImage::ImageRgb16(_) => todo!(),
            image::DynamicImage::ImageRgba16(_) => todo!(),
        };
        if vflip {
            image = image.flipv();
        }

        let data = image.as_bytes();

        let mut texture = 0;

        unsafe {
            self.gl.GenTextures(1, &mut texture);
            self.gl.BindTexture(gl::TEXTURE_2D, texture);
            self.gl
                .TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            self.gl
                .TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            self.gl
                .TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            self.gl
                .TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            self.gl.TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as i32,
                image.width() as i32,
                image.height() as i32,
                0,
                format,
                gl::UNSIGNED_BYTE,
                &data[0] as *const u8 as *const c_void,
            );
            self.gl.GenerateMipmap(gl::TEXTURE_2D);
            self.gl.BindTexture(gl::TEXTURE_2D, 0);
        }

        self.image_map.insert(id.to_string(), texture);

        Ok(ImageLoadInfo {
            gl_id: texture,
            id,
            width: image.width(),
            height: image.height(),
        })
    }

    pub fn get_texture_id(&mut self, id: &str) -> u32 {
        *self.image_map.get(id).expect("failed to get texture")
    }
}

pub struct ImageLoadInfo<'a> {
    pub gl_id: u32,
    pub id: &'a str,
    pub width: u32,
    pub height: u32,
}
