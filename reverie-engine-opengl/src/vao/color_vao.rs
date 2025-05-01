use reverie_util::math::nalgebra::Point3;

use crate::vao::{VaoBuffer, VertexWithColor};

pub trait VaoBuilder3DGeometryOutline {
    fn add_cuboid_outline(
        &mut self,
        begin: &Point3<f32>,
        end: &Point3<f32>,
        r: f32,
        g: f32,
        b: f32,
    );
}

impl VaoBuilder3DGeometryOutline for VaoBuffer<VertexWithColor> {
    fn add_cuboid_outline(
        &mut self,
        begin: &Point3<f32>,
        end: &Point3<f32>,
        r: f32,
        g: f32,
        b: f32,
    ) {
        // 上面
        let p0 = begin;
        let p1 = &Point3::new(begin.x, begin.y, end.z);
        let p2 = &Point3::new(begin.x, end.y, end.z);
        let p3 = &Point3::new(begin.x, end.y, begin.z);

        // 下面
        let p4 = &Point3::new(end.x, begin.y, end.z);
        let p5 = end;
        let p6 = &Point3::new(end.x, end.y, begin.z);
        let p7 = &Point3::new(end.x, begin.y, begin.z);

        let mut v: Vec<f32> = vec![
            // 上面
            p0.x, p0.y, p0.z, r, g, b, //
            p1.x, p1.y, p1.z, r, g, b, //
            p1.x, p1.y, p1.z, r, g, b, //
            p2.x, p2.y, p2.z, r, g, b, //
            p2.x, p2.y, p2.z, r, g, b, //
            p3.x, p3.y, p3.z, r, g, b, //
            p3.x, p3.y, p3.z, r, g, b, //
            p0.x, p0.y, p0.z, r, g, b, //
            // 下面
            p4.x, p4.y, p4.z, r, g, b, //
            p5.x, p5.y, p5.z, r, g, b, //
            p5.x, p5.y, p5.z, r, g, b, //
            p6.x, p6.y, p6.z, r, g, b, //
            p6.x, p6.y, p6.z, r, g, b, //
            p7.x, p7.y, p7.z, r, g, b, //
            p7.x, p7.y, p7.z, r, g, b, //
            p4.x, p4.y, p4.z, r, g, b, //
            // 横の線
            p0.x, p0.y, p0.z, r, g, b, //
            p4.x, p4.y, p4.z, r, g, b, //
            p1.x, p1.y, p1.z, r, g, b, //
            p5.x, p5.y, p5.z, r, g, b, //
            p2.x, p2.y, p2.z, r, g, b, //
            p6.x, p6.y, p6.z, r, g, b, //
            p3.x, p3.y, p3.z, r, g, b, //
            p7.x, p7.y, p7.z, r, g, b, //
        ];

        self.append(&mut v);
    }
}
