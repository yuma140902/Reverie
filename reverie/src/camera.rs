use reverie_util::math::{
    self,
    nalgebra::{Matrix4, Point3, Vector3},
    Deg, Rad,
};

use crate::{
    gl::Gl,
    shader::Program,
    texture::ImageLoadInfo,
    vao::{Phong3DRenderer, Phong3DRenderingInfo, PhongRenderingInfo, Renderer, Vao},
};

#[derive(Debug)]
pub struct Camera {
    pos: Point3<f32>,
    yaw: Rad<f32>,
    pitch: Rad<f32>,
    fov: Deg<f32>,
    gl: Gl,
    renderer: Phong3DRenderer,
}

impl Camera {
    pub fn new(gl: Gl, pos: Point3<f32>, yaw: Rad<f32>, pitch: Rad<f32>, fov: Deg<f32>) -> Self {
        let shader = Program::default_uv(gl.clone()).unwrap();
        let renderer = Phong3DRenderer::new(shader);
        Self {
            pos,
            yaw,
            pitch,
            fov,
            gl,
            renderer,
        }
    }

    pub fn render(
        &self,
        vao: &Vao,
        model_matrix: &Matrix4<f32>,
        width: u32,
        height: u32,
        phong_info: &PhongRenderingInfo,
        block_atlas_texture: &ImageLoadInfo,
    ) {
        let view_matrix = self.view_matrix();
        let projection_matrix = self.projection_matrix(width, height);

        let info = &Phong3DRenderingInfo {
            phong: phong_info,
            model_matrix,
            view_matrix: &view_matrix,
            projection_matrix: &projection_matrix,
            camera_pos: &self.pos,
            texture: block_atlas_texture,
        };
        self.renderer.render(self.gl.clone(), vao, info);
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        let (front, _right, up) = math::calc_front_right_up(self.yaw, self.pitch);
        Matrix4::<f32>::look_at_rh(&self.pos, &(self.pos + front), &up)
    }

    pub fn projection_matrix(&self, width: u32, height: u32) -> Matrix4<f32> {
        Matrix4::new_perspective(width as f32 / height as f32, self.fov.to_rad().into(), 0.1, 100.0)
    }

    pub fn set_pos(&mut self, pos: Point3<f32>) {
        self.pos = pos;
    }

    pub fn move_pos(&mut self, d: Vector3<f32>) {
        self.pos += d;
    }

    pub fn set_yaw(&mut self, yaw: Rad<f32>) {
        self.yaw = yaw;
    }

    pub fn add_yaw(&mut self, yaw_d: Rad<f32>) {
        self.yaw += yaw_d;
    }

    pub fn set_pitch(&mut self, pitch: Rad<f32>) {
        self.pitch = pitch;
    }

    pub fn add_pitch(&mut self, pitch_d: Rad<f32>) {
        self.pitch += pitch_d;
    }

    pub fn set_fov(&mut self, fov: Deg<f32>) {
        self.fov = fov;
    }

    pub fn pos(&self) -> Point3<f32> {
        self.pos
    }

    pub fn yaw(&self) -> Rad<f32> {
        self.yaw
    }

    pub fn pitch(&self) -> Rad<f32> {
        self.pitch
    }

    pub fn fov(&self) -> Deg<f32> {
        self.fov
    }
}
