use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct GameConfig {
    fov: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self { fov: 60_f32 }
    }
}
