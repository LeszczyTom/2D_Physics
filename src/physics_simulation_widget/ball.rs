use super::*;

#[derive(Clone, Data, PartialEq, Debug)]
pub struct Ball {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub radius: f64,
    pub color: Color,
    pub resting: bool,
    pub terminal_velocity: f64,
}

impl Ball {
    pub fn new(x: f64, y: f64, vx: f64, vy: f64, radius: f64, color: Color) -> Self {
        Self {
            x,
            y,
            vx,
            vy,
            radius,
            color,
            resting: false,
            terminal_velocity: radius,
        }
    }

    pub fn equals(&self, other: &Ball) -> bool {
        self.x == other.x && self.y == other.y && self.vx == other.vx && self.vy == other.vy && self.radius == other.radius && self.color == other.color
    }

    pub fn contains_point(&self, point: Point) -> bool {
        let dx = self.x - point.x;
        let dy = self.y - point.y;
        dx * dx + dy * dy <= self.radius * self.radius
    }

    pub fn move_ball(&mut self, cursor_pos: Point) {
        let delta_x = self.x - cursor_pos.x;
        let delta_y = self.y - cursor_pos.y;

        self.x = cursor_pos.x;
        self.y = cursor_pos.y;
        
        self.vx = 0.;
        self.vy = 0.;

        self.resting = true;
    }

    pub fn update(&mut self, data: &AppData) {
        if self.resting {
            return;
        }

        if self.x < 0. || self.x > data.size.width || self.y < 0. || self.y > data.size.height {
            if data.params.walls {
                self.resting = true;
                println!("Ball out of bounds");
                return;
            } else {
                if self.x < 0. {
                    self.x = data.size.width;
                } else if self.x > data.size.width {
                    self.x = 0.;
                }

                if self.y < 0. {
                    self.y = data.size.height;
                } else if self.y > data.size.height {
                    self.y = 0.;
                }
            }
        }
        
        self.vx *= 0.99;
        self.vy *= 0.99;

        if data.gravity_point.is_none() {
            if self.vx < self.terminal_velocity {
                self.vx += data.gravity_tuple.0;
            }
            if self.vy < self.terminal_velocity {
                self.vy += data.gravity_tuple.1;
            }            
        } else {
            let distance_from_point = f64::sqrt((self.x - data.gravity_point.unwrap().x).powi(2) + (self.y - data.gravity_point.unwrap().y).powi(2));

            let normal_x = (self.x - data.gravity_point.unwrap().x) / distance_from_point;
            let normal_y = (self.y - data.gravity_point.unwrap().y) / distance_from_point;

            let dot_product = self.vx * normal_x + self.vy * normal_y;
            
            self.vx = (dot_product * normal_x) + 0.5 * -normal_x;
            self.vy = (dot_product * normal_y) + 0.5 * -normal_y;
        }
        
        self.x += self.vx;
        self.y += self.vy;
    }
}
