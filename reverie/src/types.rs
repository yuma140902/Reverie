pub use reverie_util::math::nalgebra as na;

pub type Vector3 = na::Vector3<f32>;
pub type Matrix4 = na::Matrix4<f32>;
pub type Point3 = na::Point3<f32>;

/// 定数ジェネリクスの値が定まっていることを表す型
#[derive(Debug)]
pub struct Const<const C: u32>;

/// 定数ジェネリクスの値が定まっていないことを表す型
#[derive(Debug)]
pub struct Dynamic;
