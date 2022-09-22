use nalgebra::Point3;
use parry3d::bounding_volume::AABB;
use re::gl::Gl;
use re::CuboidTextures;
use re::Vao;
use re::VaoBuilder3DGeometry;
use re::VaoConfig;
use re::VertexWithNormUv;
use reverie_engine as re;

use crate::TextureUV;

type VaoBuffer = re::VaoBuffer<VertexWithNormUv>;
type Vector3 = nalgebra::Vector3<f32>;

pub struct World {
    blocks: [bool; 16 * 16 * 16],
}

fn pos_to_index(x: u32, y: u32, z: u32) -> usize {
    (16 * 16 * y + 16 * z + x) as usize
}

fn get_block_aabb(x: u32, y: u32, z: u32) -> AABB {
    let min = Point3::new(x as f32, y as f32, z as f32) * 0.5;
    let max = min + BLOCK_SIZE * 0.5;
    AABB::new(min, max)
}

impl World {
    pub fn new() -> World {
        World {
            blocks: [false; 16 * 16 * 16],
        }
    }

    pub fn set_block(&mut self, x: u32, y: u32, z: u32) {
        self.blocks[pos_to_index(x, y, z)] = true;
    }

    pub fn remove_block(&mut self, x: u32, y: u32, z: u32) {
        self.blocks[pos_to_index(x, y, z)] = false;
    }

    pub fn generate_collision_aabbs(&self) -> Vec<AABB> {
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

    pub fn generate_selection_aabbs(&self) -> Vec<(u32, u32, u32, AABB)> {
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

    pub fn generate_vertex_obj<'a>(
        &self,
        gl: &Gl,
        textures: &CuboidTextures<'a, TextureUV>,
        config: &'a VaoConfig,
    ) -> Vao<'a> {
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

        buffer_builder.build(gl, config)
    }
}

const BLOCK_SIZE: Vector3 = Vector3::new(1.0, 1.0, 1.0);

fn add_block(builder: &mut VaoBuffer, x: u32, y: u32, z: u32, textures: &CuboidTextures<'_, TextureUV>) {
    let begin = Point3::new(x, y, z).cast::<f32>();
    builder.add_cuboid(&begin, &(begin + BLOCK_SIZE), textures);
}
