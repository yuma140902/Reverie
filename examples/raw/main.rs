use std::{ffi::c_void, mem::size_of};

use reverie_engine::{
    gl,
    shader::{Program, Shader},
    ReverieEngine,
};

pub fn main() {
    let engine = ReverieEngine::new();
    let mut window = engine.create_window();
    let context = window.create_context_glutin();
    context.make_current();
    let gl = context.gl();

    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0, //
        0.5, -0.5, 0.0, //
        0.0, 0.5, 0.0, //
    ];

    unsafe {
        //let vert_shader = gl.CreateShader(gl::VERTEX_SHADER);
        //let source = c_str_macro::c_str!(include_str!("./shader.vert"));
        //gl.ShaderSource(vert_shader, 1, &source.as_ptr(), 0 as *const i32);
        //gl.CompileShader(vert_shader);
        let vert_shader =
            Shader::from_vert_code(context.gl(), c_str_macro::c_str!(include_str!("./shader.vert")))
                .unwrap();
        println!("vert shader setup {}", vert_shader.id());

        //let frag_shader = gl.CreateShader(gl::FRAGMENT_SHADER);
        //let source = c_str_macro::c_str!(include_str!("./shader.frag"));
        //gl.ShaderSource(frag_shader, 1, &source.as_ptr(), 0 as *const i32);
        //gl.CompileShader(frag_shader);
        let frag_shader =
            Shader::from_frag_code(context.gl(), c_str_macro::c_str!(include_str!("./shader.frag")))
                .unwrap();
        println!("frag shader setup {}", frag_shader.id());

        //let program = gl.CreateProgram();
        //gl.AttachShader(program, vert_shader);
        //gl.AttachShader(program, frag_shader);
        //gl.LinkProgram(program);
        //gl.DeleteShader(vert_shader);
        //gl.DeleteShader(frag_shader);
        let program = Program::from_shaders(context.gl(), &[vert_shader, frag_shader]).unwrap();
        println!("program setup {}", program.id());

        let mut vao = 0;
        gl.GenVertexArrays(1, &mut vao);
        gl.BindVertexArray(vao);
        println!("vao setup {}", vao);

        let mut vbo = 0;
        gl.GenBuffers(1, &mut vbo);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * size_of::<f32>()) as isize,
            vertices.as_ptr() as _,
            gl::STATIC_DRAW,
        );
        println!("vbo setup {}", vbo);

        gl.VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * size_of::<f32>() as i32,
            0 as *const c_void,
        );
        gl.EnableVertexAttribArray(0);
        println!("VertexAttribPointer setup");

        while !window.process_event() {
            gl.Viewport(0, 0, 800, 600); // TODO: window size
            gl.ClearColor(1.0, 1.0, 0.0, 1.0);
            gl.Clear(gl::COLOR_BUFFER_BIT);

            gl.UseProgram(program.id());
            gl.BindVertexArray(vao);
            gl.DrawArrays(gl::TRIANGLES, 0, 3);

            context.swap_buffers();
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }
}
