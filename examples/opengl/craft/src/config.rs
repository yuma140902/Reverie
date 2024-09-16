use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

use crate::{Point3, Vector3};

pub const CONFIG_FILE: &str = "./craft-config.json";
pub static CONFIG: OnceCell<GameConfig> = OnceCell::new();

pub fn get() -> &'static GameConfig {
    CONFIG.get().unwrap()
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct GameConfig {
    pub fov: f32,
    pub velocity_decay_rate: f32,
    pub max_velocity: f32,
    pub move_speed: f32,
    pub rotation_speed: f32,
    pub player_init_pos: Point3,
    pub player_init_yaw_deg: f32,
    pub player_init_pitch_deg: f32,
    pub player_bounding_vec: Vector3,
    pub eye: Vector3,
    pub shader_material_specular: Vector3,
    pub shader_material_shininess: f32,
    pub shader_light_direction: Vector3,
    pub shader_ambient: Vector3,
    pub shader_diffuse: Vector3,
    pub shader_specular: Vector3,
    pub shader_alpha: f32,
    pub ray: f32,
    pub highlight_e: f32,
    pub show_manual: bool,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            fov: 60_f32,
            velocity_decay_rate: 0.9,
            max_velocity: 0.5,
            move_speed: 0.01,
            rotation_speed: 0.01,
            player_init_pos: Point3::new(4.0, 3.6, 4.0),
            player_init_yaw_deg: 225_f32,
            player_init_pitch_deg: -30_f32,
            player_bounding_vec: Vector3::new(0.15, 0.45, 0.15),
            eye: Vector3::new(0.0, 0.3, 0.0),
            shader_material_specular: Vector3::new(0.1, 0.1, 0.1),
            shader_material_shininess: 0.4,
            shader_light_direction: Vector3::new(1.0, 1.0, 0.0),
            shader_ambient: Vector3::new(0.5, 0.5, 0.5),
            shader_diffuse: Vector3::new(0.6, 0.6, 0.6),
            shader_specular: Vector3::new(0.2, 0.2, 0.2),
            shader_alpha: 1.0,
            ray: 3.0,
            highlight_e: 0.01,
            show_manual: true,
        }
    }
}
