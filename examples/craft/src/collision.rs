use parry3d::{
    bounding_volume::{BoundingVolume, AABB},
    na::Isometry3,
    query::{Ray, RayCast},
    shape::Cuboid,
};

use crate::{Point3, Vector3};

pub fn modify_velocity(
    entity_velocity: &mut Vector3,
    entity_bounding_box: &Cuboid,
    entity_pos: &Point3,
    world_aabbs: &Vec<AABB>,
) {
    let entity_aabb = entity_bounding_box.aabb(&Isometry3::new(entity_pos.coords, Vector3::zeros()));

    // extended_aabbは対象のAABBと中心が同じで、エンティティの大きさの分大きくなったもの。
    // エンティティと対象のAABBとの当たり判定ではなく、
    // エンティティの中心点とextended_aabbとの当たり判定を行えば良い。
    let extended_aabbs: Vec<_> = world_aabbs
        .iter()
        .map(|aabb| {
            AABB::from_half_extents(aabb.center(), aabb.half_extents() + entity_aabb.half_extents())
        })
        .collect();

    while modify(
        world_aabbs,
        &extended_aabbs,
        entity_pos,
        entity_velocity,
        &entity_aabb,
    ) {}
}

fn modify<'a>(
    world_aabbs: &Vec<AABB>,
    extended_aabbs: &Vec<AABB>,
    entity_pos: &Point3,
    entity_velocity: &mut Vector3,
    entity_aabb: &AABB,
) -> bool {
    // 現在のエンティティのAABBと、次のフレームでのエンティティのAABBを両方とも含むようなAABB
    let extended_entity_aabb = entity_aabb.merged(&AABB::from_half_extents(
        entity_pos + *entity_velocity,
        entity_aabb.half_extents(),
    ));

    let mut nearest_toi = std::f32::INFINITY;
    let mut nearest_normal: Option<Vector3> = None;
    for (aabb, extended_aabb) in world_aabbs.iter().zip(extended_aabbs.iter()) {
        // エンティティが対象のAABBの中にいるときは当たり判定を行わない
        if extended_aabb.contains_local_point(entity_pos) {
            continue;
        }

        // エンティティの行き先が対象のAABBと重ならないときは当たり判定を行わない
        if !extended_entity_aabb.intersects(aabb) {
            continue;
        }

        if let Some(result) = extended_aabb.cast_local_ray_and_get_normal(
            &Ray::new(*entity_pos, *entity_velocity),
            50f32, /*適当な値*/
            true,  /*意味が分からない*/
        ) {
            if result.toi < nearest_toi {
                nearest_toi = result.toi;
                nearest_normal = Some(result.normal);
            }
        }
    }

    // 壁ずりベクトルを求める
    if let Some(nearest_normal) = nearest_normal {
        *entity_velocity -= nearest_normal * entity_velocity.dot(&nearest_normal);
        true
    } else {
        false
    }
}
