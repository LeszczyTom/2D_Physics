pub mod appdata;
pub mod obstacle;
pub mod ball;
pub mod ball_preview;

pub use appdata::AppData;
pub use obstacle::Obstacle;
pub use ball::Ball;
pub use ball_preview::BallPreview;
use druid::widget::prelude::*;
use druid::{ Color, Rect, TimerToken, Point, MouseButton, Lens };
use druid::piet::kurbo::{ Circle, Line };
use std::time::{ Duration, Instant };

const BACKGROUND_COLOR: Color = Color::BLACK;
const COLORS: [Color; 8] = [Color::RED, Color::GREEN, Color::SILVER, Color::YELLOW, Color::PURPLE, Color::AQUA, Color::MAROON, Color::TEAL];
const DEFAULT_BALL_SIZE: f64 = 15.;
const DEFAULT_GRAVITY_TUPLE: (f64, f64) = (0., 0.2);

pub struct PhysicsSimulationWidget {
    timer_id: TimerToken,
    last_update: Instant,
    updates_per_second: u64,
}

impl PhysicsSimulationWidget {
    pub fn new(updates_per_second: u64) -> Self {
        PhysicsSimulationWidget {
            timer_id: TimerToken::INVALID,
            last_update: Instant::now(),
            updates_per_second,
        }
    }
}

impl Widget<AppData> for PhysicsSimulationWidget {
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
                    if data.size != ctx.size() {
                        data.set_size(ctx.size());
                    }
                    if !data.params.paused {
                        data.update();
                        ctx.request_paint();
                    }
                    let deadline = Duration::from_millis(1000 / self.updates_per_second);
                    self.last_update = Instant::now();
                    self.timer_id = ctx.request_timer(deadline);


                    if data.params.zero_gravity && data.gravity_tuple != (0., 0.) {
                        data.gravity_tuple = (0., 0.);
                    } else if !data.params.zero_gravity && data.gravity_tuple == (0., 0.) {
                        data.gravity_tuple = DEFAULT_GRAVITY_TUPLE;
                    } 

                    if data.params.walls && data.border_wall.is_none() {
                        data.set_border_wall();
                    } else if !data.params.walls && data.border_wall.is_some() {
                        data.remove_wall();
                    }
                }
            },
            Event::MouseMove(_e) => {
                if !ctx.is_hot() {
                    data.preview.mouse_down_pos = None;
                    data.preview.arrow = None;
                    data.preview.color = None;
                    data.gravity_point = None;
                    return;                    
                }  
            },
            _ => {}        
        } 
        
        if data.params.paused {
            return;
        }

        attraction_tool(event, data);
        move_tool(event, data);
        delete_tool(event, data);
        spawn_tool(event, data);
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
        let size = Size::new(100.0, 100.0);
        bc.constrain(size)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppData, _env: &Env) {
        // Paint the background
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &BACKGROUND_COLOR);

        // Paint the obstacles but not the walls
        for obstacle in &data.obstacles {
            if let Some(border_wall) = &data.border_wall {
                if obstacle == &border_wall[0] || obstacle == &border_wall[1] || obstacle == &border_wall[2] || obstacle == &border_wall[3] {
                    continue;
                }
            }
                 
            obstacle.paint(ctx);
        }
       
        // Paint the balls
        for ball in &data.balls {
            ball.paint(ctx);
        }

        // Paint the preview
        data.preview.paint(ctx, DEFAULT_BALL_SIZE);
    }
}

