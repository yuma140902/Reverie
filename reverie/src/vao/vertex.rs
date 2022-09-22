use crate::gl::{
    self,
    types::{GLenum, GLint},
};

pub trait VertexType {
    fn vertex_size() -> usize;
    fn attribute_types() -> &'static [GLenum];
    fn attribute_sizes() -> &'static [GLint];
}

pub struct VertexWithNormUv;

const VNUV_ATTR_TY: [GLenum; 3] = [gl::FLOAT, gl::FLOAT, gl::FLOAT];
const VNUV_ATTR_SZ: [GLint; 3] = [3, 3, 2];

impl VertexType for VertexWithNormUv {
    fn vertex_size() -> usize {
        // * 頂点のx, y, z座標
        // * 頂点が属する面の法線ベクトルのx, y, z成分
        // * テクスチャのu, v座標
        3 + 3 + 2
    }

    fn attribute_types() -> &'static [GLenum] {
        &VNUV_ATTR_TY
    }

    fn attribute_sizes() -> &'static [GLint] {
        &VNUV_ATTR_SZ
    }
}
