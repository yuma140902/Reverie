use reverie_engine::{Game, scene::Scene};

fn setup_cli() {
    use tracing_subscriber::EnvFilter;
    use tracing_subscriber::fmt::format::FmtSpan;

    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_thread_ids(true)
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}

fn main() -> anyhow::Result<()> {
    setup_cli();

    let game = ExampleGame::default();
    reverie_engine::start_engine(game)
}

#[derive(Debug, Default)]
pub struct ExampleGame {
    scene: Scene,
}

impl Game for ExampleGame {
    fn init<'window>(&mut self) {
        tracing::info!("ExampleGame initialized");
    }

    fn get_scene_for_rendering(&mut self) -> &Scene {
        &self.scene
    }

    fn get_scene_mut_for_rendering(&mut self) -> &mut Scene {
        &mut self.scene
    }

    fn update<'a>(&mut self, _frame: &'a reverie_engine::scene::frame::Frame<'a>) {
        // pass
    }
}
