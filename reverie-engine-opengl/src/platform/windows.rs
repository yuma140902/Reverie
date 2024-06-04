pub use windows::core::Error;
use windows::Win32::{
    Foundation::POINT,
    UI::WindowsAndMessaging::{GetCursorPos, SetCursorPos},
};

use crate::window::input::cursor::{CursorPosition, DesktopOrigin};

pub fn set_cursor_pos(x: i32, y: i32) -> Result<(), Error> {
    let result = unsafe { SetCursorPos(x, y) }.ok();
    if let Err(ref err) = result {
        if err.code().is_ok() {
            return Ok(());
        }
    }
    result
}

pub fn get_cursor_pos() -> Result<CursorPosition<DesktopOrigin>, Error> {
    let mut point = POINT::default();
    let result = unsafe { GetCursorPos(&mut point).ok() };

    if let Err(err) = result {
        if err.code().is_err() {
            return Err(err);
        }
    }

    Ok(CursorPosition::new(point.x, point.y))
}
