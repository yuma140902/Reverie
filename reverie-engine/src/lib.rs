use game::Game;
use winit_app::App;

pub mod game;
pub mod scene;
pub mod texture;
pub mod wgpu_layer;
mod winit_app;

pub fn start_engine<G: Game>(game: G) -> anyhow::Result<()> {
    use anyhow::Context;

    let event_loop = winit::event_loop::EventLoop::new().context("failed: create event loop")?;
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    event_loop
        .run_app(&mut App::new(game))
        .context("failed: run app")?;
    Ok(())
}
