use crate::window::WindowBuilder;

#[derive(Debug)]
pub struct ReverieEngine {}

impl ReverieEngine {
    pub fn new() -> Self {
        Self {}
    }

    pub fn window_builder(&self) -> WindowBuilder {
        WindowBuilder::new()
    }
}
