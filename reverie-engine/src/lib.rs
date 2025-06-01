#![deny(rust_2018_idioms)]
#![deny(clippy::all)]
#![deny(clippy::nursery)]

pub mod camera;
mod game;
pub mod model;
pub mod render;
pub mod scene;
pub mod texture;
mod winit_app;

pub use game::Game;
pub use game::start_engine;
