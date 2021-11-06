//! 頂点の情報を動的に追加・削除するためのバッファ

use std::mem;

use super::vao_config::VaoConfig;
use super::Vao;

use crate::gl;
use crate::gl::{types::GLfloat, Gl};

/// 1つの頂点は`VERTEX_SIZE`個のf32から成る。
///
/// * 頂点のx, y, z座標
/// * 頂点が属する面の法線ベクトルのx, y, z成分
/// * テクスチャのu, v座標
pub const VERTEX_SIZE: usize = 8;

/// 頂点の情報を動的に追加・削除するためのバッファ
pub struct VaoBuffer {
    buffer: Vec<f32>,
    vertex_num: i32,
}

impl VaoBuffer {
    /// 空の`VaoBuffer`を作る
    pub fn new() -> Self {
        Self {
            buffer: Vec::<f32>::new(),
            vertex_num: 0,
        }
    }

    /// 初期のバッファーサイズを指定して`VaoBuilder`を作る
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::<f32>::with_capacity(capacity),
            vertex_num: 0,
        }
    }

    /// 頂点群を追加する
    ///
    /// * `v` - 頂点の情報がフラットに繰り返される`Vec`。したがって`v.len()`は`VERTEX_SIZE`の倍数になる。
    /// ※頂点情報の仕様については`VERTEX_SIZE`を参照
    pub fn append(&mut self, v: &mut Vec<f32>) {
        debug_assert_eq!(v.len() % VERTEX_SIZE, 0);
        self.vertex_num += (v.len() / VERTEX_SIZE) as i32;
        self.buffer.append(v);
    }

    /// すべての頂点を削除する
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.vertex_num = self.buffer.len() as i32;
    }

    /// バッファの余分な容量をできるだけ縮める
    pub fn shrink(&mut self) {
        self.buffer.shrink_to_fit();
    }

    /// 現在のバッファの内容をもとに`Vao`を作る
    pub fn build<'a>(&self, gl: &Gl, config: &'a VaoConfig<'a>) -> Vao<'a> {
        Vao::new(
            gl.clone(),
            (self.buffer.len() * mem::size_of::<GLfloat>()) as _,
            self.buffer.as_ptr() as _,
            gl::STATIC_DRAW,
            3usize,
            vec![gl::FLOAT, gl::FLOAT, gl::FLOAT],
            vec![3, 3, 2],
            ((3 + 3 + 2) * mem::size_of::<GLfloat>()) as _,
            self.vertex_num,
            config,
        )
    }
}
