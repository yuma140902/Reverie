use c_str_macro::c_str;
use re::gl;
use re::shader::Program;
use re::shader::Shader;
use re::shader::UniformVariables;
use re::types::Const;
use re::CuboidTextures;
use re::ImageManager;
use re::ReverieEngine;
use re::TextureAtlasPos;
use re::VaoConfigBuilder;
use reverie_engine as re;

mod camera;
mod world;
use camera::Camera;
use world::World;

#[allow(dead_code)]
type Point3 = nalgebra::Point3<f32>;
type Vector3 = nalgebra::Vector3<f32>;
type Matrix4 = nalgebra::Matrix4<f32>;

pub type TextureUV = re::TextureUV<Const<64>, Const<64>, Const<256>, Const<256>>;

fn main() {
    let width = 900;
    let height = 480;

    let engine = ReverieEngine::new();
    let mut window = engine.window_builder().title("Craft").size(width, height).build();
    let context = window.create_context_glutin();
    let gl = context.gl();

    let vert_shader =
        Shader::from_vert_code(gl.clone(), c_str!(include_str!("../resources/shader.vs"))).unwrap();
    let frag_shader =
        Shader::from_frag_code(gl.clone(), c_str!(include_str!("../resources/shader.fs"))).unwrap();
    let shader = Program::from_shaders(gl.clone(), &[vert_shader, frag_shader]).unwrap();

    let mut image_manager = ImageManager::new(gl.clone());
    let image = image::load_from_memory(include_bytes!("../resources/blocks.png")).unwrap();
    let block_atlas_texture = image_manager.load_image(image, "atlas/blocks", true).unwrap();

    let top_texture = TextureUV::of_atlas(&TextureAtlasPos::new(0, 1));
    let bottom_texture = TextureUV::of_atlas(&TextureAtlasPos::new(0, 2));
    let side_texture = TextureUV::of_atlas(&TextureAtlasPos::new(0, 0));
    let cuboid_texture = CuboidTextures {
        top: &top_texture,
        bottom: &bottom_texture,
        north: &side_texture,
        south: &side_texture,
        west: &side_texture,
        east: &side_texture,
    };

    let world = {
        let mut world = World::new();
        for i in 0..16 {
            for j in 0..16 {
                world.set_block(i, 0, j);
                world.set_block(0, i, j);
                world.set_block(i, j, 0);
            }
        }
        for i in 1..15 {
            world.set_block(i, i, 15);
        }
        world.set_block(3, 3, 3);
        world
    };

    let vao_config = {
        let depth_test = true;
        let blend = true;
        let wireframe = false;
        let culling = true;
        let alpha: f32 = 1.0;
        /* ベクトルではなく色 */
        let material_specular = Vector3::new(0.2, 0.2, 0.2);
        let material_shininess: f32 = 0.1;
        let light_direction = Vector3::new(1.0, 1.0, 0.0);
        /* ambient, diffuse, specular はベクトルではなく色 */
        let ambient = Vector3::new(0.3, 0.3, 0.3);
        let diffuse = Vector3::new(0.5, 0.5, 0.5);
        let specular = Vector3::new(0.2, 0.2, 0.2);
        VaoConfigBuilder::new(&shader)
            .depth_test(depth_test)
            .blend(blend)
            .wireframe(wireframe)
            .culling(culling)
            .alpha(alpha)
            .material_specular(material_specular)
            .material_shininess(material_shininess)
            .light_direction(light_direction)
            .ambient(ambient)
            .diffuse(diffuse)
            .specular(specular)
            .build()
    };

    let vertex_obj = world.generate_vertex_obj(&gl, &cuboid_texture, &vao_config);

    let camera = Camera::new();

    while !window.process_event() {
        unsafe {
            gl.Viewport(0, 0, width as i32, height as i32);

            gl.ClearColor(1.0, 1.0, 1.0, 1.0);
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        if window.keydown(&winit::event::VirtualKeyCode::Space) {
            println!("keydown!!");
        }
        if window.keyup(&winit::event::VirtualKeyCode::Space) {
            println!("keyup!!");
        }
        if window.keypressed(&winit::event::VirtualKeyCode::Space) {
            println!("keypressed!!");
        }

        let model_matrix =
            nalgebra_glm::scale(&Matrix4::identity(), &Vector3::new(0.5_f32, 0.5_f32, 0.5_f32));
        let view_matrix = camera.view_matrix();
        let projection_matrix: Matrix4 = camera.projection_matrix(width, height);

        let uniforms = {
            use re::shader::Uniform::*;
            let mut uniforms = UniformVariables::new();
            uniforms.add(c_str!("uModel"), Matrix4(&model_matrix));
            uniforms.add(c_str!("uView"), Matrix4(&view_matrix));
            uniforms.add(c_str!("uProjection"), Matrix4(&projection_matrix));
            uniforms.add(
                c_str!("uViewPosition"),
                TripleFloat(camera.pos.x, camera.pos.y, camera.pos.z),
            );
            uniforms
        };

        unsafe {
            gl.BindTexture(gl::TEXTURE_2D, block_atlas_texture.gl_id);
            vertex_obj.draw_triangles(&uniforms);
            gl.BindTexture(gl::TEXTURE_2D, 0);
        }

        context.swap_buffers();

        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}
