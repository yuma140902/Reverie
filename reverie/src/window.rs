#[cfg(feature = "winit")]
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use crate::{Context, ContextBackend};

pub struct EventLoop {
    #[cfg(feature = "winit")]
    event_loop: winit::event_loop::EventLoop<()>,
    #[cfg(feature = "winit")]
    queue: Arc<Mutex<VecDeque<winit::event::Event<'static, ()>>>>,
}

#[cfg(feature = "winit")]
impl EventLoop {
    fn new() -> Self {
        let event_loop = winit::event_loop::EventLoop::new();
        let queue = Arc::new(Mutex::new(VecDeque::new()));
        Self { event_loop, queue }
    }

    fn process_event(&mut self) -> bool {
        use winit::platform::run_return::EventLoopExtRunReturn;
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
                _ => (false, true),
            };

            if push_event {
                let mut q = queue.lock().unwrap();
                q.push_back(event.to_static().unwrap());
            }
            if !continue_polling {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }
        });
        exit
    }
}

pub struct Window {
    #[cfg(feature = "winit")]
    event_loop: EventLoop,
    #[cfg(feature = "winit")]
    pub(crate) window: winit::window::Window,
}

impl Window {
    #[cfg(feature = "winit")]
    pub(crate) fn new(title: Option<String>, width: u32, height: u32) -> Self {
        let event_loop = EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .with_title(title.unwrap_or_else(|| "ReverieEngine".to_owned()))
            .with_inner_size(winit::dpi::LogicalSize::new(width as f32, height as f32))
            .build(&event_loop.event_loop)
            .unwrap();

        Self { event_loop, window }
    }

    #[cfg(not(feature = "winit"))]
    pub(crate) fn new() -> Self {
        Self {}
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
    pub fn process_event(&mut self) -> bool {
        self.event_loop.process_event()
    }

    #[cfg(feature = "winit")]
    pub fn get_winit_window(&self) -> &winit::window::Window {
        &self.window
    }

    #[cfg(feature = "winit")]
    pub fn get_winit_window_mut(&mut self) -> &mut winit::window::Window {
        &mut self.window
    }
}

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
