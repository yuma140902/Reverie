use parry3d::shape::Cuboid;

use crate::{camera::Camera, collision, config, world::World, Matrix4, Point3, Vector3};

#[derive(Debug)]
pub struct Player {
    pub camera: Camera,
    pub pos: Point3,
    pub velocity: Vector3,
    pub bounding_box: Cuboid,
    pub eye: Vector3,
}

impl Player {
    pub fn new() -> Self {
        let config = config::get();

        Self {
            camera: Camera::new(),
            pos: config.player_init_pos.clone(),
            velocity: Vector3::zeros(),
            bounding_box: Cuboid::new(config.player_bounding_vec),
            eye: config.eye,
        }
    }

    pub fn camera_pos(&self) -> Point3 {
        self.pos + self.eye
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
        self.velocity *= config.velocity_decay_rate;
    }

    pub fn view_matrix(&self) -> Matrix4 {
        self.camera.view_matrix(&self.camera_pos())
    }

    pub fn projection_matrix(&self, width: u32, height: u32) -> Matrix4 {
        self.camera.projection_matrix(width, height)
    }
}
