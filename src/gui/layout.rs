/// 長方形の領域
pub struct Rect<T, U> {
    origin_x: T,
    origin_y: T,
    width: U,
    height: U,
}

/// 長方形の領域のうち、レイアウトの基準とする部分
#[derive(PartialEq)]
pub enum Origin {
    TopLeft,
    Top,
    TopRight,
    Left,
    Center,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

impl<T, U> Rect<T, U> {
    pub fn new(origin_x: T, origin_y: T, width: U, height: U) -> Self {
        Self {
            origin_x,
            origin_y,
            width,
            height,
        }
    }

    pub fn origin_x(&self) -> &T {
        &self.origin_x
    }

    pub fn origin_y(&self) -> &T {
        &self.origin_y
    }

    pub fn width(&self) -> &U {
        &self.width
    }

    pub fn height(&self) -> &U {
        &self.height
    }
}
