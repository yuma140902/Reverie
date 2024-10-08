use nalgebra::Point3;
use parry3d::bounding_volume::Aabb;
use re::gl::Gl;
use re::vao::CuboidTextures;
use re::vao::Vao;
use re::vao::VaoBuilder3DGeometry;
use re::vao::VaoConfig;
use re::vao::VertexWithNormUv;
use reverie_engine_opengl as re;

use crate::config;
use crate::TextureUV;

type VaoBuffer = re::vao::VaoBuffer<VertexWithNormUv>;
type Vector3 = nalgebra::Vector3<f32>;

pub struct World {
    blocks: [bool; 16 * 16 * 16],
}

const fn pos_to_index(x: u32, y: u32, z: u32) -> usize {
    (16 * 16 * y + 16 * z + x) as usize
}

pub fn is_valid_pos(x: u32, y: u32, z: u32) -> bool {
    (0..16).contains(&x) && (0..16).contains(&y) && (0..16).contains(&z)
}

fn get_block_aabb(x: u32, y: u32, z: u32) -> Aabb {
    let min = Point3::new(x as f32, y as f32, z as f32) * 0.5;
    let max = min + BLOCK_SIZE * 0.5;
    Aabb::new(min, max)
}

impl World {
    pub const fn new() -> Self {
        Self {
            blocks: [false; 16 * 16 * 16],
        }
    }

    pub fn set_block(&mut self, x: u32, y: u32, z: u32) {
        self.blocks[pos_to_index(x, y, z)] = true;
    }

    pub fn remove_block(&mut self, x: u32, y: u32, z: u32) {
        self.blocks[pos_to_index(x, y, z)] = false;
    }

    pub fn generate_collision_aabbs(&self) -> Vec<Aabb> {
        let mut v = Vec::new();
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let index = pos_to_index(x, y, z);
                    if self.blocks[index] {
                        v.push(get_block_aabb(x, y, z));
                    }
                }
            }
        }
        v
    }

    pub fn generate_selection_aabbs(&self) -> Vec<(u32, u32, u32, Aabb)> {
        let mut v = Vec::new();
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let index = pos_to_index(x, y, z);
                    if self.blocks[index] {
                        v.push((x, y, z, get_block_aabb(x, y, z)));
                    }
                }
            }
        }
        v
    }

    #[allow(clippy::too_many_arguments)]
    pub fn generate_vertex_obj<'a>(
        &self,
        gl: &Gl,
        textures: &CuboidTextures<'a, TextureUV>,
        manual1_texture: &TextureUV,
        manual2_texture: &TextureUV,
        selected_xyz: Option<(u32, u32, u32)>,
        selected_texture: &CuboidTextures<'a, TextureUV>,
        config: &'a VaoConfig,
    ) -> Vao<'a> {
        let game_config = config::get();
        let mut buffer_builder = VaoBuffer::new();

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let index = pos_to_index(x, y, z);
                    let block = self.blocks[index];
                    if !block {
                        continue;
                    }
                    add_block(&mut buffer_builder, x, y, z, textures);
                }
            }
        }

        if game_config.show_manual {
            buffer_builder.add_face(
                &Point3::new(1.1, 3.0, 6.0),
                &Point3::new(1.1, 1.0, 6.0),
                &Point3::new(1.1, 1.0, 4.0),
                &Point3::new(1.1, 3.0, 4.0),
                manual1_texture,
            );
            buffer_builder.add_face(
                &Point3::new(1.1, 3.0, 4.0),
                &Point3::new(1.1, 1.0, 4.0),
                &Point3::new(1.1, 1.0, 2.0),
                &Point3::new(1.1, 3.0, 2.0),
                manual2_texture,
            );
        }

        if let Some((x, y, z)) = selected_xyz {
            add_block_highlight(
                &mut buffer_builder,
                x,
                y,
                z,
                selected_texture,
                game_config.highlight_e,
            );
        }

        buffer_builder.build(gl, config)
    }
}

const BLOCK_SIZE: Vector3 = Vector3::new(1.0, 1.0, 1.0);

fn add_block(
    builder: &mut VaoBuffer,
    x: u32,
    y: u32,
    z: u32,
    textures: &CuboidTextures<'_, TextureUV>,
) {
    let begin = Point3::new(x, y, z).cast::<f32>();
    builder.add_cuboid(&begin, &(begin + BLOCK_SIZE), textures);
}

fn add_block_highlight(
    builder: &mut VaoBuffer,
    x: u32,
    y: u32,
    z: u32,
    textures: &CuboidTextures<'_, TextureUV>,
    e: f32,
) {
    let x = x as f32;
    let y = y as f32;
    let z = z as f32;
    let begin = Point3::new(x - e, y - e, z - e);
    let end = Point3::new(x + 1.0 + e, y + 1.0 + e, z + 1.0 + e);
    builder.add_cuboid(&begin, &end, textures);
}
