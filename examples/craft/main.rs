use reverie_engine::{
    gl::{self, Gl},
    window::{Context, Window},
};
use winit::event_loop::ControlFlow;

pub fn main() {
    let window = Window::new();
    let context = Context::new(&window);
    let gl = Gl::clone(&context.gl);

    window.run(move |event, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            winit::event::Event::RedrawRequested(_) => {
                context.make_current();

                unsafe {
                    gl.ClearColor(1.0, 0.0, 1.0, 1.0);
                    gl.Clear(gl::COLOR_BUFFER_BIT);
                }

                context.swap_buffers();

                context.make_not_current();
            }
            _ => {}
        }
    });
}
