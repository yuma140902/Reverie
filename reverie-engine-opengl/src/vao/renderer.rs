use c_str_macro::c_str;
use reverie_util::math::nalgebra::{Matrix4, Point3, Vector3};

use crate::{
    gl::{self, Gl},
    shader::{Program, Shader, Uniform::*, UniformVariables},
    texture::ImageLoadInfo,
    vao::Vao,
};

pub trait Renderer<T> {
    fn render(&self, gl: Gl, vao: &Vao, extra: T);
}

#[derive(Debug)]
pub struct Phong3DRenderer {
    program: Program,
}

impl Phong3DRenderer {
    pub const fn new(program: Program) -> Self {
        Self { program }
    }
}

pub struct PhongRenderingInfo<'a> {
    /* ベクトルではなく色 */
    pub material_specular: &'a Vector3<f32>,
    pub material_shininess: f32,
    pub light_direction: &'a Vector3<f32>,
    /* ambient, diffuse, specular はベクトルではなく色 */
    pub ambient: &'a Vector3<f32>,
    pub diffuse: &'a Vector3<f32>,
    pub specular: &'a Vector3<f32>,
    pub alpha: f32,
}

pub struct Phong3DRenderingInfo<'a> {
    pub phong: &'a PhongRenderingInfo<'a>,
    pub model_matrix: &'a Matrix4<f32>,
    pub view_matrix: &'a Matrix4<f32>,
    pub projection_matrix: &'a Matrix4<f32>,
    pub camera_pos: &'a Point3<f32>,
    pub texture: &'a ImageLoadInfo<'a>,
}

impl Renderer<&Phong3DRenderingInfo<'_>> for Phong3DRenderer {
    fn render(&self, gl: Gl, vao: &Vao, extra: &Phong3DRenderingInfo) {
        let uniforms = {
            use crate::shader::Uniform::*;
            let mut uniforms = UniformVariables::new();
            uniforms.add(c_str!("uModel"), Matrix4(extra.model_matrix));
            uniforms.add(c_str!("uView"), Matrix4(extra.view_matrix));
            uniforms.add(c_str!("uProjection"), Matrix4(extra.projection_matrix));
            uniforms.add(
                c_str!("uViewPosition"),
                TripleFloat(extra.camera_pos.x, extra.camera_pos.y, extra.camera_pos.z),
            );
            uniforms
        };

        unsafe {
            self.program
                .set_uniform(c_str!("uAlpha"), &Float(extra.phong.alpha));
            self.program.set_uniform(
                c_str!("uMaterial.specular"),
                &Vector3(extra.phong.material_specular),
            );
            self.program.set_uniform(
                c_str!("uMaterial.shininess"),
                &Float(extra.phong.material_shininess),
            );
            self.program.set_uniform(
                c_str!("uLight.direction"),
                &Vector3(extra.phong.light_direction),
            );
            self.program
                .set_uniform(c_str!("uLight.ambient"), &Vector3(extra.phong.ambient));
            self.program
                .set_uniform(c_str!("uLight.diffuse"), &Vector3(extra.phong.diffuse));
            self.program
                .set_uniform(c_str!("uLight.specular"), &Vector3(extra.phong.specular));

            self.program.set_used();
            self.program.set_uniforms(&uniforms);
        }

        unsafe {
            gl.BindTexture(gl::TEXTURE_2D, extra.texture.raw_gl_id());
            vao.draw_triangles(&uniforms);
            gl.BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

#[derive(Debug)]
pub struct Color3DRenderer {
    program: Program,
}

impl Color3DRenderer {
    pub fn new(gl: &Gl) -> Self {
        let vert_shader = Shader::from_vert_code(
            gl.clone(),
            c_str!(include_str!("../../resources/color3d.vert")),
        )
        .unwrap();
        let frag_shader = Shader::from_frag_code(
            gl.clone(),
            c_str!(include_str!("../../resources/color3d.frag")),
        )
        .unwrap();
        let program = Program::from_shaders(gl.clone(), &[vert_shader, frag_shader]).unwrap();
        Self { program }
    }
}

pub struct Color3DRenderingInfo<'a> {
    pub model_matrix: &'a Matrix4<f32>,
    pub view_matrix: &'a Matrix4<f32>,
    pub projection_matrix: &'a Matrix4<f32>,
    pub camera_pos: &'a Point3<f32>,
}

impl Renderer<&Color3DRenderingInfo<'_>> for Color3DRenderer {
    fn render(&self, gl: Gl, vao: &Vao, extra: &Color3DRenderingInfo) {
        let uniforms = {
            use crate::shader::Uniform::*;
            let mut uniforms = UniformVariables::new();
            uniforms.add(c_str!("uModel"), Matrix4(extra.model_matrix));
            uniforms.add(c_str!("uView"), Matrix4(extra.view_matrix));
            uniforms.add(c_str!("uProjection"), Matrix4(extra.projection_matrix));
            uniforms.add(
                c_str!("uViewPosition"),
                TripleFloat(extra.camera_pos.x, extra.camera_pos.y, extra.camera_pos.z),
            );
            uniforms
        };

        unsafe {
            self.program.set_used();
            self.program.set_uniforms(&uniforms);
        }

        unsafe {
            gl.BindTexture(gl::TEXTURE_2D, 0);
            vao.draw(&uniforms, gl::LINES);
        }
    }
}
