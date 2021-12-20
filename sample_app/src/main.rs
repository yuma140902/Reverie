use re::window::Window;
use reverie_engine as re;

fn main() {
    let config = re::window::WindowConfigBuilder::new().build();
    let mut window = re::window::create_window_depending_on_platform(&config);
    window.show();
    window.main_loop();
}
