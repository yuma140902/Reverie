use crate::{camera::Camera, Matrix4, Point3, Vector3};

const VELOCITY_DECAY_RATE: f32 = 0.9;
const MAX_VELOCITY: f32 = 0.5;

#[derive(Debug)]
pub struct Player {
    pub camera: Camera,
    pub pos: Point3,
    pub velocity: Vector3,
}

impl Player {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(),
            pos: Point3::new(4.0, 3.6, 4.0),
            velocity: Vector3::zeros(),
        }
    }

    pub fn update_pos(&mut self) {
        if self.velocity.norm() > MAX_VELOCITY {
            self.velocity.normalize_mut();
            self.velocity *= MAX_VELOCITY;
        }
        self.pos += self.velocity;
        self.velocity *= VELOCITY_DECAY_RATE;
    }

    pub fn view_matrix(&self) -> Matrix4 {
        self.camera.view_matrix(&self.pos)
    }

    pub fn projection_matrix(&self, width: u32, height: u32) -> Matrix4 {
        self.camera.projection_matrix(width, height)
    }
}
