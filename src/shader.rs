use crate::gl;
use crate::gl::types::*;
use crate::gl::Gl;

use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;
use std::str;

pub struct Program {
    gl: Gl,
    id: GLuint,
}

impl Program {
    pub fn from_shaders(gl: Gl, shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl.CreateProgram() };
        for shader in shaders {
            unsafe {
                gl.AttachShader(program_id, shader.id());
            }
        }

        unsafe {
            gl.LinkProgram(program_id);
        }

        let mut success: GLint = 1;
        unsafe {
            gl.GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }
        if success == 0 {
            let mut len: GLint = 0;
            unsafe { gl.GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len) }
            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl.GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut GLchar,
                )
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe { gl.DetachShader(program_id, shader.id()) }
        }
        Ok(Program { gl, id: program_id })
    }

    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            self.gl.UseProgram(self.id);
        }
    }

    pub unsafe fn set_bool(&self, name: &CStr, value: bool) {
        self.gl.Uniform1i(
            self.gl.GetUniformLocation(self.id, name.as_ptr()),
            value as i32,
        );
    }

    pub unsafe fn set_int(&self, name: &CStr, value: i32) {
        self.gl
            .Uniform1i(self.gl.GetUniformLocation(self.id, name.as_ptr()), value);
    }

    pub unsafe fn set_float(&self, name: &CStr, value: f32) {
        self.gl
            .Uniform1f(self.gl.GetUniformLocation(self.id, name.as_ptr()), value);
    }

    pub unsafe fn set_vector3(&self, name: &CStr, value: &nalgebra::Vector3<f32>) {
        self.gl.Uniform3fv(
            self.gl.GetUniformLocation(self.id, name.as_ptr()),
            1,
            value.as_ptr(),
        );
    }

    pub unsafe fn set_vec3(&self, name: &CStr, x: f32, y: f32, z: f32) {
        self.gl
            .Uniform3f(self.gl.GetUniformLocation(self.id, name.as_ptr()), x, y, z);
    }

    pub unsafe fn set_mat4(&self, name: &CStr, mat: &nalgebra::Matrix4<f32>) {
        self.gl.UniformMatrix4fv(
            self.gl.GetUniformLocation(self.id, name.as_ptr()),
            1,
            gl::FALSE,
            mat.as_ptr(),
        );
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.id);
        }
    }
}

pub struct Shader {
    gl: Gl,
    id: GLuint,
}

impl Shader {
    pub fn from_file(gl: Gl, path: &str, kind: GLenum) -> Result<Shader, String> {
        let id = unsafe { gl.CreateShader(kind) };

        let mut file = File::open(path)
            .unwrap_or_else(|err| panic!("ERROR: {} : Failed to open file '{}'", err, path));
        let mut code = String::new();
        file.read_to_string(&mut code)
            .unwrap_or_else(|err| panic!("ERROR: {} : Failed to read file '{}'", err, path));

        let code = CString::new(code.as_bytes()).unwrap();

        unsafe {
            gl.ShaderSource(id, 1, &code.as_ptr(), std::ptr::null());
            gl.CompileShader(id);
        }
        let mut success: GLint = 1;
        unsafe {
            gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        /* コンパイル失敗の時 */
        if success == 0 {
            let mut len: GLint = 0;
            unsafe {
                gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);
            unsafe {
                gl.GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut GLchar);
            }

            return Err(error.to_string_lossy().into_owned());
        }
        Ok(Shader { gl, id })
    }

    pub fn from_vert_file(gl: Gl, path: &str) -> Result<Shader, String> {
        Shader::from_file(gl, path, gl::VERTEX_SHADER)
    }

    pub fn from_frag_file(gl: Gl, path: &str) -> Result<Shader, String> {
        Shader::from_file(gl, path, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}
