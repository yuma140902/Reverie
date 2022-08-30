use std::time::{Duration, Instant};

use re::gl;
use re::ReverieEngine;
use reverie_engine as re;

pub fn main() {
    let engine = ReverieEngine::new();
    let mut window = engine.create_window();
    let context = window.create_context_glutin();
    context.make_current();
    let gl = context.gl();

    let mut start = Instant::now();

    while !window.process_event() {
        let elapsed = start.elapsed();
        println!(
            "{} FPS",
            Duration::from_secs(1).as_nanos() as f64 / elapsed.as_nanos() as f64
        );
        unsafe {
            gl.Viewport(0, 0, 800, 600); // TODO: window size (#26)
            gl.ClearColor(1.0, 0.0, 1.0, 1.0);
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        // rendering code here

        context.swap_buffers();
        start = Instant::now();
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}
