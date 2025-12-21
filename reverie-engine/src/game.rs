//! Game トレイト
use crate::{scene::Scene, winit_app::App};

/// ゲームが実装すべきトレイト
pub trait Game {
    /// シーンを生成する
    ///
    /// ゲームが開始されたときに呼ばれる。
    fn generate_scene(&mut self) -> anyhow::Result<Scene>;
}

pub fn start_engine<G: Game>(game: G) -> anyhow::Result<()> {
    use anyhow::Context;

    let event_loop = winit::event_loop::EventLoop::new().context("failed: create event loop")?;
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    event_loop
        .run_app(&mut App::new(game))
        .context("failed: run app")?;
    Ok(())
}
