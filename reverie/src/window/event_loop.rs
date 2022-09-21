use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use crate::window::input::Input;

#[derive(Debug)]
pub struct EventLoop {
    #[cfg(feature = "winit")]
    pub(super) event_loop: winit::event_loop::EventLoop<()>,
    #[cfg(feature = "winit")]
    queue: Arc<Mutex<VecDeque<winit::event::Event<'static, ()>>>>,
}

#[cfg(feature = "winit")]
impl EventLoop {
    pub(super) fn new() -> Self {
        let event_loop = winit::event_loop::EventLoop::new();
        let queue = Arc::new(Mutex::new(VecDeque::new()));
        Self { event_loop, queue }
    }

    pub(super) fn process_event(
        &mut self,
        input: &mut Input,
        winit_window: &winit::window::Window,
    ) -> bool {
        use winit::platform::run_return::EventLoopExtRunReturn;
        let queue = Arc::clone(&self.queue);
        let mut exit = false;
        self.event_loop.run_return(|event, _, control_flow| {
            let (push_event, continue_polling) = match event {
                winit::event::Event::WindowEvent { ref event, .. } => match event {
                    winit::event::WindowEvent::CloseRequested => {
                        exit = true;
                        (false, false)
                    }
                    winit::event::WindowEvent::KeyboardInput {
                        input:
                            winit::event::KeyboardInput {
                                state,
                                virtual_keycode: Some(ref virtual_keycode),
                                ..
                            },
                        ..
                    } => {
                        match state {
                            winit::event::ElementState::Pressed => {
                                input.update_key_pressed(virtual_keycode)
                            }
                            winit::event::ElementState::Released => {
                                input.update_key_released(virtual_keycode)
                            }
                        };
                        (false, true)
                    }
                    winit::event::WindowEvent::CursorMoved { position, .. } => {
                        input.update_cursor_position(position.x as i32, position.y as i32);
                        let winpos = winit_window.inner_position().unwrap();
                        let winsize = winit_window.inner_size();
                        #[cfg(windows)]
                        crate::platform::set_cursor_pos(
                            winpos.x + winsize.width as i32 / 2,
                            winpos.y + winsize.height as i32 / 2,
                        )
                        .unwrap();
                        (false, true)
                    }
                    _ => (false, true),
                },
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
