use parry3d::query::{Ray, RayCast};

use crate::{
    camera,
    player::{Player, PLAYER_EYE},
    world::World,
};

pub fn hit_block(player: &Player, world: &World) -> Option<(u32, u32, u32)> {
    let eye_pos = player.pos + PLAYER_EYE;
    let (front, _right, _up) = camera::calc_front_right_up(player.camera.yaw, player.camera.pitch);
    let ray = Ray::new(eye_pos, front);

    let mut nearest_toi = std::f32::INFINITY;
    let mut nearest_xyz = None;
    for (x, y, z, aabb) in world.generate_selection_aabbs().iter() {
        // プレイヤーがブロックに埋まっているとき
        if aabb.contains_local_point(&eye_pos) {
            return Some((*x, *y, *z));
        }

        if let Some(result) = aabb.cast_local_ray_and_get_normal(&ray, 3_f32, true) {
            if result.toi < nearest_toi {
                nearest_toi = result.toi;
                nearest_xyz = Some((*x, *y, *z));
            }
        }
    }

    nearest_xyz
}
