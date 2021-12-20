pub trait Window {
    fn new(config: &WindowConfig) -> Self;
    fn show(&mut self);
    fn main_loop(&mut self);
}

#[cfg(target_os = "windows")]
pub fn create_window_depending_on_platform(config: &WindowConfig) -> Result<Box<impl Window>, String> {
    Ok(Box::new(crate::window::ms_windows::MsWindowsWindow::new(config)))
}

#[cfg(not(target_os = "windows"))]
pub fn create_window_depending_on_platform(config: &WindowConfig) -> Result<Box<impl Window>, String> {
    Err("Not Implmented")
}

pub struct WindowConfig {
    pub(crate) title: String,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

pub struct WindowConfigBuilder {
    title: Option<String>,
    width: u32,
    height: u32,
}

impl WindowConfigBuilder {
    pub fn new() -> Self {
        Self {
            title: None,
            width: 400,
            height: 300,
        }
    }

    pub fn title(&mut self, title: String) -> &mut Self {
        self.title = Some(title);
        self
    }

    pub fn width(&mut self, width: u32) -> &mut Self {
        self.width = width;
        self
    }

    pub fn height(&mut self, height: u32) -> &mut Self {
        self.height = height;
        self
    }

    pub fn build(self) -> WindowConfig {
        WindowConfig {
            title: self.title.unwrap_or("Reverie Engine".to_string()),
            width: self.width,
            height: self.height,
        }
    }
}
