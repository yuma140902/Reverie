use parry3d::shape::Cuboid;

use crate::{camera::Camera, collision, world::World, Matrix4, Point3, Vector3};

const VELOCITY_DECAY_RATE: f32 = 0.9;
const MAX_VELOCITY: f32 = 0.5;
pub const PLAYER_EYE: Vector3 = Vector3::new(0.0, 0.3, 0.0);

#[derive(Debug)]
pub struct Player {
    pub camera: Camera,
    pub pos: Point3,
    pub velocity: Vector3,
    pub bounding_box: Cuboid,
}

impl Player {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(),
            pos: Point3::new(4.0, 3.6, 4.0),
            velocity: Vector3::zeros(),
            bounding_box: Cuboid::new(Vector3::new(0.15, 0.45, 0.15)),
        }
    }

    pub fn update_pos(&mut self, world: &World) {
        collision::modify_velocity(
            &mut self.velocity,
            &self.bounding_box,
            &self.pos,
            &world.generate_collision_aabbs(),
        );
        if self.velocity.norm() > MAX_VELOCITY {
            self.velocity.normalize_mut();
            self.velocity *= MAX_VELOCITY;
        }
        self.pos += self.velocity;
        self.velocity *= VELOCITY_DECAY_RATE;
    }

    pub fn view_matrix(&self) -> Matrix4 {
        self.camera.view_matrix(&(self.pos + PLAYER_EYE))
    }

    pub fn projection_matrix(&self, width: u32, height: u32) -> Matrix4 {
        self.camera.projection_matrix(width, height)
    }
}
