use nalgebra::{Matrix4, Point3, Vector3};
use reverie_engine::util::math::{Deg, Rad};

use crate::config;

#[derive(Debug)]
pub struct Camera {
    pub yaw: Rad<f32>,
    pub pitch: Rad<f32>,
}

impl Camera {
    pub fn new() -> Self {
        let config = config::get();
        Self {
            yaw: Deg(config.player_init_yaw_deg).to_rad(),
            pitch: Deg(config.player_init_pitch_deg).to_rad(),
        }
    }

    pub fn view_matrix(&self, pos: &Point3<f32>) -> Matrix4<f32> {
        let (front, _right, up) = calc_front_right_up(self.yaw, self.pitch);
        Matrix4::<f32>::look_at_rh(pos, &(pos + front), &up)
    }

    pub fn projection_matrix(&self, width: u32, height: u32) -> Matrix4<f32> {
        Matrix4::new_perspective(
            width as f32 / height as f32,
            Deg(config::get().fov).to_rad().into(),
            0.1,
            100.0,
        )
    }
}

pub(crate) fn calc_front_right_up(
    yaw: Rad<f32>,
    pitch: Rad<f32>,
) -> (Vector3<f32>, Vector3<f32>, Vector3<f32>) {
    let front = Vector3::new(pitch.cos() * yaw.sin(), pitch.sin(), pitch.cos() * yaw.cos()).normalize();

    let right_rad = yaw - Deg(90.0f32).to_rad();
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
