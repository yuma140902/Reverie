//! [`VaoBuffer`]を操作するユーティリティ

use crate::texture::texture_atlas::TextureUV;
use crate::types::*;
use crate::VertexWithNormUv;

use crate::VaoBuffer;

/// [`VaoBuffer`]上に立方体などの立体を追加する
///
/// * `T` - テクスチャの型
pub trait VaoBuilder3DGeometry<T> {
    /// 各辺が軸に並行な直方体を追加する
    ///
    /// `begin`は`end`よりも(-∞, -∞, -∞)に近い
    fn add_cuboid<'b>(&mut self, begin: &Point3, end: &Point3, textures: &CuboidTextures<'b, T>);

    /// 各辺が軸に並行な長方形を追加する
    ///
    /// `p1`: 左上, `p2`: 左下, `p3`: 右下, `p4`: 右上
    ///
    /// # y軸と平行のとき
    /// +yが「上」
    ///
    /// # y軸と垂直で表面が+yを向いているとき
    /// +xが「上」
    fn add_face(&mut self, p1: &Point3, p2: &Point3, p3: &Point3, p4: &Point3, uv: &T);

    /// 正八面体を追加する
    ///
    /// `r`は中心から頂点までの距離
    fn add_octahedron(&mut self, center: &Point3, r: f32, uv: &T);
}

impl<Width, Height, AtlasWidth, AtlasHeight>
    VaoBuilder3DGeometry<TextureUV<Width, Height, AtlasWidth, AtlasHeight>>
    for VaoBuffer<VertexWithNormUv>
{
    fn add_cuboid<'b>(
        &mut self,
        begin: &Point3,
        end: &Point3,
        textures: &CuboidTextures<'b, TextureUV<Width, Height, AtlasWidth, AtlasHeight>>,
    ) {
        // 上面
        self.add_face(
            &Point3::new(begin.x, end.y, begin.z),
            &Point3::new(begin.x, end.y, end.z),
            end,
            &Point3::new(end.x, end.y, begin.z),
            textures.top,
        );

        // 下面
        self.add_face(
            &Point3::new(end.x, begin.y, begin.z),
            &Point3::new(end.x, begin.y, end.z),
            &Point3::new(begin.x, begin.y, end.z),
            begin,
            textures.bottom,
        );

        // 南
        self.add_face(
            &Point3::new(begin.x, end.y, begin.z),
            &Point3::new(begin.x, begin.y, begin.z),
            &Point3::new(begin.x, begin.y, end.z),
            &Point3::new(begin.x, end.y, end.z),
            textures.south,
        );

        // 北
        self.add_face(
            &Point3::new(end.x, end.y, end.z),
            &Point3::new(end.x, begin.y, end.z),
            &Point3::new(end.x, begin.y, begin.z),
            &Point3::new(end.x, end.y, begin.z),
            textures.north,
        );

        // 西
        self.add_face(
            &Point3::new(end.x, end.y, begin.z),
            &Point3::new(end.x, begin.y, begin.z),
            &Point3::new(begin.x, begin.y, begin.z),
            &Point3::new(begin.x, end.y, begin.z),
            textures.west,
        );

        // 東
        self.add_face(
            &Point3::new(begin.x, end.y, end.z),
            &Point3::new(begin.x, begin.y, end.z),
            &Point3::new(end.x, begin.y, end.z),
            &Point3::new(end.x, end.y, end.z),
            textures.east,
        );
    }

    fn add_face(
        &mut self,
        p1: &Point3,
        p2: &Point3,
        p3: &Point3,
        p4: &Point3,
        uv: &TextureUV<Width, Height, AtlasWidth, AtlasHeight>,
    ) {
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

        self.append(&mut v);
    }

    fn add_octahedron(
        &mut self,
        center: &Point3,
        r: f32,
        uv: &TextureUV<Width, Height, AtlasWidth, AtlasHeight>,
    ) {
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

        self.append(&mut v);
    }
}

/// 直方体の各面のテクスチャを指定するための構造体
///
/// OpenGLは同時に1つのテクスチャしかバインドできないので、
/// 各面のテクスチャは同じテクスチャアトラス上にある必要がある
///
/// * `T` - テクスチャの型
#[derive(Debug)]
pub struct CuboidTextures<'a, T> {
    pub top: &'a T,
    pub bottom: &'a T,
    pub south: &'a T,
    pub north: &'a T,
    pub west: &'a T,
    pub east: &'a T,
}
