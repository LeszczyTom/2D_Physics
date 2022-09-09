use super::*;

#[derive(Clone, Data, PartialEq, Debug)]
pub struct BallPreview {
    pub color: Option<Color>,
    pub mouse_down_pos: Option<Point>,
    pub arrow: Option<Line>,
}

impl BallPreview {
    pub fn new() -> Self {
        Self {
            color: None,
            mouse_down_pos: None,
            arrow: None,
        }
    }
}