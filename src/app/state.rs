use super::pos::Pos;

#[derive(Clone, Copy, Debug)]
pub struct State {
    pub pos: Pos,
    pub c_julia: Pos,
    pub zoom: f32,
    pub contrast: f32,
    pub brightness: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub gamma: f32,
}

impl State {
    pub fn new() -> State {
        State {
            pos: Pos { x: 0.0, y: 0.0 },
            c_julia: Pos { x: -0.76, y: -0.08 },
            zoom: 3000.0,
            contrast: 0.35,
            brightness: 0.0,
            r: 0.16,
            g: 0.40,
            b: 1.0,
            gamma: 1.25,
        }
    }
}
