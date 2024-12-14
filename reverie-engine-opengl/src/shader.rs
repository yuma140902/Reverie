//! シェーダプログラム

use crate::gl;
use crate::gl::types::*;
use crate::gl::Gl;

use reverie_util::math::nalgebra;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;
use std::str;

/// プログラム
#[derive(Debug)]
pub struct Program {
    gl: Gl,
    id: GLuint,
}

impl Program {
    pub fn default_uv(gl: Gl) -> Result<Self, String> {
        let vert = Shader::from_vert_code(
            Gl::clone(&gl),
            &CString::new(include_str!("../resources/uv.vert")).unwrap(),
        )?;
        let frag = Shader::from_frag_code(
            Gl::clone(&gl),
            &CString::new(include_str!("../resources/uv.frag")).unwrap(),
        )?;
        Self::from_shaders(gl, &[vert, frag])
    }

    /// 頂点シェーダーとフラグメントシェーダーをリンクしてプログラムを作る
    ///
    /// # Returns
    ///
    /// `Ok`のときは`Program`、`Err`のときはエラーメッセージ
    pub fn from_shaders(gl: Gl, shaders: &[Shader]) -> Result<Self, String> {
        let program_id = unsafe { gl.CreateProgram() };
        for shader in shaders {
            unsafe {
                gl.AttachShader(program_id, shader.raw_id());
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
                // as_ptr() の戻り値へ書き込むのは未定義動作である (as_mut_ptr()は用意されていない)
                //
                // > The returned pointer is read-only; writing to it (including passing it to C code that writes to it) causes undefined behavior.
                // https://doc.rust-lang.org/std/ffi/struct.CString.html#method.as_ptr
                //
                // でも動いているのでとりあえずそのまま使う。
                // glow にもそういうコードがある。
                // https://github.com/grovesNL/glow/blob/eb44a878a756d5ddce8505158690ec9bd272be8f/src/native.rs#L422
                #[allow(clippy::as_ptr_cast_mut)]
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
            unsafe { gl.DetachShader(program_id, shader.raw_id()) }
        }
        Ok(Self { gl, id: program_id })
    }

    /// OpenGLの関数に渡すためのプログラムID
    ///
    /// # Safety
    /// glDeleteProgramされていない限り安全
    pub const unsafe fn raw_id(&self) -> GLuint {
        self.id
    }

    /// このプログラムをOpenGLで使うように設定する(glUseProgram)
    pub fn set_used(&self) {
        unsafe {
            self.gl.UseProgram(self.id);
        }
    }

    #[allow(clippy::missing_safety_doc)]
    // TODO: Safetyの説明を書く
    /// ユニフォーム変数を送る
    pub unsafe fn set_uniforms(&self, uniforms: &UniformVariables<'_>) {
        for (name, uniform) in uniforms.map.iter() {
            self.set_uniform(name, uniform);
        }
    }

    #[allow(clippy::missing_safety_doc)]
    // TODO: Safetyの説明を書く
    /// ユニフォーム変数を送る
    pub unsafe fn set_uniform(&self, name: &CStr, value: &Uniform<'_>) {
        match *value {
            Uniform::Bool(b) => self.set_bool(name, b),
            Uniform::Int(i) => self.set_int(name, i),
            Uniform::Float(f) => self.set_float(name, f),
            Uniform::Vector3(v) => self.set_vector3(name, v),
            Uniform::TripleFloat(f1, f2, f3) => self.set_vec3(name, f1, f2, f3),
            Uniform::Matrix4(m) => self.set_mat4(name, m),
        }
    }

    #[allow(clippy::missing_safety_doc)]
    // TODO: Safetyの説明を書く
    /// bool型のユニフォーム変数を送る
    pub unsafe fn set_bool(&self, name: &CStr, value: bool) {
        self.gl.Uniform1i(
            self.gl.GetUniformLocation(self.id, name.as_ptr()),
            value as i32,
        );
    }

    #[allow(clippy::missing_safety_doc)]
    // TODO: Safetyの説明を書く
    /// int型のユニフォーム変数を送る
    pub unsafe fn set_int(&self, name: &CStr, value: i32) {
        self.gl
            .Uniform1i(self.gl.GetUniformLocation(self.id, name.as_ptr()), value);
    }

    #[allow(clippy::missing_safety_doc)]
    // TODO: Safetyの説明を書く
    /// float型のユニフォーム変数を送る
    pub unsafe fn set_float(&self, name: &CStr, value: f32) {
        self.gl
            .Uniform1f(self.gl.GetUniformLocation(self.id, name.as_ptr()), value);
    }

    #[allow(clippy::missing_safety_doc)]
    // TODO: Safetyの説明を書く
    /// 3次元ベクトル型(float)のユニフォーム変数を送る
    pub unsafe fn set_vector3(&self, name: &CStr, value: &nalgebra::Vector3<f32>) {
        self.gl.Uniform3fv(
            self.gl.GetUniformLocation(self.id, name.as_ptr()),
            1,
            value.as_ptr(),
        );
    }

    #[allow(clippy::missing_safety_doc)]
    // TODO: Safetyの説明を書く
    /// float型のユニフォーム変数3つを送る
    pub unsafe fn set_vec3(&self, name: &CStr, x: f32, y: f32, z: f32) {
        self.gl
            .Uniform3f(self.gl.GetUniformLocation(self.id, name.as_ptr()), x, y, z);
    }

    #[allow(clippy::missing_safety_doc)]
    // TODO: Safetyの説明を書く
    /// 4次行列型(float)のユニフォーム変数を送る
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
    /// OpenGLが保持しているプログラムの実体も削除される(glDeleteProgram)
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.id);
        }
    }
}

