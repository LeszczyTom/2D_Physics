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

    pub fn update(&mut self, gravity_point: Option<Point>, gravity_tuple: (f64, f64)) {
        if self.resting {
            return;
        }

        if self.x < 0. || self.x > 5000. || self.y < 0. || self.y > 5000. {
            self.resting = true;
            println!("Ball out of bounds");
            return;
        }
        
        self.vx *= 0.99;
        self.vy *= 0.99;

        if gravity_point.is_none() {
            if self.vx < self.terminal_velocity {
                self.vx += gravity_tuple.0;
            }
            if self.vy < self.terminal_velocity {
                self.vy += gravity_tuple.1;
            }            
        } else {
            let distance_from_point = f64::sqrt((self.x - gravity_point.unwrap().x).powi(2) + (self.y - gravity_point.unwrap().y).powi(2));

            let normal_x = (self.x - gravity_point.unwrap().x) / distance_from_point;
            let normal_y = (self.y - gravity_point.unwrap().y) / distance_from_point;

            let dot_product = self.vx * normal_x + self.vy * normal_y;
            
            self.vx = (dot_product * normal_x) + 0.5 * -normal_x;
            self.vy = (dot_product * normal_y) + 0.5 * -normal_y;
        }
        
        self.x += self.vx;
        self.y += self.vy;
    }
}
