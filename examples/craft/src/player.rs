use parry3d::shape::Cuboid;
use reverie_engine_opengl::{camera::Camera, gl::Gl, math::Deg};

use crate::{collision, config, world::World, Point3, Vector3};

#[derive(Debug)]
pub struct Player {
    pub camera: Camera,
    pub pos: Point3,
    pub velocity: Vector3,
    pub bounding_box: Cuboid,
    pub eye: Vector3,
}

impl Player {
    pub fn new(gl: Gl) -> Self {
        let config = config::get();

        Self {
            camera: Camera::new(
                gl,
                config.player_init_pos + config.eye,
                Deg(config.player_init_yaw_deg).to_rad(),
                Deg(config.player_init_pitch_deg).to_rad(),
                Deg(config.fov),
            ),
            pos: config.player_init_pos,
            velocity: Vector3::zeros(),
            bounding_box: Cuboid::new(config.player_bounding_vec),
            eye: config.eye,
        }
    }

    pub fn update_pos(&mut self, world: &World) {
        let config = config::get();

        collision::modify_velocity(
            &mut self.velocity,
            &self.bounding_box,
            &self.pos,
            &world.generate_collision_aabbs(),
        );
        if self.velocity.norm() > config.max_velocity {
            self.velocity.normalize_mut();
            self.velocity *= config.max_velocity;
        }
        self.pos += self.velocity;
        self.camera.set_pos(self.pos + self.eye);
        self.velocity *= config.velocity_decay_rate;
    }
}
