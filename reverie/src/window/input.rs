use std::collections::HashSet;

#[derive(Debug)]
pub(crate) struct Input {
    #[cfg(feature = "winit")]
    pressed_keys: HashSet<winit::event::VirtualKeyCode>,
    #[cfg(feature = "winit")]
    unhandled_keydowns: HashSet<winit::event::VirtualKeyCode>,
    #[cfg(feature = "winit")]
    unhandled_keyups: HashSet<winit::event::VirtualKeyCode>,
    cursor_x: i32,
    cursor_y: i32,
    prev_cursor_x: i32,
    prev_cursor_y: i32,
}

impl Input {
    #[cfg(feature = "winit")]
    pub(super) fn new() -> Self {
        let pressed_keys = HashSet::new();
        let unhandled_keydowns = HashSet::new();
        let unhandled_keyups = HashSet::new();
        Self {
            pressed_keys,
            unhandled_keydowns,
            unhandled_keyups,
            cursor_x: 0,
            cursor_y: 0,
            prev_cursor_x: 0,
            prev_cursor_y: 0,
        }
    }

    #[cfg(feature = "winit")]
    pub(crate) fn update_key_pressed(&mut self, keycode: &winit::event::VirtualKeyCode) {
        // pressedはキーが押されている間ずっと発生する
        if !self.get_key_pressed(keycode) {
            self.unhandled_keydowns.insert(*keycode);
        }
        self.pressed_keys.insert(*keycode);
    }

    #[cfg(feature = "winit")]
    pub(crate) fn update_key_released(&mut self, keycode: &winit::event::VirtualKeyCode) {
        self.unhandled_keyups.insert(*keycode);
        self.pressed_keys.remove(keycode);
    }

    #[cfg(feature = "winit")]
    pub(crate) fn update_cursor_position(&mut self, x: i32, y: i32) {
        self.prev_cursor_x = self.cursor_x;
        self.prev_cursor_y = self.cursor_y;
        self.cursor_x = x;
        self.cursor_y = y;
    }

    #[cfg(feature = "winit")]
    pub(crate) fn get_key_pressed(&self, keycode: &winit::event::VirtualKeyCode) -> bool {
        self.pressed_keys.contains(keycode)
    }

    #[cfg(feature = "winit")]
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

    #[cfg(feature = "winit")]
    pub(crate) fn get_keyup(&mut self, keycode: &winit::event::VirtualKeyCode) -> bool {
        let key_is_up = self.unhandled_keyups.contains(keycode);
        self.unhandled_keyups.remove(keycode);
        key_is_up && !self.get_key_pressed(keycode)
    }

    #[cfg(feature = "winit")]
    pub(crate) fn get_cursor_pos_desktop(&self) -> (i32, i32) {
        (self.cursor_x, self.cursor_y)
    }

    #[cfg(feature = "winit")]
    pub(crate) fn get_cursor_pos(
        &self,
        origin: CursorPositionOrigin,
        window: &winit::window::Window,
    ) -> (i32, i32) {
        match origin {
            CursorPositionOrigin::Desktop => self.get_cursor_pos_desktop(),
            CursorPositionOrigin::Window => {
                let winpos = window.inner_position().unwrap();
                (self.cursor_x - winpos.x, self.cursor_y - winpos.y)
            }
            CursorPositionOrigin::WindowCenter => {
                let winpos = window.inner_position().unwrap();
                let winsize = window.inner_size();
                (
                    self.cursor_x - winpos.x - winsize.width as i32 / 2,
                    self.cursor_y - winpos.y - winsize.height as i32 / 2,
                )
            }
        }
    }

    #[cfg(feature = "winit")]
    pub(crate) fn get_cursor_delta(&mut self) -> (i32, i32) {
        let ret = (
            self.cursor_x - self.prev_cursor_x,
            self.cursor_y - self.prev_cursor_y,
        );
        self.update_cursor_position(self.cursor_x, self.cursor_y);
        ret
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CursorPositionOrigin {
    Desktop,
    Window,
    WindowCenter,
}