fn resolve_ball_overlap(balls: &mut Vec<Ball>, i: usize, j: usize) {
    let mut b1 = balls[i].clone();
    let mut b2 = balls[j].clone();

    let delta_x = b1.x - f64::max(b2.x, f64::min(b1.x, b2.x));
    let delta_y = b1.y - f64::max(b2.y, f64::min(b1.y, b2.y));
    let radius_sum: f64 = b1.radius + b2.radius;
    if (delta_x * delta_x + delta_y * delta_y) > (radius_sum * radius_sum) {
        return ;
    }

    let distance = f64::sqrt(delta_x * delta_x + delta_y * delta_y);
    if distance == 0. {
        return;
    }

    b1.resting = false;
    b2.resting = false;

    let overlap = 0.5 * (distance - radius_sum);

    b1.x -= delta_x * overlap / distance;
    b1.y -= delta_y * overlap / distance;

    b2.x += delta_x * overlap / distance;
    b2.y += delta_y * overlap / distance;

    let normal_x = delta_x / distance;
    let normal_y = delta_y / distance;

    let tan_x = -normal_y;
    let tan_y = normal_x;

    let dot_product_tan_b1 = b1.vx * tan_x + b1.vy * tan_y;
    let dot_product_tan_b2 = b2.vx * tan_x + b2.vy * tan_y;

    let dot_product_normal_b1 = b1.vx * normal_x + b1.vy * normal_y;
    let dot_product_normal_b2 = b2.vx * normal_x + b2.vy * normal_y;

    //https://en.wikipedia.org/wiki/Elastic_collision
    // Same mass for the moment
    let v1 = dot_product_normal_b2;
    let v2 = dot_product_normal_b1;

    b1.vx = tan_x * dot_product_tan_b1 + normal_x * v1;
    b1.vy = tan_y * dot_product_tan_b1 + normal_y * v1;
    b2.vx = tan_x * dot_product_tan_b2 + normal_x * v2;
    b2.vy = tan_y * dot_product_tan_b2 + normal_y * v2;

    if b1.vx.abs() < 0.1 {
        b1.vx = 0.;
    }
    if b1.vy.abs() < 0.1 {
        b1.vy = 0.;
    }
    if b2.vx.abs() < 0.1 {
        b2.vx = 0.;
    }
    if b2.vy.abs() < 0.1 {
        b2.vy = 0.;
    }

    balls[i] = b1;
    balls[j] = b2;
}

fn are_balls_overlapping(b1: &Ball, b2: &Ball) -> bool {
    let ac = b1.x - f64::max(b2.x, f64::min(b1.x, b2.x));
    let bc = b1.y - f64::max(b2.y, f64::min(b1.y, b2.y));
    let radius_sum: f64 = b1.radius + b2.radius;
    if (ac * ac + bc * bc) < (radius_sum * radius_sum) {
        return true;
    }
    false
}

pub fn get_new_appdata(size: Size) -> AppData {
    let mut data = AppData::new(size, DEFAULT_GRAVITY_TUPLE);

    data.add_ball(Ball::new(100.0, 100.0, 21.0, -10.0, DEFAULT_BALL_SIZE, Color::WHITE));
    data.add_ball(Ball::new(25.0, 15.0, -5.0, -3.0, DEFAULT_BALL_SIZE, Color::GREEN));
    data.add_ball(Ball::new(50.0, 50.0, 0.0, 15.0, DEFAULT_BALL_SIZE, Color::SILVER));
    data.add_ball(Ball::new(220.0, 110.0, 0.0, 5.0, DEFAULT_BALL_SIZE, Color::YELLOW));
    data.add_ball(Ball::new(400.0, 500.0, -10.0, 1.0, DEFAULT_BALL_SIZE, Color::OLIVE));
    data.add_obstacle(Obstacle::new(Point::new(200., 200.), Point::new(300., 300.), Color::RED));
    data.add_obstacle(Obstacle::new(Point::new(50., 600.), Point::new(550., 610.), Color::RED));
    data.add_obstacle(Obstacle::new(Point::new(50., 50.), Point::new(59., 59.), Color::RED));
    
    data
}

