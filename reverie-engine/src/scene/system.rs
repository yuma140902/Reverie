use std::time::{Duration, Instant};

use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, TouchPhase},
};

use crate::render::RenderingResource;

#[derive(Debug)]
/// フレームごとに更新される情報
pub struct Frame<'a> {
    pub now: Instant,
    pub delta_time: Duration,
    pub key_events: &'a [KeyEvent],
    pub mouse_clicks: &'a [(ElementState, MouseButton, PhysicalPosition<f64>)],
    pub mouse_wheels: &'a [(MouseScrollDelta, TouchPhase, PhysicalPosition<f64>)],
    pub mouse_position: PhysicalPosition<f64>,
}

pub trait System {
    fn setup(&mut self, resource: &RenderingResource<'_>);

    fn update(
        &mut self,
        frame: &Frame<'_>,
        world: &mut hecs::World,
        resource: &RenderingResource<'_>,
    );
}
