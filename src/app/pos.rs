use eframe::epaint::Pos2;
use std::ops::{Div, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}

impl Pos {
    pub fn from_screen_space(pixels_per_point: f32, screen_space: Pos2) -> Self {
        Self {
            x: screen_space.x * pixels_per_point,
            y: screen_space.y * pixels_per_point,
        }
    }
}

impl Sub for Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Div<f32> for Pos {
    type Output = Pos;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
