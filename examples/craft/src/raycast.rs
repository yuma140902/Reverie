use parry3d::query::{Ray, RayCast};

use crate::{
    camera,
    player::{Player, PLAYER_EYE},
    world::World,
};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum Side {
    Top,
    Bottom,
    XPos,
    XNeg,
    ZPos,
    ZNeg,
}

impl Side {
    pub fn offset(&self, x: u32, y: u32, z: u32) -> (u32, u32, u32) {
        match self {
            Side::Top => (x, y + 1, z),
            Side::Bottom => {
                if y == 0 {
                    (x, 0, z)
                } else {
                    (x, y - 1, z)
                }
            }
            Side::XPos => (x + 1, y, z),
            Side::XNeg => {
                if x == 0 {
                    (0, y, z)
                } else {
                    (x - 1, y, z)
                }
            }
            Side::ZPos => (x, y, z + 1),
            Side::ZNeg => {
                if z == 0 {
                    (x, y, 0)
                } else {
                    (x, y, z - 1)
                }
            }
        }
    }
}

pub fn hit_block(player: &Player, world: &World) -> Option<(u32, u32, u32, Option<Side>)> {
    let eye_pos = player.pos + PLAYER_EYE;
    let (front, _right, _up) = camera::calc_front_right_up(player.camera.yaw, player.camera.pitch);
    let ray = Ray::new(eye_pos, front);

    let mut nearest_toi = std::f32::INFINITY;
    let mut nearest_xyzside = None;
    for (x, y, z, aabb) in world.generate_selection_aabbs().iter() {
        // プレイヤーがブロックに埋まっているとき
        if aabb.contains_local_point(&eye_pos) {
            return Some((*x, *y, *z, None));
        }

        if let Some(result) = aabb.cast_local_ray_and_get_normal(&ray, 3_f32, true) {
            if result.toi < nearest_toi {
                nearest_toi = result.toi;
                let side = if result.normal.y > 0.0 {
                    Some(Side::Top)
                } else if result.normal.y < 0.0 {
                    Some(Side::Bottom)
                } else if result.normal.x > 0.0 {
                    Some(Side::XPos)
                } else if result.normal.x < 0.0 {
                    Some(Side::XNeg)
                } else if result.normal.z > 0.0 {
                    Some(Side::ZPos)
                } else if result.normal.z < 0.0 {
                    Some(Side::ZNeg)
                } else {
                    None
                };
                nearest_xyzside = Some((*x, *y, *z, side));
            }
        }
    }

    nearest_xyzside
}
