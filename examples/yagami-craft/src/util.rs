use crate::Vector3;

pub fn take_xy_normalized(v: &Vector3) -> Vector3 {
    Vector3::new(v.x, v.y, 0.0).normalize()
}

pub fn take_xz_normalized(v: &Vector3) -> Vector3 {
    Vector3::new(v.x, 0.0, v.z).normalize()
}

pub fn take_yz_normalized(v: &Vector3) -> Vector3 {
    Vector3::new(0.0, v.y, v.z).normalize()
}
