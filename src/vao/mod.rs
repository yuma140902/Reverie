//! Vertex Array Object

use std::mem;
use std::os::raw::c_void;

use crate::gl;
use crate::gl::types::{GLenum, GLfloat, GLint, GLsizei, GLsizeiptr};
use crate::gl::Gl;
use crate::shader::{Program, UniformVariables};

use self::vao_config::VaoConfig;

pub mod vao_builder;
pub mod vao_config;

/// OpenGLのVertex Array ObjectとVertex Buffer Objectに対応する構造体
pub struct Vao<'a> {
    gl: Gl,
    vao: u32,
    _vbo: u32,
    vertex_num: i32,
    config: &'a VaoConfig<'a>,
}

impl<'a> Vao<'a> {
    /// 代わりに`VaoBuilder`を使うことを推奨
    pub fn new(
        gl: Gl,
        size: GLsizeiptr,
        data: *const c_void,
        usage: GLenum,
        num_attributes: usize,
        attribute_type_vec: std::vec::Vec<GLenum>,
        attribute_size_vec: std::vec::Vec<GLint>,
        stride: GLsizei,
        vertex_num: i32,
        config: &'a VaoConfig<'a>,
    ) -> Vao {
        assert!(num_attributes == attribute_type_vec.len());
        assert!(num_attributes == attribute_size_vec.len());

        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            // create vertex array object and vertex buffer object
            gl.GenVertexArrays(1, &mut vao);
            gl.GenBuffers(1, &mut vbo);

            // bind buffer
            gl.BindVertexArray(vao);
            gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl.BufferData(gl::ARRAY_BUFFER, size, data, usage);

            let mut offset = 0;
            for i in 0..num_attributes {
                gl.EnableVertexAttribArray(i as u32);
                gl.VertexAttribPointer(
                    i as u32,
                    attribute_size_vec[i],
                    attribute_type_vec[i],
                    gl::FALSE,
                    stride,
                    (offset * mem::size_of::<GLfloat>()) as *const c_void,
                );
                offset += attribute_size_vec[i] as usize;
            }

            // unbind
            gl.BindBuffer(gl::ARRAY_BUFFER, 0);
            gl.BindVertexArray(0);
        }

        Vao {
            gl,
            vao,
            _vbo: vbo,
            vertex_num,
            config,
        }
    }

    pub fn draw(&self, uniforms: &UniformVariables, draw_mode: GLenum) {
        unsafe {
            if let Some(texture) = self.config.texture {
                self.gl.BindTexture(gl::TEXTURE_2D, texture.gl_id);
            }

            if self.config.depth_test {
                self.gl.Enable(gl::DEPTH_TEST);
            } else {
                self.gl.Disable(gl::DEPTH_TEST);
            }

            if self.config.blend {
                self.gl.Enable(gl::BLEND);
                self.gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            } else {
                self.gl.Disable(gl::BLEND);
            }

            if self.config.wireframe {
                self.gl.PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            } else {
                self.gl.PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            }

            if self.config.culling {
                self.gl.Enable(gl::CULL_FACE);
            } else {
                self.gl.Disable(gl::CULL_FACE);
            }

            use crate::shader::Uniform::*;
            use c_str_macro::c_str;
            let program = self.config.program;
            program.set_uniform(c_str!("uAlpha"), &Float(self.config.alpha));
            program.set_uniform(
                c_str!("uMaterial.specular"),
                &Vector3(&self.config.material_specular),
            );
            program.set_uniform(
                c_str!("uMaterial.shininess"),
                &Float(self.config.material_shininess),
            );
            program.set_uniform(
                c_str!("uLight.direction"),
                &Vector3(&self.config.light_direction),
            );
            program.set_uniform(c_str!("uLight.ambient"), &Vector3(&self.config.ambient));
            program.set_uniform(c_str!("uLight.diffuse"), &Vector3(&self.config.diffuse));
            program.set_uniform(c_str!("uLight.specular"), &Vector3(&self.config.specular));

            self.config.program.set_used();
            self.config.program.set_uniforms(uniforms);
            self.gl.BindVertexArray(self.vao);
            self.gl.DrawArrays(draw_mode, 0, self.vertex_num);
            self.gl.BindVertexArray(0);
            self.gl.BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    /// ポリゴンを描画する
    pub fn draw_triangles(&self, uniforms: &UniformVariables) {
        self.draw(uniforms, gl::TRIANGLES);
    }
}
