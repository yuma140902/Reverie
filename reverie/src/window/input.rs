use std::collections::HashSet;

#[derive(Debug)]
pub(crate) struct Input {
    pressed_keys: HashSet<winit::event::VirtualKeyCode>,
    unhandled_keydowns: HashSet<winit::event::VirtualKeyCode>,
    unhandled_keyups: HashSet<winit::event::VirtualKeyCode>,
}

impl Input {
    pub(super) fn new() -> Self {
        let pressed_keys = HashSet::new();
        let unhandled_keydowns = HashSet::new();
        let unhandled_keyups = HashSet::new();
        Self {
            pressed_keys,
            unhandled_keydowns,
            unhandled_keyups,
        }
    }

    pub(crate) fn set_key_pressed(&mut self, keycode: &winit::event::VirtualKeyCode) {
        // pressedはキーが押されている間ずっと発生する
        if !self.get_key_pressed(keycode) {
            self.unhandled_keydowns.insert(*keycode);
        }
        self.pressed_keys.insert(*keycode);
    }

    pub(crate) fn set_key_released(&mut self, keycode: &winit::event::VirtualKeyCode) {
        self.unhandled_keyups.insert(*keycode);
        self.pressed_keys.remove(keycode);
    }

    pub(crate) fn get_key_pressed(&self, keycode: &winit::event::VirtualKeyCode) -> bool {
        self.pressed_keys.contains(keycode)
    }

    pub(crate) fn get_keydown(&mut self, keycode: &winit::event::VirtualKeyCode) -> bool {
        let key_is_down = self.unhandled_keydowns.contains(keycode);
        self.unhandled_keydowns.remove(keycode);
        if !(!key_is_down && !self.get_key_pressed(keycode)) {
            //    println!(
            //       "get_keydown key_is_down: {}, pressed: {}",
            //      key_is_down,
            //     self.get_key_pressed(keycode)
            //);
        }
        if key_is_down && self.get_key_pressed(keycode) {
            // TODO: repeat handle time
            true
        } else {
            false
        }
    }

    pub(crate) fn get_keyup(&mut self, keycode: &winit::event::VirtualKeyCode) -> bool {
        let key_is_up = self.unhandled_keyups.contains(keycode);
        self.unhandled_keyups.remove(keycode);
        key_is_up && !self.get_key_pressed(keycode)
    }
}
