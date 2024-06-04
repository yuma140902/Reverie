use crate::window::Window;

pub struct WindowConfig {
    pub(crate) title: Option<String>,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) maximize: bool,
}

impl WindowConfig {
    pub(crate) fn new() -> Self {
        Self {
            title: None,
            width: 800,
            height: 600,
            maximize: false,
        }
    }
}

pub struct WindowBuilder {
    config: WindowConfig,
}

impl WindowBuilder {
    pub(crate) fn new() -> Self {
        Self {
            config: WindowConfig::new(),
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.config.title = Some(title.into());
        self
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.config.width = width;
        self.config.height = height;
        self
    }

    pub fn maximize(mut self) -> Self {
        self.config.maximize = true;
        self
    }

    #[cfg(feature = "winit")]
    pub fn build(self) -> Window {
        Window::new(self.config)
    }
}
