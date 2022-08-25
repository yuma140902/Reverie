use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
};

use winit::platform::run_return::EventLoopExtRunReturn;

use crate::gl::Gl;

pub struct EventLoop {
    event_loop: winit::event_loop::EventLoop<()>,
    queue: Arc<Mutex<VecDeque<winit::event::Event<'static, ()>>>>,
}

impl EventLoop {
    fn new() -> Self {
        let event_loop = winit::event_loop::EventLoop::new();
        let queue = Arc::new(Mutex::new(VecDeque::new()));
        Self { event_loop, queue }
    }

    fn process_event(&mut self) -> bool {
        let queue = Arc::clone(&self.queue);
        let mut exit = false;
        self.event_loop.run_return(|event, _, control_flow| {
            let (push_event, continue_polling) = match event {
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::CloseRequested,
                    ..
                } => {
                    exit = true;
                    (false, false)
                }
                winit::event::Event::MainEventsCleared => (false, false),
                winit::event::Event::RedrawRequested(_) => (false, false),
                _ => (true, true),
            };

            if push_event {
                let mut q = queue.lock().unwrap();
                q.push_back(event.to_static().unwrap());
            }
            if !continue_polling {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }
        });
        return exit;
    }
}

pub struct Window {
    event_loop: EventLoop,
    window: winit::window::Window,
}

impl Window {
    pub(crate) fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .build(&event_loop.event_loop)
            .unwrap();

        Self { event_loop, window }
    }

    pub fn create_context(&self) -> Context {
        Context::new(&self)
    }

    pub fn process_event(&mut self) -> bool {
        self.event_loop.process_event()
    }
}

pub struct Context {
    context: raw_gl_context::GlContext,
    gl: Gl,
}

impl Context {
    pub fn new(window: &Window) -> Self {
        let context =
            raw_gl_context::GlContext::create(&window.window, raw_gl_context::GlConfig::default())
                .unwrap();
        let gl = Gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);

        Self { context, gl }
    }

    pub fn gl(&self) -> Gl {
        Gl::clone(&self.gl)
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
