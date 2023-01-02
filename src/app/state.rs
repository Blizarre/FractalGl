use super::pos::Pos;

#[derive(Clone, Copy, Debug)]
pub struct State {
    pub pos: Pos,
    pub julia_expanded: bool,
    pub c_julia: Pos,
    pub zoom: f32,
    pub color_expanded: bool,
    pub contrast: f32,
    pub brightness: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
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
            r: 0.8,
            g: 0.75,
            b: 1.0,
        }
    }
}
