use crate::window::WindowBuilder;

pub struct ReverieEngine {}

impl ReverieEngine {
    pub fn new() -> Self {
        Self {}
    }

    pub fn window_builder(&self) -> WindowBuilder {
        WindowBuilder::new()
    }
}