/// 頂点シェーダーまたはフラグメントシェーダー
#[derive(Debug)]
pub struct Shader {
    gl: Gl,
    id: GLuint,
}

impl Shader {
    /// シェーダのソースコードをコンパイルする
    pub fn from_code(gl: Gl, code: &CStr, kind: GLenum) -> Result<Self, String> {
        let id = unsafe { gl.CreateShader(kind) };

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
                // as_ptr() の戻り値へ書き込むのは未定義動作である (as_mut_ptr()は用意されていない)
                //
                // > The returned pointer is read-only; writing to it (including passing it to C code that writes to it) causes undefined behavior.
                // https://doc.rust-lang.org/std/ffi/struct.CString.html#method.as_ptr
                //
                // でも動いているのでとりあえずそのまま使う。
                // glow にもそういうコードがある。
                // https://github.com/grovesNL/glow/blob/eb44a878a756d5ddce8505158690ec9bd272be8f/src/native.rs#L333
                #[allow(clippy::as_ptr_cast_mut)]
                gl.GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut GLchar);
            }

            return Err(error.to_string_lossy().into_owned());
        }
        Ok(Self { gl, id })
    }

    /// ファイルからシェーダーのコードを読み込み、コンパイルする
    ///
    /// # Panics
    ///
    /// ファイルが開けなかった時にパニックする
    pub fn from_file(gl: Gl, path: &str, kind: GLenum) -> Result<Self, String> {
        let mut file = File::open(path)
            .unwrap_or_else(|err| panic!("ERROR: {} : Failed to open file '{}'", err, path));
        let mut code = String::new();
        file.read_to_string(&mut code)
            .unwrap_or_else(|err| panic!("ERROR: {} : Failed to read file '{}'", err, path));

        let code = CString::new(code.as_bytes()).unwrap();

        Self::from_code(gl, &code, kind)
    }

    /// 頂点シェーダーをファイルから作る
    pub fn from_vert_file(gl: Gl, path: &str) -> Result<Self, String> {
        Self::from_file(gl, path, gl::VERTEX_SHADER)
    }

    /// フラグメントシェーダーをファイルから作る
    pub fn from_frag_file(gl: Gl, path: &str) -> Result<Self, String> {
        Self::from_file(gl, path, gl::FRAGMENT_SHADER)
    }

    /// 頂点シェーダーをソースコードから作る
    pub fn from_vert_code(gl: Gl, code: &CStr) -> Result<Self, String> {
        Self::from_code(gl, code, gl::VERTEX_SHADER)
    }

    /// フラグメントシェーダーをソースコードから作る
    pub fn from_frag_code(gl: Gl, code: &CStr) -> Result<Self, String> {
        Self::from_code(gl, code, gl::FRAGMENT_SHADER)
    }

    /// OpenGLの関数に渡すためのシェーダーID
    ///
    /// # Safety
    /// glDeleteShaderされていない限り安全
    pub const unsafe fn raw_id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader {
    /// OpenGLが保持しているシェーダーの実体も削除される(glDeleteProgram)
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend(std::iter::once(b' ').cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}

/// ユニフォーム変数
#[derive(Debug)]
pub enum Uniform<'a> {
    Bool(bool),
    Int(i32),
    Float(f32),
    Vector3(&'a nalgebra::Vector3<f32>),
    TripleFloat(f32, f32, f32),
    Matrix4(&'a nalgebra::Matrix4<f32>),
}

/// ユニフォーム変数のセット
pub struct UniformVariables<'a> {
    map: HashMap<&'a CStr, Uniform<'a>>,
}

impl Default for UniformVariables<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> UniformVariables<'a> {
    /// 空のセットを作る
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// 名前を指定してユニフォーム変数を追加する
    pub fn add(&mut self, name: &'a CStr, value: Uniform<'a>) -> &mut Self {
        self.map.insert(name, value);
        self
    }
}
