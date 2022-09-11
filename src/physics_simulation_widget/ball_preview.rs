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

    pub fn paint(&self, ctx: &mut PaintCtx, ball_size: f64) {
        if let Some(color) = &self.color {
            if let Some(mouse_down_pos) = self.mouse_down_pos {
                let circle = Circle::new((mouse_down_pos.x, mouse_down_pos.y), ball_size);
                ctx.fill(circle, color);
                if let Some(arrow) = self.arrow {
                    ctx.stroke(arrow, &Color::WHITE, 2.);
                }
            }
        }
    }
}