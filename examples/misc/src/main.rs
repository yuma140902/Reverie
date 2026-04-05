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

#[derive(Debug)]
pub struct ExampleGame {
    scene: Scene,
    skybox_h: f64,
    skybox_s: f64,
    skybox_v: f64,
}

impl Default for ExampleGame {
    fn default() -> Self {
        let scene = Scene::default();
        Self {
            scene,
            skybox_h: 218.0 / 360.0,
            skybox_s: 54.0 / 100.0,
            skybox_v: 46.0 / 100.0,
        }
    }
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

    fn update<'a>(&mut self, frame: &'a reverie_engine::scene::frame::Frame<'a>) {
        // pass
        self.skybox_h += (frame.delta_time.as_millis() as f64 / 32.0) / 360.0;
        if self.skybox_h > 360.0 {
            self.skybox_h = 0.0;
        }

        self.scene.skybox = hsv_to_rgb(self.skybox_h, self.skybox_s, self.skybox_v);

        fn hsv_to_rgb(h: f64, s: f64, v: f64) -> wgpu::Color {
            let i = (h * 6.0).floor() as u32;
            let f = h * 6.0 - i as f64;
            let p = v * (1.0 - s);
            let q = v * (1.0 - f * s);
            let t = v * (1.0 - (1.0 - f) * s);
            match i % 6 {
                0 => wgpu::Color {
                    r: v,
                    g: t,
                    b: p,
                    a: 1.0,
                },
                1 => wgpu::Color {
                    r: q,
                    g: v,
                    b: p,
                    a: 1.0,
                },
                2 => wgpu::Color {
                    r: p,
                    g: v,
                    b: t,
                    a: 1.0,
                },
                3 => wgpu::Color {
                    r: p,
                    g: q,
                    b: v,
                    a: 1.0,
                },
                4 => wgpu::Color {
                    r: t,
                    g: p,
                    b: v,
                    a: 1.0,
                },
                _ => wgpu::Color {
                    r: v,
                    g: p,
                    b: q,
                    a: 1.0,
                },
            }
        }
    }
}
