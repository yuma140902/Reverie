use std::ops::{Deref, DerefMut};

use crate::gl::Gl;

pub struct Window {
    event_loop: winit::event_loop::EventLoop<()>,
    window: winit::window::Window,
}

impl Window {
    pub fn new() -> Self {
        let event_loop = winit::event_loop::EventLoop::new();
        let window = winit::window::WindowBuilder::new().build(&event_loop).unwrap();

        Self { event_loop, window }
    }

    pub fn run<F>(self, mut handler: F)
    where
        F: 'static + FnMut(winit::event::Event<'_, ()>, &mut winit::event_loop::ControlFlow) -> (),
    {
        self.event_loop
            .run(move |event, _, control_flow| handler(event, control_flow));
    }
}

pub struct Context {
    pub context: raw_gl_context::GlContext,
    pub gl: Gl,
}

impl Context {
    pub fn new(window: &Window) -> Self {
        let context =
            raw_gl_context::GlContext::create(&window.window, raw_gl_context::GlConfig::default())
                .unwrap();
        let gl = Gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);

        Self { context, gl }
    }
}

impl Deref for Context {
    type Target = raw_gl_context::GlContext;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl DerefMut for Context {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.context
    }
}
