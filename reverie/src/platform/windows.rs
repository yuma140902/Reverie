pub use windows::core::Error;
use windows::Win32::UI::WindowsAndMessaging::SetCursorPos;

pub fn set_cursor_pos(x: i32, y: i32) -> Result<(), Error> {
    unsafe { SetCursorPos(x, y) }.ok()
}
