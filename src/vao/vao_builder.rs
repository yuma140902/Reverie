//! `Vao`のビルダー

use std::mem;

use super::Vao;
use crate::gl;
use crate::gl::{types::GLfloat, Gl};
use crate::shader::Program;
use crate::texture::texture_atlas::TextureUV;

type Point3 = nalgebra::Point3<f32>;

/// `Vao`のビルダー
pub struct VaoBuilder<'a> {
    buffer: Vec<f32>,
    vertex_num: i32,
    program: Option<&'a Program>,
}

impl<'a> VaoBuilder<'a> {
    /// 空の`VaoBuilder`を作る
    pub fn new() -> Self {
        Self {
            buffer: Vec::<f32>::new(),
            vertex_num: 0,
            program: None,
        }
    }

    /// 初期のバッファーサイズを指定して`VaoBuilder`を作る
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::<f32>::with_capacity(capacity),
            vertex_num: 0,
            program: None,
        }
    }

    /// 各辺が軸に並行な直方体を追加する
    /// 
    /// `begin`は`end`よりも(-∞, -∞, -∞)に近い
    pub fn add_cuboid<'b>(&mut self, begin: &Point3, end: &Point3, textures: &CuboidTextures<'b>) {
        // 上面
        self.add_face(
            &Point3::new(begin.x, end.y, begin.z),
            &Point3::new(begin.x, end.y, end.z),
            &end,
            &Point3::new(end.x, end.y, begin.z),
            &textures.top,
        );

        // 下面
        self.add_face(
            &Point3::new(end.x, begin.y, begin.z),
            &Point3::new(end.x, begin.y, end.z),
            &Point3::new(begin.x, begin.y, end.z),
            &begin,
            &textures.bottom,
        );

        // 南
        self.add_face(
            &Point3::new(begin.x, end.y, begin.z),
            &Point3::new(begin.x, begin.y, begin.z),
            &Point3::new(begin.x, begin.y, end.z),
            &Point3::new(begin.x, end.y, end.z),
            &textures.south,
        );

        // 北
        self.add_face(
            &Point3::new(end.x, end.y, end.z),
            &Point3::new(end.x, begin.y, end.z),
            &Point3::new(end.x, begin.y, begin.z),
            &Point3::new(end.x, end.y, begin.z),
            &textures.north,
        );

        // 西
        self.add_face(
            &Point3::new(end.x, end.y, begin.z),
            &Point3::new(end.x, begin.y, begin.z),
            &Point3::new(begin.x, begin.y, begin.z),
            &Point3::new(begin.x, end.y, begin.z),
            &textures.west,
        );

        // 東
        self.add_face(
            &Point3::new(begin.x, end.y, end.z),
            &Point3::new(begin.x, begin.y, end.z),
            &Point3::new(end.x, begin.y, end.z),
            &Point3::new(end.x, end.y, end.z),
            &textures.east,
        );
    }

    /// 各辺が軸に並行な長方形を追加する
    /// 
    /// `p1`: 左上, `p2`: 左下, `p3`: 右下, `p4`: 右上
    pub fn add_face(&mut self, p1: &Point3, p2: &Point3, p3: &Point3, p4: &Point3, uv: &TextureUV) {
        let normal = (p3 - p1).cross(&(p2 - p4)).normalize();
        #[rustfmt::skip]
        let mut v: Vec<f32> = vec![
            p1.x, p1.y, p1.z, normal.x, normal.y, normal.z, uv.begin_u, uv.end_v,/* UVはtodo */
            p2.x, p2.y, p2.z, normal.x, normal.y, normal.z, uv.begin_u, uv.begin_v,
            p3.x, p3.y, p3.z, normal.x, normal.y, normal.z, uv.end_u, uv.begin_v,

            p1.x, p1.y, p1.z, normal.x, normal.y, normal.z, uv.begin_u, uv.end_v,
            p3.x, p3.y, p3.z, normal.x, normal.y, normal.z, uv.end_u, uv.begin_v,
            p4.x, p4.y, p4.z, normal.x, normal.y, normal.z, uv.end_u, uv.end_v,
        ];

        self.vertex_num += 6;

        self.buffer.append(&mut v);
    }

    /// 正八面体を追加する
    /// 
    /// `r`は中心から頂点までの距離
    pub fn add_octahedron(&mut self, center: &Point3, r: f32, uv: &TextureUV) {
        #[rustfmt::skip]
        let mut v: Vec<f32> = vec![
            center.x+r, center.y  , center.z  ,  1.0,  1.0,  1.0, uv.begin_u, uv.begin_v,
            center.x  , center.y+r, center.z  ,  1.0,  1.0,  1.0, uv.begin_u, uv.end_v,
            center.x  , center.y  , center.z+r,  1.0,  1.0,  1.0, uv.end_u, uv.end_v,
            
            center.x+r, center.y  , center.z  ,  1.0,  1.0, -1.0, uv.begin_u, uv.begin_v,
            center.x  , center.y  , center.z-r,  1.0,  1.0, -1.0, uv.end_u, uv.end_v,
            center.x  , center.y+r, center.z  ,  1.0,  1.0, -1.0, uv.begin_u, uv.end_v,

            center.x+r, center.y  , center.z  ,  1.0, -1.0,  1.0, uv.begin_u, uv.begin_v,
            center.x  , center.y  , center.z+r,  1.0, -1.0,  1.0, uv.end_u, uv.end_v,
            center.x  , center.y-r, center.z  ,  1.0, -1.0,  1.0, uv.begin_u, uv.end_v,

            center.x+r, center.y  , center.z  ,  1.0, -1.0, -1.0, uv.begin_u, uv.begin_v,
            center.x  , center.y-r, center.z  ,  1.0, -1.0, -1.0, uv.begin_u, uv.end_v,
            center.x  , center.y  , center.z-r,  1.0, -1.0, -1.0, uv.end_u, uv.end_v,

            center.x-r, center.y  , center.z  , -1.0,  1.0,  1.0, uv.begin_u, uv.begin_v,
            center.x  , center.y  , center.z+r, -1.0,  1.0,  1.0, uv.end_u, uv.end_v,
            center.x  , center.y+r, center.z  , -1.0,  1.0,  1.0, uv.begin_u, uv.end_v,

            center.x-r, center.y  , center.z  , -1.0,  1.0, -1.0, uv.begin_u, uv.begin_v,
            center.x  , center.y+r, center.z  , -1.0,  1.0, -1.0, uv.begin_u, uv.end_v,
            center.x  , center.y  , center.z-r, -1.0,  1.0, -1.0, uv.end_u, uv.end_v,

            center.x-r, center.y  , center.z  , -1.0, -1.0,  1.0, uv.begin_u, uv.begin_v,
            center.x  , center.y-r, center.z  , -1.0, -1.0,  1.0, uv.begin_u, uv.end_v,
            center.x  , center.y  , center.z+r, -1.0, -1.0,  1.0, uv.end_u, uv.end_v,

            center.x-r, center.y  , center.z  , -1.0, -1.0, -1.0, uv.begin_u, uv.begin_v,
            center.x  , center.y  , center.z-r, -1.0, -1.0, -1.0, uv.end_u, uv.end_v,
            center.x  , center.y-r, center.z  , -1.0, -1.0, -1.0, uv.begin_u, uv.end_v,
        ];

        self.vertex_num += 24;
        self.buffer.append(&mut v);
    }

    /// 描画に用いるプログラムを指定する
    /// 
    /// # Panics
    /// 
    /// すでに他のプログラムが指定されているとき
    pub fn attatch_program(&mut self, program: &'a Program) {
        if self.program.is_some() {
            panic!("Cannot attatch multiple shader program");
        }
        self.program = Some(program)
    }

    /// `Vao`を作る
    /// 
    /// # Panics
    /// 
    /// 描画に用いるプログラムが指定されていないとき(see: `attatch_program`)
    pub fn build(self, gl: &Gl) -> Vao<'a> {
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
            self.program.unwrap(),
        )
    }
}

/// 直方体の各面のテクスチャを指定するための構造体
/// 
/// OpenGLは同時に1つのテクスチャしかバインドできないので、
/// 各面のテクスチャは同じテクスチャアトラス上にある必要がある
pub struct CuboidTextures<'a> {
    pub top: &'a TextureUV,
    pub bottom: &'a TextureUV,
    pub south: &'a TextureUV,
    pub north: &'a TextureUV,
    pub west: &'a TextureUV,
    pub east: &'a TextureUV,
}
