use std::ops::{Div, Sub};

use eframe::epaint::Pos2;

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

#[derive(Clone, Copy, Debug)]
pub struct State {
    pub pos: Pos,
    pub julia_expanded: bool,
    pub c_julia: Pos,
    pub zoom: f32,
    pub color_expanded: bool,
    pub contrast: f32,
    pub brightness: f32,
}

impl State {
    pub fn new() -> State {
        State {
            pos: Pos { x: 0.0, y: 0.0 },
            julia_expanded: true,
            c_julia: Pos { x: -0.76, y: -0.08 },
            zoom: 3000.0,
            color_expanded: true,
            contrast: 0.3,
            brightness: -0.73,
        }
    }
}
