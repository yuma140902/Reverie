use crate::{gl::Gl, texture::ImageManager, window::WindowBuilder};

#[derive(Debug)]
pub struct ReverieEngine {}

impl Default for ReverieEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ReverieEngine {
    pub fn new() -> Self {
        Self {}
    }

    pub fn window_builder(&self) -> WindowBuilder {
        WindowBuilder::new()
    }

    pub fn create_image_manager(&self, gl: Gl) -> ImageManager {
        ImageManager::new(gl)
    }
}
