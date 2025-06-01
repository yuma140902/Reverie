use nalgebra::{Matrix4, Point3};
use tracing_unwrap::ResultExt;

use crate::{
    model::colored::ColoredVertex,
    render::{RenderingResource, buffer::VertexIndexBuffer},
    scene::TransformComponent,
};

pub struct ColoredComponent {
    /// RGBA
    color0: [f32; 4],
    color1: [f32; 4],
    color2: [f32; 4],
    color3: [f32; 4],
    buffer: Option<VertexIndexBuffer<ColoredVertex>>,
}

impl ColoredComponent {
    pub const fn new(
        color0: [f32; 4],
        color1: [f32; 4],
        color2: [f32; 4],
        color3: [f32; 4],
    ) -> Self {
        Self {
            color0,
            color1,
            color2,
            color3,
            buffer: None,
        }
    }

    pub(crate) fn setup(&mut self, resource: &RenderingResource<'_>) {
        let buffer = VertexIndexBuffer::new(&resource.device, 4, 6, None).unwrap_or_log();
        self.buffer = Some(buffer);
    }

    pub(crate) fn render(
        &mut self,
        rp: &mut wgpu::RenderPass<'_>,
        resource: &RenderingResource<'_>,
        transform: &TransformComponent,
    ) {
        if let Some(buffer) = &mut self.buffer {
            // バッファのアップデート
            {
                let mut update = buffer.start_update(&resource.queue);
                let affine = transform.to_affine3();
                const POINTS: Matrix4<f32> = Matrix4::new(
                    -0.5, 0.5, -0.5, 0.5, //
                    0.5, 0.5, -0.5, -0.5, //
                    0.0, 0.0, 0.0, 0.0, //
                    1.0, 1.0, 1.0, 1.0, //
                );
                let points = affine.matrix() * POINTS;
                let top_left = Point3::from_homogeneous(points.column(0).into()).unwrap();
                let top_right = Point3::from_homogeneous(points.column(1).into()).unwrap();
                let bottom_left = Point3::from_homogeneous(points.column(2).into()).unwrap();
                let bottom_right = Point3::from_homogeneous(points.column(3).into()).unwrap();

                let range = {
                    let v = update.vertex_mut();
                    v.clear();
                    v.push(ColoredVertex {
                        position: top_left.into(),
                        color: self.color0,
                    });
                    v.push(ColoredVertex {
                        position: top_right.into(),
                        color: self.color1,
                    });
                    v.push(ColoredVertex {
                        position: bottom_left.into(),
                        color: self.color2,
                    });
                    v.push(ColoredVertex {
                        position: bottom_right.into(),
                        color: self.color3,
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

            rp.set_index_buffer(buffer.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            rp.set_vertex_buffer(0, buffer.vertex_buffer.slice(..));
            rp.draw_indexed(buffer.index_buffer_range.clone(), 0, 0..1);
        } else {
            tracing::warn!("buffer is not initialized");
        }
    }
}
