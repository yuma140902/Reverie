//! Game トレイト
use crate::{
    scene::{Scene, frame::Frame},
    window::App,
};

/// ゲームが実装すべきトレイト
pub trait Game {
    /// ゲームが初期化されたときに呼ばれる。
    fn init(&mut self);

    /// レンダリング用に現在の [`Scene`] を返す。
    fn get_scene_for_rendering(&mut self) -> &Scene;

    /// レンダリング用に現在の [`Scene`] を返す。
    fn get_scene_mut_for_rendering(&mut self) -> &mut Scene;

    /// フレームごとに呼ばれる。引数の [`Frame`] をもとにゲームの状態を更新する。
    fn update<'a>(&mut self, frame: &'a Frame<'a>);
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
