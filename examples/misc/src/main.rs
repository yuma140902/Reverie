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

    let game = LineDefense::default();
    reverie_engine::start_engine(game)
}

#[derive(Debug, Default)]
pub struct LineDefense {}

impl Game for LineDefense {
    fn generate_scene(
        &mut self,
        _registry: &mut reverie_engine::texture::TextureRegistry,
    ) -> anyhow::Result<Scene> {
        Ok(Scene::default())
    }
}
