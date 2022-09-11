use super::*;

#[derive(Clone, Data, PartialEq, Debug)]
pub struct Obstacle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub color: Color,
}

impl Obstacle {
    pub fn equals(&self, other: &Obstacle) -> bool {
        self.x == other.x && self.y == other.y && self.width == other.width && self.height == other.height && self.color == other.color
    }

    pub fn paint(&self, ctx: &mut PaintCtx) {
        let rect = Rect::from_origin_size((self.x, self.y), (self.width, self.height));
        ctx.fill(rect, &self.color);
    }
}
