use pollster::FutureExt;
use reverie_engine::ReverieEngine;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
fn main() {
    let subscriber = FmtSubscriber::builder().with_max_level(Level::TRACE).finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    println!("hello");
    tracing::info!("hello tracing");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut engine = ReverieEngine::new(&window).block_on();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { window_id, ref event } if window_id == window.id() => {
            if !engine.input(event) {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        engine.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        engine.resize(**new_inner_size)
                    }
                    _ => {}
                }
            }
        }
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            engine.update();
            match engine.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => engine.resize(engine.size),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(err) if err == wgpu::SurfaceError::Outdated => {
                    tracing::trace!("{:?}", err);
                }
                Err(err) => tracing::warn!("{:?}", err),
            }
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}
