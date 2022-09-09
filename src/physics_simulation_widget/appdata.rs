use super::*;

#[derive(Clone, Data)]
pub struct AppData {
    #[data(same_fn="PartialEq::eq")]
    pub obstacles: Vec<Obstacle>,
    #[data(same_fn="PartialEq::eq")]
    pub balls: Vec<Ball>,
    pub size: Size,
    pub preview: BallPreview,
    pub gravity_point: Option<Point>,
    pub gravity_tuple: (f64, f64),
    pub border_wall: Option<[Obstacle; 4]>
}

impl AppData {
    pub fn new(size: Size, gravity_tuple: (f64, f64)) -> Self {
        Self {
            obstacles: Vec::new(),
            balls: Vec::new(),
            size,
            preview: BallPreview::new(),
            gravity_point: None,
            gravity_tuple,
            border_wall: None
        }
    }

    pub fn set_size(&mut self, size: Size) {
        self.size = size;
        self.set_border_wall();
    }

    pub fn add_obstacle(&mut self, obstacle: Obstacle) {
        self.obstacles.push(obstacle);
    }

    pub fn remove_obstacle(&mut self, obstacle: Obstacle) {
        for (i, obs) in self.obstacles.iter().enumerate() {
            if obstacle.equals(obs) {
                self.obstacles.remove(i);
                return;
            }
        }
    }

    pub fn add_ball(&mut self, ball: Ball) {
        self.balls.push(ball);
    }

    pub fn set_border_wall(&mut self) {
        let borders = [
            Obstacle { x: 0., y: self.size.height - 1., width: self.size.width, height: 100., color: Color::RED },
            Obstacle { x: self.size.width - 1., y: 0., width: 100., height: self.size.height, color: Color::RED },
            Obstacle { x: -99., y: 0., width: 100., height: self.size.height, color: Color::RED },
            Obstacle { x: 0., y: -99., width: self.size.width, height: 100., color: Color::RED },
        ];

        if self.border_wall.is_some() {
            for i in 0..4 {
                self.remove_obstacle(self.border_wall.as_ref().unwrap()[i].clone());
            }
        }
        for i in 0..4 {
            self.add_obstacle(borders[i].clone());
        }
        self.border_wall = Some(borders);
    }

    pub fn remove_wall(&mut self) {
        self.border_wall = None;
    }

    pub fn update(&mut self) {
        let mut balls = self.balls.clone();
        for i in 0..balls.len() {
            balls[i].update(self.gravity_point, self.gravity_tuple);
            for obstacle in self.obstacles.iter() {
                if are_ball_obstacle_overlapping(&balls[i], obstacle) {
                    resolve_overlap(&mut balls, i, obstacle);
                }
            }

            for j in 0..balls.len() {
                if i != j {
                    if are_balls_overlapping(&balls[i], &balls[j]) {
                        resolve_ball_overlap(&mut balls, i, j);
                    }
                }
            }
        }
        self.balls = balls;
    }
}