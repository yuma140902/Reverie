use reverie_engine as re;

fn main() {
    let config = re::window::WindowConfigBuilder::new().build();
    let window = re::window::Window::new(&config);
    window.show();
    window.main_loop();
}
