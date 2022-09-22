use crate::{camera::Camera, Matrix4, Point3};

#[derive(Debug)]
pub struct Player {
    pub camera: Camera,
    pub pos: Point3,
}

impl Player {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(),
            pos: Point3::new(4.0, 3.6, 4.0),
        }
    }

    pub fn view_matrix(&self) -> Matrix4 {
        self.camera.view_matrix(&self.pos)
    }

    pub fn projection_matrix(&self, width: u32, height: u32) -> Matrix4 {
        self.camera.projection_matrix(width, height)
    }
}