fn attraction_tool(event: &Event, data: &mut AppData) {    
    if !data.params.attraction_tool {
        return;
    }

    match event {
        Event::MouseDown(e) => {
            if e.button != MouseButton::Right {
                return;
            }
                
            data.gravity_point = Some(e.pos);
        },
        Event::MouseUp(e) => {
            if e.button != MouseButton::Right {
                return;
            }

            data.gravity_point = None;
        },
        Event::MouseMove(e) => {
            if e.buttons.has_right() {
                if data.params.attraction_tool {
                    data.gravity_point = Some(e.pos);
                }
            } 
        },
        _ => {} 
    };
}

fn move_tool(event: &Event, data: &mut AppData) {
    if !data.params.move_tool {
        return;
    }
    match event {
        Event::MouseUp(e) => {
            if e.button != MouseButton::Left {
                return;
            }

            for i in 0..data.balls.len() {
                if data.balls[i].contains_point(e.pos) {
                    data.balls[i].resting = false;
                    break;
                }
            }  
        },
        Event::MouseMove(e) => {
            if e.buttons.has_left() {
                for i in 0..data.balls.len() {
                    if data.balls[i].contains_point(e.pos) {
                        data.balls[i].move_ball(e.pos);
                        break;
                    }
                }  
            }
        },
        _ => {}
    }
}

fn delete_tool(event: &Event, data: &mut AppData) {
    if !data.params.delete_tool {
        return;
    }

    match event {
        Event::MouseDown(e) => {
            if e.button != MouseButton::Right {
                return;
            }

            for i in 0..data.balls.len() {
                if data.balls[i].contains_point(e.pos) {
                    data.balls.remove(i);
                    break;
                }
            }   
        },
        _ => {}
    }
}

fn spawn_tool(event: &Event, data: &mut AppData) {
    if !data.params.spawn_tool {
        return;
    }
    match event {
        Event::MouseDown(e) => {
            if e.button != MouseButton::Left {
                return;
            }

            if data.params.spawn_tool {
                data.preview.mouse_down_pos = Some(e.pos);
                data.preview.color = Some(COLORS[data.balls.len().rem_euclid(8)].clone());
            }   
        },
        Event::MouseUp(e) => {
            if e.button != MouseButton::Left || data.preview.mouse_down_pos.is_none() {
                return;
            }

            let mouse_down = data.preview.mouse_down_pos.unwrap();
            let mut delta_x = mouse_down.x - e.pos.x;
            let mut delta_y = mouse_down.y - e.pos.y;
            
            //scale down vector
            let mut scale = 1.;
            if delta_x.abs() > DEFAULT_BALL_SIZE || delta_y.abs() > DEFAULT_BALL_SIZE {
                scale = 15. / delta_x.abs().max(delta_y.abs());
            }

            delta_x *= scale;
            delta_y *= scale;

            let new_ball = Ball::new(mouse_down.x, mouse_down.y, delta_x, delta_y, DEFAULT_BALL_SIZE, data.preview.color.as_ref().unwrap().clone());
            data.balls.push(new_ball);
            //println!("Normal: {}, {}", delta_x, delta_y);
            data.preview.mouse_down_pos = None;
            data.preview.color = None;
            data.preview.arrow = None;
        },
        Event::MouseMove(e) => {
            if !data.params.spawn_tool || !e.buttons.has_left(){
                return;
            }
                
            let mouse_down_pos = data.preview.mouse_down_pos.unwrap();
        
            let delta_x = mouse_down_pos.x - e.pos.x;
            let delta_y = mouse_down_pos.y - e.pos.y;

            let angle = (delta_y / delta_x).atan();
            let x: f64;
            let y: f64;
            let r = 60.;

            if e.pos.x < mouse_down_pos.x {
                x = r * angle.cos() + mouse_down_pos.x;
                y = r * angle.sin() + mouse_down_pos.y;
            } else {
                x = -r * angle.cos() + mouse_down_pos.x;
                y = -r * angle.sin() + mouse_down_pos.y;
            }  

            data.preview.arrow = Some(Line::new(mouse_down_pos, Point::new(x, y)));          
        },
        _ => {}
    }
}