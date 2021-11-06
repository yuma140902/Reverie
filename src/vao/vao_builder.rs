//! `Vao`のビルダー

use std::mem;

use super::Vao;
use super::vao_config::VaoConfig;
use crate::gl;
use crate::gl::{types::GLfloat, Gl};
use crate::texture::texture_atlas::TextureUV;

type Point3 = nalgebra::Point3<f32>;

/// `Vao`のビルダー
pub struct VaoBuilder {
    buffer: Vec<f32>,
    vertex_num: i32,
}

pub trait VaoBuilder3DGeometry<const W: u32, const H: u32, const ATLAS_W: u32, const ATLAS_H: u32> {
    /// 各辺が軸に並行な直方体を追加する
    /// 
    /// `begin`は`end`よりも(-∞, -∞, -∞)に近い
    fn add_cuboid<'b>(&mut self, begin: &Point3, end: &Point3, textures: &CuboidTextures<'b, W, H, ATLAS_W, ATLAS_H>);

    /// 各辺が軸に並行な長方形を追加する
    /// 
    /// `p1`: 左上, `p2`: 左下, `p3`: 右下, `p4`: 右上
    fn add_face(&mut self, p1: &Point3, p2: &Point3, p3: &Point3, p4: &Point3, uv: &TextureUV<W, H, ATLAS_W, ATLAS_H>);

    /// 正八面体を追加する
    /// 
    /// `r`は中心から頂点までの距離
    fn add_octahedron(&mut self, center: &Point3, r: f32, uv: &TextureUV<W, H, ATLAS_W, ATLAS_H>);
}

impl VaoBuilder {
    /// 空の`VaoBuilder`を作る
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

    /// `Vao`を作る
    pub fn build<'a>(self, gl: &Gl, config: &'a VaoConfig<'a>) -> Vao<'a> {
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

impl<const W: u32, const H: u32, const ATLAS_W: u32, const ATLAS_H: u32> VaoBuilder3DGeometry<W, H, ATLAS_W, ATLAS_H> for VaoBuilder {
    fn add_cuboid<'b>(&mut self, begin: &Point3, end: &Point3, textures: &CuboidTextures<'b, W, H, ATLAS_W, ATLAS_H>) {
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

    fn add_face(&mut self, p1: &Point3, p2: &Point3, p3: &Point3, p4: &Point3, uv: &TextureUV<W, H, ATLAS_W, ATLAS_H>) {
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

    fn add_octahedron(&mut self, center: &Point3, r: f32, uv: &TextureUV<W, H, ATLAS_W, ATLAS_H>) {
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
}

/// 直方体の各面のテクスチャを指定するための構造体
/// 
/// OpenGLは同時に1つのテクスチャしかバインドできないので、
/// 各面のテクスチャは同じテクスチャアトラス上にある必要がある
pub struct CuboidTextures<'a, const W: u32, const H: u32, const ATLAS_W: u32, const ATLAS_H: u32> {
    pub top: &'a TextureUV<W, H, ATLAS_W, ATLAS_H>,
    pub bottom: &'a TextureUV<W, H, ATLAS_W, ATLAS_H>,
    pub south: &'a TextureUV<W, H, ATLAS_W, ATLAS_H>,
    pub north: &'a TextureUV<W, H, ATLAS_W, ATLAS_H>,
    pub west: &'a TextureUV<W, H, ATLAS_W, ATLAS_H>,
    pub east: &'a TextureUV<W, H, ATLAS_W, ATLAS_H>,
}
