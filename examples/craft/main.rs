use std::time::{Duration, Instant};

use reverie_engine::{
    gl::{self, Gl},
    window::{Context, Window},
};

pub fn main() {
    let mut window = Window::new();
    let context = Context::new(&window);
    let gl = Gl::clone(&context.gl);

    let mut start = Instant::now();

    while !window.process_event() {
        let elapsed = start.elapsed();
        if elapsed > Duration::from_millis(16) {
            println!(
                "{} FPS",
                Duration::from_secs(1).as_nanos() as f64 / elapsed.as_nanos() as f64
            );
            context.make_current();
            unsafe {
                gl.ClearColor(1.0, 0.0, 1.0, 1.0);
                gl.Clear(gl::COLOR_BUFFER_BIT);
            }
            context.swap_buffers();
            context.make_not_current();
            start = Instant::now();
        }
    }
}
