use self::builder::WindowConfig;
use crate::gl::Gl;
use crate::{Context, ContextBackend};
use input::Input;

#[cfg(feature = "glutin")]
pub use glutin;
#[cfg(feature = "winit")]
pub use winit;

mod builder;
mod event_loop;
pub(crate) mod input;

pub use builder::WindowBuilder;
pub use event_loop::EventLoop;
pub use input::cursor::{CursorPosition, DesktopOrigin, WindowCenter, WindowOrigin};

#[derive(Debug)]
pub struct Window {
    #[cfg(feature = "winit")]
    event_loop: EventLoop,
    #[cfg(feature = "winit")]
    pub(crate) window: winit::window::Window,
    #[cfg(feature = "winit")]
    input: Input,
    should_stop: bool,
}

impl Window {
    #[cfg(feature = "winit")]
    pub(crate) fn new(config: WindowConfig) -> Self {
        let event_loop = EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .with_title(config.title.unwrap_or_else(|| "ReverieEngine".to_owned()))
            .with_inner_size(winit::dpi::LogicalSize::new(
                config.width as f32,
                config.height as f32,
            ))
            .with_maximized(config.maximize)
            .build(&event_loop.event_loop)
            .unwrap();
        let input = Input::new();

        Self {
            event_loop,
            window,
            input,
            should_stop: false,
        }
    }

    #[cfg(not(feature = "winit"))]
    pub(crate) fn new() -> Self {
        Self { should_stop: false }
    }

    #[cfg(feature = "raw_gl_context")]
    pub fn create_context_raw_gl_context(&self) -> Context<raw_gl_context::GlContext> {
        self.create_context_with_backend::<raw_gl_context::GlContext>()
    }

    #[cfg(feature = "glutin")]
    pub fn create_context_glutin(&self) -> Context<glutin::RawContext<glutin::PossiblyCurrent>> {
        self.create_context_with_backend::<glutin::RawContext<glutin::PossiblyCurrent>>()
    }

    pub fn create_context_with_backend<C: ContextBackend>(&self) -> Context<C> {
        Context::new(self)
    }

    #[cfg(feature = "winit")]
    pub fn update(&mut self, gl: &Gl) {
        self.should_stop = self
            .event_loop
            .process_event(&mut self.input, &self.window, gl);
    }

    pub const fn should_stop(&self) -> bool {
        self.should_stop
    }

    #[cfg(feature = "winit")]
    pub const fn get_winit_window(&self) -> &winit::window::Window {
        &self.window
    }

    #[cfg(feature = "winit")]
    pub fn get_winit_window_mut(&mut self) -> &mut winit::window::Window {
        &mut self.window
    }

    #[cfg(feature = "winit")]
    pub fn keydown(&mut self, keycode: &winit::event::VirtualKeyCode) -> bool {
        self.input.get_keydown(keycode)
    }

    #[cfg(feature = "winit")]
    pub fn keyup(&mut self, keycode: &winit::event::VirtualKeyCode) -> bool {
        self.input.get_keyup(keycode)
    }

    #[cfg(feature = "winit")]
    pub fn keypressed(&self, keycode: &winit::event::VirtualKeyCode) -> bool {
        self.input.get_key_pressed(keycode)
    }

    #[cfg(feature = "winit")]
    pub const fn cursor_pos(&self) -> CursorPosition<DesktopOrigin> {
        self.input.get_cursor_pos()
    }

    #[cfg(feature = "winit")]
    pub const fn cursor_delta(&self) -> (i32, i32) {
        self.input.get_cursor_delta()
    }

    #[cfg(feature = "winit")]
    pub fn mouse_down(&mut self, button: &winit::event::MouseButton) -> bool {
        input::mouse_button_index_3(button).map_or(false, |index| self.input.get_mouse_down(index))
    }

    #[cfg(feature = "winit")]
    pub fn mouse_up(&mut self, button: &winit::event::MouseButton) -> bool {
        input::mouse_button_index_3(button).map_or(false, |index| self.input.get_mouse_up(index))
    }

    #[cfg(feature = "winit")]
    pub fn mouse_pressed(&self, button: &winit::event::MouseButton) -> bool {
        input::mouse_button_index_3(button)
            .map_or(false, |index| self.input.get_mouse_pressed(index))
    }
}
