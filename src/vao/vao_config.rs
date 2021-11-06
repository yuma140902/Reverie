//! `Vao`の設定
use crate::shader::Program;
use crate::texture::image_manager::ImageLoadInfo;
use crate::types::*;

/// `Vao`の設定
#[derive(Clone, Copy)]
pub struct VaoConfig<'a> {
    /// シェーダープログラム
    pub(crate) program: &'a Program,
    pub(crate) depth_test: bool,
    pub(crate) blend: bool,
    pub(crate) wireframe: bool,
    pub(crate) culling: bool,
    pub(crate) alpha: f32,
    pub(crate) material_specular: Vector3,
    pub(crate) material_shininess: f32,
    pub(crate) light_direction: Vector3,
    pub(crate) ambient: Vector3,
    pub(crate) diffuse: Vector3,
    pub(crate) specular: Vector3,
    pub(crate) texture: Option<&'a ImageLoadInfo<'a>>,
}

/// `VaoConfig`のビルダー
pub struct VaoConfigBuilder<'a> {
    program: &'a Program,
    depth_test: bool,
    blend: bool,
    wireframe: bool,
    culling: bool,
    alpha: f32,
    material_specular: Vector3,
    material_shininess: f32,
    light_direction: Vector3,
    ambient: Vector3,
    diffuse: Vector3,
    specular: Vector3,
    texture: Option<&'a ImageLoadInfo<'a>>,
}

impl<'a> VaoConfigBuilder<'a> {
    pub fn new(program: &'a Program) -> Self {
        Self {
            program,
            depth_test: true,
            blend: true,
            wireframe: false,
            culling: true,
            alpha: 1.0_f32,
            material_specular: Vector3::new(0.2, 0.2, 0.2),
            material_shininess: 0.1_f32,
            light_direction: Vector3::new(0.2, 1.0, 0.2),
            ambient: Vector3::new(0.3, 0.3, 0.3),
            diffuse: Vector3::new(0.5, 0.5, 0.5),
            specular: Vector3::new(0.2, 0.2, 0.2),
            texture: None,
        }
    }

    pub fn build(self) -> VaoConfig<'a> {
        VaoConfig {
            program: self.program,
            depth_test: self.depth_test,
            blend: self.blend,
            wireframe: self.wireframe,
            culling: self.culling,
            alpha: self.alpha,
            material_specular: self.material_specular,
            material_shininess: self.material_shininess,
            light_direction: self.light_direction,
            ambient: self.ambient,
            diffuse: self.diffuse,
            specular: self.specular,
            texture: self.texture,
        }
    }

    pub fn depth_test(mut self, value: bool) -> Self {
        self.depth_test = value;
        self
    }

    pub fn blend(mut self, value: bool) -> Self {
        self.blend = value;
        self
    }

    pub fn wireframe(mut self, value: bool) -> Self {
        self.wireframe = value;
        self
    }

    pub fn culling(mut self, value: bool) -> Self {
        self.culling = value;
        self
    }

    pub fn alpha(mut self, value: f32) -> Self {
        self.alpha = value;
        self
    }

    pub fn material_specular(mut self, value: Vector3) -> Self {
        self.material_specular = value;
        self
    }

    pub fn material_shininess(mut self, value: f32) -> Self {
        self.material_shininess = value;
        self
    }

    pub fn light_direction(mut self, value: Vector3) -> Self {
        self.light_direction = value;
        self
    }

    pub fn ambient(mut self, value: Vector3) -> Self {
        self.ambient = value;
        self
    }

    pub fn diffuse(mut self, value: Vector3) -> Self {
        self.diffuse = value;
        self
    }

    pub fn specular(mut self, value: Vector3) -> Self {
        self.specular = value;
        self
    }

    pub fn texture(mut self, value: &'a ImageLoadInfo<'a>) -> Self {
        self.texture = Some(value);
        self
    }

    pub fn no_texture(mut self) -> Self {
        self.texture = None;
        self
    }
}
