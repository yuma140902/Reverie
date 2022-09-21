use std::marker::PhantomData;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CursorPosition<O> {
    pub x: i32,
    pub y: i32,
    _phantom: PhantomData<O>,
}

impl<O> CursorPosition<O> {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            _phantom: PhantomData,
        }
    }
}

impl CursorPosition<DesktopOrigin> {
    pub fn to_window_origin(&self, window_x: i32, window_y: i32) -> CursorPosition<WindowOrigin> {
        CursorPosition::<WindowOrigin>::new(self.x - window_x, self.y - window_y)
    }

    pub fn to_window_center(
        &self,
        window_x: i32,
        window_y: i32,
        window_width: i32,
        window_height: i32,
    ) -> CursorPosition<WindowCenter> {
        CursorPosition::<WindowCenter>::new(
            self.x - window_x - window_width / 2,
            self.y - window_y - window_height / 2,
        )
    }
}

pub struct DesktopOrigin;
pub struct WindowOrigin;
pub struct WindowCenter;
