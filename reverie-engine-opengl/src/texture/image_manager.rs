//! 画像ファイルを読み込み、管理する

use std::collections::HashMap;
use std::os::raw::c_void;
use std::path::Path;

use image::{DynamicImage, ImageError};

use crate::gl;
use crate::gl::Gl;

/// 画像ファイルを読み込み、管理する
#[derive(Debug)]
pub struct ImageManager {
    gl: Gl,
    image_map: HashMap<String, u32>,
}

impl ImageManager {
    pub(crate) fn new(gl: Gl) -> Self {
        Self {
            gl,
            image_map: HashMap::new(),
        }
    }

    pub fn load_image<'a>(
        &mut self,
        image: DynamicImage,
        id: &'a str,
        vflip: bool,
    ) -> Result<ImageLoadInfo<'a>, ImageError> {
        let mut image = image;
        let format = match image {
            DynamicImage::ImageLuma8(_) => gl::RED,
            DynamicImage::ImageLumaA8(_) => gl::RG,
            DynamicImage::ImageRgb8(_) => gl::RGB,
            DynamicImage::ImageRgba8(_) => gl::RGBA,
            DynamicImage::ImageLuma16(_) => todo!(),
            DynamicImage::ImageLumaA16(_) => todo!(),
            DynamicImage::ImageRgb16(_) => todo!(),
            DynamicImage::ImageRgba16(_) => todo!(),
            DynamicImage::ImageRgb32F(_) => todo!(),
            DynamicImage::ImageRgba32F(_) => todo!(),
            _ => todo!(),
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

    /// バイト列から画像を読み込み、OpenGLにテクスチャとして読み込ませる
    ///
    /// 管理用のIDとして文字列を渡す必要がある
    pub fn load_from_memory<'a>(
        &mut self,
        bytes: &[u8],
        id: &'a str,
        vflip: bool,
    ) -> Result<ImageLoadInfo<'a>, ImageError> {
        let image = image::load_from_memory(bytes)?;
        self.load_image(image, id, vflip)
    }

    /// ファイルから画像を読み込み、OpenGLにテクスチャとして読み込ませる
    ///
    /// 管理用のIDとして文字列を渡す必要がある
    pub fn load_from_file<'a>(
        &mut self,
        path: &Path,
        id: &'a str,
        vflip: bool,
    ) -> Result<ImageLoadInfo<'a>, ImageError> {
        let image = image::open(path)?;
        self.load_image(image, id, vflip)
    }

    /// OpenGLの関数に渡すためのテクスチャIDを得る
    pub fn get_texture_id(&self, id: &str) -> u32 {
        *self.image_map.get(id).expect("failed to get texture")
    }
}

/// 画像読み込みの結果
#[derive(Debug)]
pub struct ImageLoadInfo<'a> {
    /// OpenGLの関数に渡すためのテクスチャID
    gl_id: u32,
    /// `ImageManager`で管理される(ヒューマンリーダブルな)管理用ID
    pub id: &'a str,
    /// 画像の幅
    pub width: u32,
    /// 画像の高さ
    pub height: u32,
}

impl ImageLoadInfo<'_> {
    /// OpenGLの関数に渡すためのテクスチャID
    ///
    /// # Safety
    /// OpenGLにテクスチャ読み込まれている限り安全
    pub const unsafe fn raw_gl_id(&self) -> u32 {
        self.gl_id
    }
}
