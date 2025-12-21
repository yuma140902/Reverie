#![allow(dead_code)]
use anyhow::Context;
use nalgebra::{Matrix4, Point3};
use tracing_unwrap::ResultExt;

use crate::{
    model::Vertex,
    render::{RenderingResource, buffer::VertexIndexBuffer, sprite},
    scene::{Scene, TransformComponent},
    texture::TextureId,
};

#[derive(Debug)]
/// エンティティの見た目を表すコンポーネント
pub struct SpriteComponent {
    texture: TextureId,
    buffer: Option<VertexIndexBuffer<Vertex>>,
}

impl SpriteComponent {
    pub const fn new(texture: TextureId) -> Self {
        Self {
            texture,
            buffer: None,
        }
    }

    pub(crate) fn setup(&mut self, resource: &RenderingResource<'_>) {
        let buffer = VertexIndexBuffer::new(&resource.device, 4, 6, None).unwrap_or_log();
        self.buffer = Some(buffer);
    }

    pub(crate) fn render(
        &mut self,
        scene: &Scene,
        rp: &mut wgpu::RenderPass<'_>,
        resource: &RenderingResource<'_>,
        transform: &TransformComponent,
    ) {
        if let Some(buffer) = &mut self.buffer {
            // バッファのアップデート
            {
                let mut update = buffer.start_update(&resource.queue);
                let (min_u, min_v, max_u, max_v) =
                    scene.textures.get_uv(self.texture).unwrap_or_log();
                let affine = transform.to_affine3();
                const POINTS: Matrix4<f32> = Matrix4::new(
                    -0.5, 0.5, -0.5, 0.5, //
                    0.5, 0.5, -0.5, -0.5, //
                    0.0, 0.0, 0.0, 0.0, //
                    1.0, 1.0, 1.0, 1.0, //
                );
                const NORMALS: Matrix4<f32> = Matrix4::new(
                    0.0, 0.0, 0.0, 0.0, //
                    0.0, 0.0, 0.0, 0.0, //
                    1.0, 1.0, 1.0, 1.0, //
                    0.0, 0.0, 0.0, 0.0, //
                );
                let points = affine.matrix() * POINTS;
                let tl = Point3::from_homogeneous(points.column(0).into()).unwrap();
                let tr = Point3::from_homogeneous(points.column(1).into()).unwrap();
                let bl = Point3::from_homogeneous(points.column(2).into()).unwrap();
                let br = Point3::from_homogeneous(points.column(3).into()).unwrap();

                let normals = affine.matrix() * NORMALS;
                let tln = Point3::from_homogeneous(normals.column(0).into()).unwrap();
                let trn = Point3::from_homogeneous(normals.column(1).into()).unwrap();
                let bln = Point3::from_homogeneous(normals.column(2).into()).unwrap();
                let brn = Point3::from_homogeneous(normals.column(3).into()).unwrap();

                let range = {
                    let v = update.vertex_mut();
                    v.clear();
                    v.push(Vertex {
                        position: tl.into(),
                        uv: [min_u, min_v],
                        normal: tln.into(),
                    });
                    v.push(Vertex {
                        position: tr.into(),
                        uv: [max_u, min_v],
                        normal: trn.into(),
                    });
                    v.push(Vertex {
                        position: bl.into(),
                        uv: [min_u, max_v],
                        normal: bln.into(),
                    });
                    v.push(Vertex {
                        position: br.into(),
                        uv: [max_u, max_v],
                        normal: brn.into(),
                    });
                    0..v.len()
                };
                update.set_vertex_update(range);

                let range = {
                    let i = update.index_mut();
                    i.clear();
                    i.extend_from_slice(&[0, 3, 1, 0, 2, 3]);
                    0..i.len()
                };
                update.set_index_update(range.clone());
                update.set_render_range(range.start as u32..range.end as u32);
            }

            let bind_group = scene
                .textures
                .get_bind_group(self.texture)
                .context("texture not found for index")
                .unwrap_or_log();
            rp.set_bind_group(sprite::GROUP_TEXTURE, bind_group, &[]);
            rp.set_index_buffer(buffer.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            rp.set_vertex_buffer(0, buffer.vertex_buffer.slice(..));
            rp.draw_indexed(buffer.index_buffer_range.clone(), 0, 0..1);
        } else {
            tracing::warn!("buffer is not initialized");
        }
    }
}
