use self::cursor::{CursorPosition, DesktopOrigin};
use std::collections::HashSet;

pub mod cursor;

#[cfg(feature = "winit")]
pub(crate) fn mouse_button_index(mouse_button: &winit::event::MouseButton) -> usize {
    match mouse_button {
        winit::event::MouseButton::Left => 0,
        winit::event::MouseButton::Right => 1,
        winit::event::MouseButton::Middle => 2,
        winit::event::MouseButton::Other(id) => *id as usize * 2,
    }
}

#[cfg(feature = "winit")]
pub(crate) fn mouse_button_index_3(mouse_button: &winit::event::MouseButton) -> Option<usize> {
    match mouse_button {
        winit::event::MouseButton::Left
        | winit::event::MouseButton::Right
        | winit::event::MouseButton::Middle => Some(mouse_button_index(mouse_button)),
        winit::event::MouseButton::Other(_) => None,
    }
}

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
    cursor_dx: i32,
    cursor_dy: i32,
    mousedown_unhandled: [bool; 3],
    mouseup_unhandled: [bool; 3],
    mouse_pressed: [bool; 3],
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
            cursor_dx: 0,
            cursor_dy: 0,
            mousedown_unhandled: [false; 3],
            mouseup_unhandled: [false; 3],
            mouse_pressed: [false; 3],
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
    pub(crate) fn update_cursor_position(
        &mut self,
        pos: CursorPosition<DesktopOrigin>,
        window_x: i32,
        window_y: i32,
        window_width: i32,
        window_height: i32,
    ) {
        self.cursor_x = pos.x;
        self.cursor_y = pos.y;
        let pos_rel = pos.to_window_center(window_x, window_y, window_width, window_height);
        self.cursor_dx = pos_rel.x;
        self.cursor_dy = pos_rel.y;
    }

    #[cfg(feature = "winit")]
    pub(crate) fn update_mouse_pressed(&mut self, button: &winit::event::MouseButton) {
        if let Some(index) = mouse_button_index_3(button) {
            self.mousedown_unhandled[index] = true;
            self.mouse_pressed[index] = true;
        }
    }

    #[cfg(feature = "winit")]
    pub(crate) fn update_mouse_released(&mut self, button: &winit::event::MouseButton) {
        if let Some(index) = mouse_button_index_3(button) {
            self.mouseup_unhandled[index] = true;
            self.mouse_pressed[index] = false;
        }
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
    pub(crate) fn get_cursor_pos(&self) -> CursorPosition<DesktopOrigin> {
        CursorPosition::new(self.cursor_x, self.cursor_y)
    }

    #[cfg(feature = "winit")]
    pub(crate) fn get_cursor_delta(&self) -> (i32, i32) {
        (self.cursor_dx, self.cursor_dy)
    }

    #[cfg(feature = "winit")]
    pub(crate) fn get_mouse_pressed(&self, index: usize) -> bool {
        self.mouse_pressed[index]
    }

    #[cfg(feature = "winit")]
    pub(crate) fn get_mouse_down(&mut self, index: usize) -> bool {
        let mouse_down = self.mousedown_unhandled[index];
        self.mousedown_unhandled[index] = false;
        mouse_down && self.mouse_pressed[index]
    }

    #[cfg(feature = "winit")]
    pub(crate) fn get_mouse_up(&mut self, index: usize) -> bool {
        let mouse_up = self.mouseup_unhandled[index];
        self.mouseup_unhandled[index] = false;
        mouse_up && !self.mouse_pressed[index]
    }
}
