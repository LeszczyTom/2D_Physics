#![windows_subsystem = "windows"]

use druid::widget::prelude::*;
use druid::{ AppLauncher, Color, LocalizedString, WindowDesc, Rect, TimerToken, Point };
use druid::piet::kurbo::{Circle, Line};
use std::time::{Duration, Instant};

struct CustomWidget {
    timer_id: TimerToken,
    last_update: Instant,
    updates_per_second: u64,
    paused: bool,
}

impl Widget<AppData> for CustomWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppData, _env: &Env) {
        match event {
            Event::WindowConnected => {
                ctx.request_paint();
                let deadline = Duration::from_millis(1000 / self.updates_per_second);
                self.last_update = Instant::now();
                self.timer_id = ctx.request_timer(deadline);
            },
            Event::Timer(id) => {
                if *id == self.timer_id {
                    if data.size == SIZE {
                        data.set_size(ctx.size());
                    }
                    if !self.paused {
                        data.update();
                        ctx.request_paint();
                    }
                    let deadline = Duration::from_millis(1000 / self.updates_per_second);
                    self.last_update = Instant::now();
                    self.timer_id = ctx.request_timer(deadline);
                }
            },
            Event::MouseDown(e) => {
                data.preview.mouse_down_pos = Some(e.pos);
                let colors_id = data.balls.len().rem_euclid(8) ;
                data.preview.color = Some(COLORS[colors_id].clone());
                //self.mouse_down_pos = Some(e.pos);
            },
            Event::MouseUp(e) => {
                //println!("Mouse up at {:?}", e.pos);
                //println!("Mouse down at {:?}", self.mouse_down_pos);
                if data.preview.mouse_down_pos.is_none() {
                    return;
                }

                let mouse_down = data.preview.mouse_down_pos.unwrap();
                let mut delta_x = mouse_down.x - e.pos.x;
                let mut delta_y = mouse_down.y - e.pos.y;
                
                //scale down vector
                let mut scale = 1.;
                if delta_x.abs() > BALL_SIZE || delta_y.abs() > BALL_SIZE {
                    scale = 15. / delta_x.abs().max(delta_y.abs());
                }

                delta_x *= scale;
                delta_y *= scale;

                let new_ball = Ball::new(mouse_down.x, mouse_down.y, delta_x, delta_y, BALL_SIZE, data.preview.color.as_ref().unwrap().clone());
                data.balls.push(new_ball);
                //println!("Normal: {}, {}", delta_x, delta_y);
                data.preview.mouse_down_pos = None;
                data.preview.color = None;
                data.preview.arrow = None;
            },
            Event::MouseMove(e) => {
                if data.preview.mouse_down_pos.is_none() {
                    return;
                }
                let mouse_down_pos = data.preview.mouse_down_pos.unwrap();
                
                let delta_x = mouse_down_pos.x - e.pos.x;
                let delta_y = mouse_down_pos.y - e.pos.y;

                let angle = (delta_y / delta_x).atan();
                let x: f64;
                let y: f64;
                let r = BALL_SIZE * 4.;

                if e.pos.x < mouse_down_pos.x {
                    x = r * angle.cos() + mouse_down_pos.x;
                    y = r * angle.sin() + mouse_down_pos.y;
                } else {
                    x = -r * angle.cos() + mouse_down_pos.x;
                    y = -r * angle.sin() + mouse_down_pos.y;
                }        

                data.preview.arrow = Some(Line::new(mouse_down_pos, Point::new(x, y)));
            }
            _ => (),
        }

        if data.preview.mouse_down_pos.is_some() && !ctx.is_hot() {
            data.preview.mouse_down_pos = None;
            data.preview.arrow = None;
            data.preview.color = None;
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &AppData,
        _env: &Env,
    ) {
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppData, _data: &AppData, _env: &Env) {

    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppData,
        _env: &Env,
    ) -> Size {
        if bc.is_width_bounded() && bc.is_height_bounded() {
            return bc.max();
        } 
        bc.constrain(SIZE)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppData, _env: &Env) {
        // Paint the background
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &BACKGROUND_COLOR);

        // Paint the obstacles
        for obstacle in &data.obstacles {
            let rect = Rect::from_origin_size((obstacle.x, obstacle.y), (obstacle.width, obstacle.height));
            ctx.fill(rect, &obstacle.color);
        }

        // Paint the balls
        for ball in &data.balls {
            let circle = Circle::new((ball.x, ball.y), ball.radius);
            ctx.fill(circle, &ball.color);
        }

        // Paint the preview
        if data.preview.mouse_down_pos.is_some() && data.preview.color.is_some() {
            let cursor_pos = data.preview.mouse_down_pos.unwrap();
            let circle = Circle::new((cursor_pos.x, cursor_pos.y), BALL_SIZE);
            ctx.fill(circle, data.preview.color.as_ref().unwrap());

            if data.preview.arrow.is_some() {
                ctx.stroke(data.preview.arrow.as_ref().unwrap(), &Color::WHITE, 2.);
            }
        }
    }
}

#[derive(Clone, Data)]
struct AppData {
    #[data(same_fn="PartialEq::eq")]
    obstacles: Vec<Obstacle>,
    #[data(same_fn="PartialEq::eq")]
    balls: Vec<Ball>,
    size: Size,
    preview: BallPreview,
}

