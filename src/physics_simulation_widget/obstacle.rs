use super::*;

#[derive(Clone, Data, PartialEq, Debug)]
pub struct Obstacle {
    pub rectangle: Option<Rect>,
    pub circle: Option<Circle>,
    pub color: Color,
}

const MIN_SIZE: f64 = 10.;

impl Obstacle {
    pub fn new(p1: Point, p2: Point, color: Color) -> Self {
        let diag = p1.distance(p2);
        if diag < MIN_SIZE * 2. {
            //cirlce
            let center = Point::new((p1.x + p2.x) / 2., (p1.y + p2.y) / 2.);
            let radius = diag / 2.;
            return Self {
                rectangle: None,
                circle: Some(Circle::new(center, radius)),
                color,
            };
        } 
        // rectangle
        Self {
            rectangle: Some(Rect::from_points(p1, p2)),
            circle: None,
            color,
        }
    }

    pub fn equals(&self, other: &Obstacle) -> bool {
        self.rectangle == other.rectangle && self.circle == other.circle && self.color == other.color
    }

    pub fn paint(&self, ctx: &mut PaintCtx) {
        if let Some(circle) = &self.circle {
            // circle
            ctx.fill(circle, &self.color);
        } 
        if let Some(rectangle) = &self.rectangle {
            ctx.fill(rectangle, &self.color);
        }
    }

    pub fn collide(&self, balls: &mut Vec<Ball>, i: usize) {
        if let Some(rectangle) = self.rectangle {
            let mut ball = balls[i].clone();

            let delta_x = ball.x - f64::max(rectangle.x0, f64::min(ball.x, rectangle.x1));
            let delta_y = ball.y - f64::max(rectangle.y0, f64::min(ball.y, rectangle.y1));
            let distance = f64::sqrt(delta_x * delta_x + delta_y * delta_y);
            
            if distance >= ball.radius {
                return;
            } 
            
            if rectangle.contains(Point::new(ball.x, ball.y)) {
                // When the ball is inside the rectangle, we need to move it out of the rectangle
                let distance_from_top = ball.y - rectangle.min_y();
                let distance_from_bottom = rectangle.max_y() - ball.y;
                let distance_from_left = ball.x - rectangle.min_x();
                let distance_from_right = rectangle.max_x() - ball.x;
                let min_distance = f64::min(f64::min(distance_from_bottom, distance_from_top), f64::min(distance_from_left, distance_from_right));

                if min_distance == distance_from_bottom {
                    // Closest border is bottom
                    ball.y = rectangle.max_y() + ball.radius;
                    ball.vy = -ball.vy;
                    ball.vx += 0.1;
                } else if min_distance == distance_from_top {
                    // Closest border is top
                    ball.y = rectangle.min_y() - ball.radius;
                    ball.vy = -ball.vy;
                    ball.vx += 0.1;
                } else if min_distance == distance_from_left {
                    // Closest border is left
                    ball.x = rectangle.min_x() - ball.radius;
                    ball.vx = -ball.vx;
                    ball.vy += 0.1;
                } else if min_distance == distance_from_right {
                    // Closest border is right
                    ball.x = rectangle.max_x() + ball.radius;
                    ball.vx = -ball.vx;
                    ball.vy += 0.1;
                }

                balls[i] = ball;
                return;
            }

            let normal_x = delta_x / distance;
            let normal_y = delta_y / distance;
            
            let dot_product = ball.vx * normal_x + ball.vy * normal_y;
            
            if dot_product > 0. {
                return;
            }

            let overlap = ball.radius - distance;
            ball.x += delta_x * overlap / distance;
            ball.y += delta_y * overlap / distance;
    
            ball.vx -= 2. * dot_product * normal_x;
            ball.vy -= 2. * dot_product * normal_y;
    
            ball.vx *= 0.7;
            ball.vy *= 0.7;
    
            ball.x += ball.vx;
            ball.y += ball.vy;

            balls[i] = ball;
        }
    }
}