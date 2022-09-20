use crate::Window;

pub struct WindowBuilder {
    title: Option<String>,
    width: u32,
    height: u32,
}

impl WindowBuilder {
    pub(crate) fn new() -> Self {
        Self {
            title: None,
            width: 800,
            height: 600,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    #[cfg(feature = "winit")]
    pub fn build(self) -> Window {
        Window::new(self.title, self.width, self.height)
    }
}
