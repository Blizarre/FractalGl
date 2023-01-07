use eframe::epaint::Pos2;
use std::ops::{Div, Sub};

#[derive(Clone, Copy, Debug)]
/// Location in the fractal space, by opposition to [Pos2] which is a location
/// in the UI space. Provides ways to convert from [Pos2] to [Position].
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    /// Convert a [Pos2] coordinate into a [Position].
    ///
    /// *Note: The coordinate will still be relative to the upper-left corner of the visible fractal area.*
    pub fn from_screen_space(pixels_per_point: f32, screen_space: Pos2) -> Self {
        Self {
            x: screen_space.x * pixels_per_point,
            y: screen_space.y * pixels_per_point,
        }
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Div<f32> for Position {
    type Output = Position;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
