use std::time::{Duration, Instant};

use re::gl;
use re::ReverieEngine;
use reverie_engine as re;

pub fn main() {
    let width = 800;
    let height = 600;
    let engine = ReverieEngine::new();
    let mut window = engine
        .window_builder()
        .title("example-window")
        .size(width, height)
        .build();
    let context = window.create_context_glutin();
    context.make_current();
    let gl = context.gl();

    let mut start = Instant::now();

    while !window.should_stop() {
        window.update(&gl);
        let elapsed = start.elapsed();
        println!(
            "{} FPS",
            Duration::from_secs(1).as_nanos() as f64 / elapsed.as_nanos() as f64
        );
        unsafe {
            gl.Viewport(0, 0, width as i32, height as i32);
            gl.ClearColor(1.0, 0.0, 1.0, 1.0);
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        // rendering code here

        context.swap_buffers();
        start = Instant::now();
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}
