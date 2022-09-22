use c_str_macro::c_str;
use nalgebra::{Matrix4, Point3};

use crate::{
    gl::{self, Gl},
    shader::UniformVariables,
    ImageLoadInfo, Vao,
};

pub trait Renderer<T> {
    fn render(&self, gl: Gl, vao: &Vao, extra: T);
}

#[derive(Debug)]
pub struct PhongRenderer {}

impl PhongRenderer {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct RenderingInfo<'a> {
    pub model_matrix: &'a Matrix4<f32>,
    pub view_matrix: &'a Matrix4<f32>,
    pub projection_matrix: &'a Matrix4<f32>,
    pub camera_pos: &'a Point3<f32>,
    pub texture: &'a ImageLoadInfo<'a>,
}

impl Renderer<&RenderingInfo<'_>> for PhongRenderer {
    fn render(&self, gl: Gl, vao: &Vao, extra: &RenderingInfo) {
        let uniforms = {
            use crate::shader::Uniform::*;
            let mut uniforms = UniformVariables::new();
            uniforms.add(c_str!("uModel"), Matrix4(&extra.model_matrix));
            uniforms.add(c_str!("uView"), Matrix4(&extra.view_matrix));
            uniforms.add(c_str!("uProjection"), Matrix4(&extra.projection_matrix));
            uniforms.add(
                c_str!("uViewPosition"),
                TripleFloat(extra.camera_pos.x, extra.camera_pos.y, extra.camera_pos.z),
            );
            uniforms
        };

        unsafe {
            gl.BindTexture(gl::TEXTURE_2D, extra.texture.gl_id);
            vao.draw_triangles(&uniforms);
            gl.BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}