impl AppData {
    fn new() -> Self {
        Self {
            obstacles: Vec::new(),
            balls: Vec::new(),
            size: SIZE,
            preview: BallPreview::new(),
        }
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;

        self.obstacles.push( Obstacle { x: 0., y: 0., width: size.width, height: 1., color: Color::RED } );
        self.obstacles.push( Obstacle { x: 0., y: 0., width: 1., height: size.height, color: Color::RED } );
        self.obstacles.push( Obstacle { x: size.width - 1., y: 0., width: 1., height: size.height, color: Color::RED } );
        self.obstacles.push( Obstacle { x: 0., y: size.height - 1., width: size.width, height: 1., color: Color::RED } );
    }

    fn add_obstacle(&mut self, obstacle: Obstacle) {
        self.obstacles.push(obstacle);
    }

    fn add_ball(&mut self, ball: Ball) {
        self.balls.push(ball);
    }

    fn update(&mut self) {
        for ball in self.balls.iter_mut() {
            ball.update();
            for obstacle in self.obstacles.iter() {
                if !ball.resting {
                    resolve_colision(ball, obstacle);     
                }  
            }
        }
    }
}

fn resolve_colision(ball: &mut Ball, obstacle: &Obstacle) {
    let delta_x = ball.x - f64::max(obstacle.x, f64::min(ball.x, obstacle.x + obstacle.width));
    let delta_y = ball.y - f64::max(obstacle.y, f64::min(ball.y, obstacle.y + obstacle.height));
    if (delta_x * delta_x + delta_y * delta_y) > (ball.radius * ball.radius) {
        return;
    }

    let distance = f64::sqrt(delta_x * delta_x + delta_y * delta_y);
    if distance == 0. {
        return;
    }

    let normal_x = delta_x / distance;
    let normal_y = delta_y / distance;

    let dot_product = ball.vx * normal_x + ball.vy * normal_y;
    if dot_product > 0. {
        return;
    }
    
    if distance < ball.radius && f64::abs(ball.vy) < 0.1 && f64::abs(ball.vx) < 0.1 {
        ball.resting = true;
        return;
    }

    ball.vx -= 2. * dot_product * normal_x;
    ball.vy -= 2. * dot_product * normal_y;

    ball.vx *= 0.8;
    ball.vy *= 0.8;

    ball.x += ball.vx;
    ball.y += ball.vy;
}

#[derive(Clone, Data, PartialEq, Debug)]
struct Obstacle {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    color: Color,
}

#[derive(Clone, Data, PartialEq, Debug)]
struct Ball {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    radius: f64,
    color: Color,
    resting: bool,
    terminal_velocity: f64,
}

impl Ball {
    fn new(x: f64, y: f64, vx: f64, vy: f64, radius: f64, color: Color) -> Self {
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

    fn update(&mut self) {
        if self.resting {
            return;
        }

        if self.x < 0. || self.x > SIZE.width || self.y < 0. || self.y > SIZE.height {
            self.resting = true;
            println!("Ball out of bounds");
            return;
        }
        
        self.vx *= 0.99;
        self.vy *= 0.99;

        if self.vy < self.terminal_velocity {
            self.vy += GRAVITY;
        }

        self.x += self.vx;
        self.y += self.vy;
    }
}

#[derive(Clone, Data, PartialEq, Debug)]
struct BallPreview {
    color: Option<Color>,
    mouse_down_pos: Option<Point>,
    arrow: Option<Line>,
}

impl BallPreview {
    fn new() -> Self {
        Self {
            color: None,
            mouse_down_pos: None,
            arrow: None,
        }
    }
}

const BACKGROUND_COLOR: Color = Color::BLACK;
const SIZE: Size = Size::new(1000., 700.);
const UPDATE_PER_SECOND: u64 = 60;
const GRAVITY: f64 = 0.15;
const COLORS: [Color; 8] = [Color::RED, Color::GREEN, Color::BLUE, Color::YELLOW, Color::PURPLE, Color::AQUA, Color::MAROON, Color::TEAL];
const BALL_SIZE: f64 = 15.;

pub fn main() {
    let custom_widget: CustomWidget = CustomWidget {
        timer_id: TimerToken::INVALID,
        last_update: Instant::now(),
        updates_per_second: UPDATE_PER_SECOND,
        paused: false
    };
    let window = WindowDesc::new(|| {custom_widget})
                    .title(LocalizedString::new("2D_Physics"))
                    .window_size(SIZE)
                    .resizable(false);

    let launcher = AppLauncher::with_window(window);
    let mut data = AppData::new();
   
    data.add_ball(Ball::new(100.0, 100.0, 21.0, -10.0, BALL_SIZE, Color::WHITE));
    data.add_ball(Ball::new(20.0, 10.0, -5.0, -3.0, BALL_SIZE, Color::GREEN));
    data.add_ball(Ball::new(50.0, 50.0, 0.0, 15.0, BALL_SIZE, Color::BLUE));
    data.add_ball(Ball::new(220.0, 110.0, 0.0, 5.0, BALL_SIZE, Color::YELLOW));
    data.add_ball(Ball::new(400.0, 500.0, -10.0, 1.0, BALL_SIZE, Color::OLIVE));
    data.add_obstacle(Obstacle { x: 200., y: 200., width: 100., height: 100., color: Color::RED });
    data.add_obstacle(Obstacle { x: 50., y: 600., width: 500., height: 10., color: Color::RED });

    launcher
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
}