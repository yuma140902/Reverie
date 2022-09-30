#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use c_str_macro::c_str;

use player::Player;
use world::World;

use re::gl;
use re::math::Deg;
use re::math::Rad;
use re::shader::Program;
use re::shader::Shader;
use re::texture::ImageManager;
use re::texture::TextureAtlasPos;
use re::types::Const;
use re::CuboidTextures;
use re::Phong3DRenderer;
use re::Phong3DRenderingInfo;
use re::PhongRenderingInfo;
use re::Renderer;
use re::ReverieEngine;
use re::VaoConfigBuilder;
use reverie_engine as re;

use crate::config::CONFIG;
use crate::config::CONFIG_FILE;

mod camera;
mod collision;
mod config;
mod player;
mod raycast;
pub mod util;
mod world;

#[allow(dead_code)]
type Point3 = nalgebra::Point3<f32>;
type Vector3 = nalgebra::Vector3<f32>;
type Matrix4 = nalgebra::Matrix4<f32>;

pub type TextureUV = re::texture::TextureUV<Const<64>, Const<64>, Const<256>, Const<256>>;

fn main() {
    let width = 900;
    let height = 480;

    let config = CONFIG.get_or_init(|| {
        let string = std::fs::read_to_string(CONFIG_FILE).unwrap_or("{}".to_string());
        serde_json::from_str(&string).unwrap_or_default()
    });
    println!("{:?}", config);

    let engine = ReverieEngine::new();
    let mut window = engine
        .window_builder()
        .title("Craft")
        .size(width, height)
        .maximize()
        .build();
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
    let manual1_texture = TextureUV::of_atlas(&TextureAtlasPos::new(1, 0));
    let manual2_texture = TextureUV::of_atlas(&TextureAtlasPos::new(1, 1));
    let highlight_texture = TextureUV::of_atlas(&TextureAtlasPos::new(0, 3));
    let cuboid_texture = CuboidTextures {
        top: &top_texture,
        bottom: &bottom_texture,
        north: &side_texture,
        south: &side_texture,
        west: &side_texture,
        east: &side_texture,
    };
    let highlight_cuboid_texture = CuboidTextures {
        top: &highlight_texture,
        bottom: &highlight_texture,
        south: &highlight_texture,
        north: &highlight_texture,
        west: &highlight_texture,
        east: &highlight_texture,
    };

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

    let vao_config = VaoConfigBuilder::new()
        .depth_test(true)
        .blend(true)
        .wireframe(false)
        .culling(true)
        .build();

    let phong_info = PhongRenderingInfo {
        material_specular: &config.shader_material_specular,
        material_shininess: config.shader_material_shininess,
        light_direction: &config.shader_light_direction,
        ambient: &config.shader_ambient,
        diffuse: &config.shader_diffuse,
        specular: &config.shader_specular,
        alpha: config.shader_alpha,
    };

    let renderer = Phong3DRenderer::new(shader);
    let mut vertex_obj = world.generate_vertex_obj(
        &gl,
        &cuboid_texture,
        &manual1_texture,
        &manual2_texture,
        None,
        &highlight_cuboid_texture,
        &vao_config,
    );

    let mut player = Player::new();

    while !window.process_event(&gl) {
        unsafe {
            gl.ClearColor(1.0, 1.0, 1.0, 1.0);
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        if window.keypressed(&winit::event::VirtualKeyCode::Escape) {
            break;
        }

        const UP: Vector3 = Vector3::new(0.0, 1.0, 0.0);
        let (front, right, _up) = camera::calc_front_right_up(player.camera.yaw, player.camera.pitch);
        let front = util::take_xz_normalized(&front);
        if window.keypressed(&winit::event::VirtualKeyCode::W) {
            player.velocity += front * config.move_speed;
        }
        if window.keypressed(&winit::event::VirtualKeyCode::S) {
            player.velocity -= front * config.move_speed;
        }
        if window.keypressed(&winit::event::VirtualKeyCode::D) {
            player.velocity += right * config.move_speed;
        }
        if window.keypressed(&winit::event::VirtualKeyCode::A) {
            player.velocity -= right * config.move_speed;
        }
        if window.keypressed(&winit::event::VirtualKeyCode::Space) {
            player.velocity += UP * config.move_speed;
        }
        if window.keypressed(&winit::event::VirtualKeyCode::LShift) {
            player.velocity -= UP * config.move_speed;
        }

        let (dx, dy) = window.cursor_delta();
        if dy != 0 {
            player.camera.pitch += Rad(-dy as f32 * config.rotation_speed);
            player.camera.pitch = player.camera.pitch.normalized();
            if player.camera.pitch < Deg(-90_f32).to_rad() {
                player.camera.pitch = Deg(-90_f32).to_rad();
            }
            if Deg(90_f32).to_rad() < player.camera.pitch {
                player.camera.pitch = Deg(90_f32).to_rad();
            }
        }
        if dx != 0 {
            player.camera.yaw += Rad(-dx as f32 * config.rotation_speed);
            player.camera.yaw = player.camera.yaw.normalized();
        }

        if window.mouse_down(&winit::event::MouseButton::Left) {
            if let Some((x, y, z)) = raycast::hit_block(&player, &world) {
                world.remove_block(x, y, z);
            }
        }

        if window.mouse_down(&winit::event::MouseButton::Right) {
            if let Some((x, y, z, Some(side))) = raycast::hit_block_and_side(&player, &world) {
                let (x, y, z) = side.offset(x, y, z);
                if world::is_valid_pos(x, y, z) {
                    world.set_block(x, y, z);
                }
            }
        }

        if let Some((x, y, z)) = raycast::hit_block(&player, &world) {
            vertex_obj = world.generate_vertex_obj(
                &gl,
                &cuboid_texture,
                &manual1_texture,
                &manual2_texture,
                Some((x, y, z)),
                &highlight_cuboid_texture,
                &vao_config,
            );
        } else {
            vertex_obj = world.generate_vertex_obj(
                &gl,
                &cuboid_texture,
                &manual1_texture,
                &manual2_texture,
                None,
                &highlight_cuboid_texture,
                &vao_config,
            );
        }

        player.update_pos(&world);

        let model_matrix =
            nalgebra_glm::scale(&Matrix4::identity(), &Vector3::new(0.5_f32, 0.5_f32, 0.5_f32));
        let view_matrix = player.view_matrix();
        let projection_matrix: Matrix4 = player.projection_matrix(width, height);

        let info = &Phong3DRenderingInfo {
            phong: &phong_info,
            model_matrix: &model_matrix,
            view_matrix: &view_matrix,
            projection_matrix: &projection_matrix,
            camera_pos: &player.pos,
            texture: &block_atlas_texture,
        };
        renderer.render(gl.clone(), &vertex_obj, info);

        context.swap_buffers();

        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    println!("exit");
    let config = CONFIG.get().unwrap();
    if let Ok(string) = serde_json::to_string_pretty(config) {
        std::fs::write(CONFIG_FILE, string).unwrap();
    }
}
