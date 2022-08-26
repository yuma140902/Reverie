use std::time::{Duration, Instant};

use re::gl;
use re::shader::UniformVariables;
use re::vao::VaoBuffer;
use re::vao::VaoConfigBuilder;
use re::Program;
use re::ReverieEngine;
use reverie_engine as re;

pub fn main() {
    let engine = ReverieEngine::new();
    let mut window = engine.create_window();
    let context = window.create_context_glutin();
    context.make_current();
    let gl = context.gl();

    let program = Program::default_uv(context.gl()).unwrap();
    let config = VaoConfigBuilder::new(&program).build();
    let vao_empty = VaoBuffer::new().build(&gl, &config);

    let mut start = Instant::now();

    while !window.process_event() {
        let elapsed = start.elapsed();
        if elapsed > Duration::from_millis(16) {
            println!(
                "{} FPS",
                Duration::from_secs(1).as_nanos() as f64 / elapsed.as_nanos() as f64
            );
            vao_empty.draw_triangles(&UniformVariables::new());
            unsafe {
                gl.ClearColor(1.0, 0.0, 1.0, 1.0);
                gl.Clear(gl::COLOR_BUFFER_BIT);
            }
            context.swap_buffers();
            start = Instant::now();
        }
    }
}
