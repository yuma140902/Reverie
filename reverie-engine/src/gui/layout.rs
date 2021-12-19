use std::fmt::Debug;

/// 長方形の領域
#[derive(Debug, PartialEq)]
pub struct Rect<T, U>
where
    T: PartialEq,
    U: PartialEq,
{
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

impl Origin {
    pub fn x_diff(&self, width: u32) -> i32 {
        use Origin::*;
        match *self {
            TopLeft | Left | BottomLeft => 0,
            Top | Center | Bottom => width as i32 / 2,
            TopRight | Right | BottomRight => width as i32,
        }
    }

    pub fn y_diff(&self, height: u32) -> i32 {
        use Origin::*;
        match *self {
            TopLeft | Top | TopRight => 0,
            Left | Center | Right => height as i32 / 2,
            BottomLeft | Bottom | BottomRight => height as i32,
        }
    }
}

pub enum Position<T> {
    Positive(T),
    Center(T),
    Negative(T),
}

impl Position<i32> {
    pub fn actual_value(&self, max: i32) -> i32 {
        match *self {
            Position::Positive(distance) => distance,
            Position::Center(distance) => max / 2_i32 + distance,
            Position::Negative(distance) => max - distance,
        }
    }
}

impl<T, U> Rect<T, U>
where
    T: PartialEq,
    U: PartialEq,
{
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

impl Rect<i32, u32> {
    pub const fn new_const(origin_x: i32, origin_y: i32, width: u32, height: u32) -> Self {
        Self {
            origin_x,
            origin_y,
            width,
            height,
        }
    }

    pub fn new_in_rect(
        outer: &Rect<i32, u32>,
        origin: &Origin,
        position_x: &Position<i32>,
        position_y: &Position<i32>,
        inner_width: u32,
        inner_height: u32,
    ) -> Self {
        let x = outer.origin_x() + position_x.actual_value(*outer.width() as i32)
            - origin.x_diff(inner_width);
        let y = outer.origin_y() + position_y.actual_value(*outer.height() as i32)
            - origin.y_diff(inner_height);
        Self {
            origin_x: x,
            origin_y: y,
            width: inner_width,
            height: inner_height,
        }
    }

    pub fn new_biggest_in_rect(outer: &Rect<i32, u32>, inner_width: u32, inner_height: u32) -> Self {
        let outer_aspect: f32 = *outer.width() as f32 / *outer.height() as f32;
        let inner_aspect: f32 = inner_width as f32 / inner_height as f32;

        let (big_inner_width, big_inner_height) = if outer_aspect >= inner_aspect
        /* 外枠のRectの方が横に長い */
        {
            let width = (*outer.height() as f32 * inner_aspect) as u32;
            let height = *outer.height();
            (width, height)
        } else
        /* 外枠のRectの方が縦に長い */
        {
            let width = *outer.width();
            let height = (*outer.width() as f32 / inner_aspect) as u32;
            (width, height)
        };

        Self::new_in_rect(
            outer,
            &Origin::Center,
            &Position::Center(0),
            &Position::Center(0),
            big_inner_width,
            big_inner_height,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_x_diff() {
        assert_eq!(Origin::TopLeft.x_diff(10), 0);
        assert_eq!(Origin::Center.x_diff(10), 5);
        assert_eq!(Origin::BottomRight.x_diff(10), 10);
    }

    #[test]
    fn origin_y_diff() {
        assert_eq!(Origin::TopLeft.y_diff(10), 0);
        assert_eq!(Origin::Center.y_diff(10), 5);
        assert_eq!(Origin::BottomRight.y_diff(10), 10);
    }

    #[test]
    fn position_positive() {
        assert_eq!(Position::Positive(10).actual_value(100), 10);
        assert_eq!(Position::Positive(0).actual_value(100), 0);
    }

    #[test]
    fn position_center() {
        assert_eq!(Position::Center(10).actual_value(100), 60);
        assert_eq!(Position::Center(-10).actual_value(100), 40);
        assert_eq!(Position::Center(0).actual_value(100), 50);
    }

    #[test]
    fn position_negative() {
        assert_eq!(Position::Negative(10).actual_value(100), 90);
        assert_eq!(Position::Negative(0).actual_value(100), 100);
    }

    #[test]
    fn new_rect_in_rect_topleft() {
        let outer = Rect::<i32, u32>::new(0, 0, 400, 300);
        let rect = Rect::new_in_rect(
            &outer,
            &Origin::TopLeft,
            &Position::Positive(10),
            &Position::Positive(20),
            100,
            100,
        );

        assert_eq!(rect, Rect::new(10, 20, 100, 100));
    }

    #[test]
    fn new_rect_in_rect_center() {
        let outer = Rect::<i32, u32>::new(0, 0, 400, 300);
        let rect = Rect::new_in_rect(
            &outer,
            &Origin::Center,
            &Position::Center(0),
            &Position::Center(0),
            100,
            100,
        );

        assert_eq!(rect, Rect::new(150, 100, 100, 100));
    }
}
