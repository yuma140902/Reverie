use crate::window::Window;

pub struct ReverieEngine {}

impl ReverieEngine {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_window(&self) -> Window {
        Window::new()
    }
}
