use super::*;

#[derive(Clone, Data, Lens, PartialEq)]
pub struct Params {
    pub zero_gravity: bool,
    pub walls: bool,
    pub delete_tool: bool,
    pub spawn_tool: bool,
    pub attraction_tool: bool,
    pub move_tool: bool,
    pub paused: bool,
}

impl Params {
    pub fn new() -> Self {
        Self {
            zero_gravity: false,
            walls: true,
            delete_tool: false,
            spawn_tool: true,
            attraction_tool: true,
            move_tool: false,
            paused: false,
        }
    }
}

#[derive(Clone, Data, Lens)]
pub struct AppData {
    #[data(same_fn="PartialEq::eq")]
    pub obstacles: Vec<Obstacle>,
    #[data(same_fn="PartialEq::eq")]
    pub balls: Vec<Ball>,
    pub size: Size,
    pub preview: BallPreview,
    pub gravity_point: Option<Point>,
    pub gravity_tuple: (f64, f64),
    pub border_wall: Option<[Obstacle; 4]>,
    pub params: Params,
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
            border_wall: None,
            params: Params::new(),
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

    pub fn remove_ball(&mut self, ball: Ball) {
        for (i, b) in self.balls.iter().enumerate() {
            if ball.equals(b) {
                self.balls.remove(i);
                return;
            }
        }
    }

    pub fn add_ball(&mut self, ball: Ball) {
        self.balls.push(ball);
    }

    pub fn set_border_wall(&mut self) {
        let borders = [
            Obstacle::new(Point::new(0., self.size.height), Point::new(self.size.width, self.size.height + 100.), Color::RED),
            Obstacle::new(Point::new(self.size.width, 0.), Point::new(self.size.width + 100., self.size.height), Color::RED),
            Obstacle::new(Point::new(-100., 0.), Point::new(0., self.size.height), Color::RED),
            Obstacle::new(Point::new(0., -100.), Point::new(self.size.width, 0.), Color::RED), 
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
        for i in 0..4 {
            self.remove_obstacle(self.border_wall.as_ref().unwrap()[i].clone());
        }
        self.border_wall = None;
    }

    pub fn update(&mut self) {
        let mut balls = self.balls.clone();
        for i in 0..balls.len() {
            balls[i].update(self);
            for obstacle in self.obstacles.iter() {
                obstacle.collide(&mut balls, i);
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