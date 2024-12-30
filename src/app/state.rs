use emath::Vec2;

use super::position::Position;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FractalType {
    // Keep in sync with the fragment shader
    Julia = 0,
    Mandelbrot = 1,
}

#[derive(Clone, Copy, Debug)]
pub struct State {
    pub center_position: Position,
    pub c_julia: Vec2,
    pub zoom: f32,
    pub contrast: f32,
    pub brightness: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub gamma: f32,
    pub high_quality: bool,
    pub fractal_type: FractalType,
}

impl State {
    pub fn new() -> State {
        State {
            center_position: Position { x: 0.0, y: 0.0 },
            c_julia: Vec2::new(-0.76, -0.08),
            zoom: 1000.0,
            contrast: 0.35,
            brightness: 0.0,
            r: 0.16,
            g: 0.40,
            b: 1.0,
            gamma: 1.25,
            high_quality: true,
            fractal_type: FractalType::Julia,
        }
    }
}
