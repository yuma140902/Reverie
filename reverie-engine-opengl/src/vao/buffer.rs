//! 頂点の情報を動的に追加・削除するためのバッファ

use std::marker::PhantomData;
use std::mem;

use crate::gl;
use crate::gl::types::{GLenum, GLint};
use crate::gl::{types::GLfloat, Gl};
use crate::vao::Vao;
use crate::vao::VaoConfig;
use crate::vao::VertexType;

/// 頂点の情報を動的に追加・削除するためのバッファ
#[derive(Debug)]
pub struct VaoBuffer<V: VertexType> {
    vertex_size: usize,
    num_attributes: usize,
    attribute_types: &'static [GLenum],
    attribute_sizes: &'static [GLint],
    buffer: Vec<f32>,
    vertex_num: i32,
    _phantom: PhantomData<V>,
}

impl<V: VertexType> Default for VaoBuffer<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: VertexType> VaoBuffer<V> {
    /// 空の[`VaoBuffer`]を作る
    pub fn new() -> Self {
        Self {
            buffer: Vec::<f32>::new(),
            vertex_num: 0,
            vertex_size: V::vertex_size(),
            num_attributes: V::attribute_sizes().len(),
            attribute_sizes: V::attribute_sizes(),
            attribute_types: V::attribute_types(),
            _phantom: PhantomData,
        }
    }

    /// 初期のバッファーサイズを指定して[`VaoBuffer`]を作る
    ///
    /// `num_vertex_to_reserve`個の頂点が確保できるだけの初期容量になる。
    pub fn with_num_vertex(num_vertex_to_reserve: usize) -> Self {
        let vertex_size = V::vertex_size();
        Self {
            buffer: Vec::<f32>::with_capacity(num_vertex_to_reserve * vertex_size),
            vertex_num: 0,
            vertex_size,
            num_attributes: V::attribute_sizes().len(),
            attribute_types: V::attribute_types(),
            attribute_sizes: V::attribute_sizes(),
            _phantom: PhantomData,
        }
    }

    /// 頂点群を追加する
    ///
    /// * `v` - 頂点の情報がフラットに繰り返される`Vec`。したがって`v.len()`は[`VERTEX_SIZE`]の倍数になる。※頂点情報の仕様については`[VERTEX_SIZE`]を参照
    pub fn append(&mut self, v: &mut Vec<f32>) {
        debug_assert_eq!(v.len() % self.vertex_size, 0);
        self.vertex_num += (v.len() / self.vertex_size) as i32;
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

    /// バッファの容量を予め確保する
    ///
    /// 少なくとも`additional_num_vertex`個の頂点が格納できるように確保する。
    pub fn reserve(&mut self, additional_num_vertex: usize) {
        self.buffer
            .reserve(additional_num_vertex * self.vertex_size);
    }

    /// 先頭の`num_vertex_to_preserve`個の頂点以外の頂点を削除する
    pub fn clear_preserving_first(&mut self, num_vertex_to_preserve: usize) {
        self.buffer
            .truncate(num_vertex_to_preserve * self.vertex_size);
    }

    /// 現在のバッファの内容をもとに[`Vao`]を作る
    pub fn build<'a>(&self, gl: &Gl, config: &'a VaoConfig) -> Vao<'a> {
        unsafe {
            Vao::new(
                gl.clone(),
                (self.buffer.len() * mem::size_of::<GLfloat>()) as _,
                self.buffer.as_ptr() as _,
                gl::STATIC_DRAW,
                self.num_attributes,
                self.attribute_types,
                self.attribute_sizes,
                ((3 + 3 + 2) * mem::size_of::<GLfloat>()) as _,
                self.vertex_num,
                config,
            )
        }
    }
}
