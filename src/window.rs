use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use winit::platform::run_return::EventLoopExtRunReturn;

use crate::{Context, ContextBackend};

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
    pub(crate) window: winit::window::Window,
}

impl Window {
    pub(crate) fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .build(&event_loop.event_loop)
            .unwrap();

        Self { event_loop, window }
    }

    #[cfg(feature = "raw_gl_context")]
    pub fn create_context(&self) -> Context<raw_gl_context::GlContext> {
        self.create_context_with_backend::<raw_gl_context::GlContext>()
    }

    pub fn create_context_with_backend<C: ContextBackend>(&self) -> Context<C> {
        Context::new(&self)
    }

    pub fn process_event(&mut self) -> bool {
        self.event_loop.process_event()
    }
}
