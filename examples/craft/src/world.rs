use nalgebra::Point3;
use re::gl::Gl;
use re::CuboidTextures;
use re::Vao;
use re::VaoBuffer;
use re::VaoBuilder3DGeometry;
use re::VaoConfig;
use reverie_engine as re;

use crate::TextureUV;

type Vector3 = nalgebra::Vector3<f32>;

pub struct World {
    blocks: [bool; 16 * 16 * 16],
}

fn pos_to_index(x: u32, y: u32, z: u32) -> usize {
    (16 * 16 * y + 16 * z + x) as usize
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
                    let index = (16 * 16 * y + 16 * z + x) as usize;
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

fn add_block(
    builder: &mut VaoBuffer,
    x: u32,
    y: u32,
    z: u32,
    textures: &CuboidTextures<'_, TextureUV>,
) {
    let begin = Point3::new(x, y, z).cast::<f32>();
    builder.add_cuboid(&begin, &(begin + BLOCK_SIZE), &textures);
}
