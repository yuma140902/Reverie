use nalgebra::{Matrix4, Point3, Vector3};

pub struct Camera {
    pub pos: Point3<f32>,
    pub pitch_rad: f32,
    pub yaw_rad: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            pos: Point3::new(4.0, 3.6, 4.0),
            pitch_rad: deg_to_rad(225.0),
            yaw_rad: deg_to_rad(-30.0),
        }
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        let (front, _right, up) = calc_front_right_up(self.pitch_rad, self.yaw_rad);
        Matrix4::<f32>::look_at_rh(&self.pos, &(self.pos + front), &up)
    }

    pub fn projection_matrix(&self, width: u32, height: u32) -> Matrix4<f32> {
        Matrix4::new_perspective(
            width as f32 / height as f32,
            deg_to_rad(45.0f32),
            0.1,
            100.0,
        )
    }
}

fn calc_front_right_up(pitch_rad: f32, yaw_rad: f32) -> (Vector3<f32>, Vector3<f32>, Vector3<f32>) {
    let front = Vector3::new(
        yaw_rad.cos() * pitch_rad.sin(),
        yaw_rad.sin(),
        yaw_rad.cos() * pitch_rad.cos(),
    )
    .normalize();

    let right_rad = pitch_rad - deg_to_rad(90.0f32);
    // 右方向のベクトル
    let right = Vector3::new(
        right_rad.sin(),
        0.0f32, /* ロールは0なので常に床と水平 */
        right_rad.cos(),
    )
    .normalize();

    // 上方向のベクトル
    let up = right.cross(&front);

    (front, right, up)
}

fn deg_to_rad(deg: f32) -> f32 {
    deg * std::f32::consts::PI / 180_f32
}
