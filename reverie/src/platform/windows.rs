pub use windows::core::Error;
use windows::Win32::UI::WindowsAndMessaging::SetCursorPos;

pub fn set_cursor_pos(x: i32, y: i32) -> Result<(), Error> {
    let result = unsafe { SetCursorPos(x, y) }.ok();
    if let Err(ref err) = result {
        if err.code().is_ok() {
            return Ok(());
        }
    }
    result
}
