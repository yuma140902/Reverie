use crate::window::{Context, Window};

pub struct ReverieEngine {}

impl ReverieEngine {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_window(&self) -> Window {
        Window::new()
    }

    pub fn create_context(&self, window: &Window) -> Context {
        Context::new(window)
    }
}
