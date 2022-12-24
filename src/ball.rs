/*
 ██████╗  ██████╗ ███████╗███████╗    ██████╗  ██████╗ ███╗   ██╗ ██████╗               ██████╗  █████╗ ██╗     ██╗     
██╔════╝ ██╔════╝ ██╔════╝╚══███╔╝    ██╔══██╗██╔═══██╗████╗  ██║██╔════╝               ██╔══██╗██╔══██╗██║     ██║     
██║  ███╗██║  ███╗█████╗    ███╔╝     ██████╔╝██║   ██║██╔██╗ ██║██║  ███╗    █████╗    ██████╔╝███████║██║     ██║     
██║   ██║██║   ██║██╔══╝   ███╔╝      ██╔═══╝ ██║   ██║██║╚██╗██║██║   ██║    ╚════╝    ██╔══██╗██╔══██║██║     ██║     
╚██████╔╝╚██████╔╝███████╗███████╗    ██║     ╚██████╔╝██║ ╚████║╚██████╔╝              ██████╔╝██║  ██║███████╗███████╗
 ╚═════╝  ╚═════╝ ╚══════╝╚══════╝    ╚═╝      ╚═════╝ ╚═╝  ╚═══╝ ╚═════╝               ╚═════╝ ╚═╝  ╚═╝╚══════╝╚══════╝
                                                                                                                        
 */

 ///
 /// This module contain ball context
 /// 
use ggez::{graphics::{Rect, Color, DrawParam, Drawable, self}, mint::Vector2 };
use rand::{thread_rng, Rng};
use crate::common::{Limits, Direction, VelocityRange};


pub struct Ball {
    rect: Option<Rect>,
    default_rect: Option<Rect>,
    color: Color,
    limits_out: Limits,
    limits_bounce: Limits,
    pub velocity: Vector2<f32>,
    velocity_range: VelocityRange,
}

impl Drawable for Ball {
    fn dimensions(&self, _gfx: &impl ggez::context::Has<graphics::GraphicsContext>) -> Option<Rect> {
        self.rect
    }

    fn draw(&self, canvas: &mut graphics::Canvas, _param: impl Into<DrawParam>) {
        if let Some(rect) = self.rect {       
            canvas.draw(
                &graphics::Quad, 
                graphics::DrawParam::new()
                    .dest(rect.point())
                    .color(self.color)
                    .scale(rect.size()
                )
            );
        }
    }
}

impl Default for Ball {
    fn default() -> Self {
        Ball { 
            color: Color::WHITE,
            rect: None,
            limits_out: Limits::default(),
            limits_bounce: Limits::default(),
            velocity: Vector2 { x: 0., y: 0. },
            default_rect: None,
            velocity_range: VelocityRange::default(),
        }
    }
}

#[allow(dead_code)]
impl Ball {
    pub fn new() -> Self {
        Ball { 
            ..Default::default()
        }
    }

    pub fn set_limits_out(mut self, limits: Limits) -> Self {
        self.limits_out = limits;
        self
    }
    
    pub fn set_limits_bounce(mut self, limits: Limits) -> Self {
        self.limits_bounce = limits;
        self
    }

    pub fn set_default_rect(mut self, x: f32, y: f32 , w: f32, h: f32) -> Self {
        self.default_rect = Some(Rect::new(x, y, w, h));
        self
    }

    pub fn set_rect(mut self, x: f32, y: f32, w: f32, h: f32) -> Self {
        self.rect = Some(Rect::new(x, y, w, h));
        self
    }

    pub fn set_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn is_out(&self) -> Direction {
        if let Some(rect) = self.rect {
            match(self.limits_out.is_out_min_x(rect.x), 
                  self.limits_out.is_out_min_y(rect.y),
                  self.limits_out.is_out_max_x(rect.x + rect.w),
                  self.limits_out.is_out_max_y(rect.y + rect.h)) {
                    (Ok(_), _, _, _) => return Direction::Left,
                    (_, Ok(_), _, _) => return Direction::Top,
                    (_, _, Ok(_), _) => return Direction::Right,
                    (_, _, _, Ok(_)) => return Direction::Bottom,
                    _ => return Direction::None
                  }
        } else {
            return Direction::None;
        }
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn set_velocity_range(mut self, min_x: f32, max_x: f32, min_y: f32, max_y: f32, apply: bool) -> Self {
        self.velocity_range = VelocityRange {
            min_x,
            max_x,
            min_y,
            max_y,
        };
        if apply {
            self.apply_random_velocity();
        }
        self
    }

    pub fn apply_random_velocity(&mut self) {

        let mut rng = thread_rng(); // initialize random number generator
        let mut x_vel = rng.gen_range(self.velocity_range.min_x..self.velocity_range.max_x); // generate random 
        let mut y_vel = rng.gen_range(self.velocity_range.min_y..self.velocity_range.max_y);

        // rng.gen::<bool> generates either true or false with a 50% chance of each
        if rng.gen::<bool>() {
            x_vel *= -1.0;
        }
        if rng.gen::<bool>() {
            y_vel *= -1.0;
        }

        self.velocity.x = x_vel;
        self.velocity.y = y_vel;
    }

    pub fn apply_default_rect(&mut self) {
        self.rect = self.default_rect.clone();
    }

    pub fn reset(&mut self, default_rect: bool, random_velocity: bool) {
        if default_rect {
            self.apply_default_rect();
        }
        if random_velocity {
            self.apply_random_velocity();
        }
    }

    pub fn apply_velocity(&mut self, dt: f32) {
        if let Some(rect) = &mut self.rect {
            let velocity: Vector2<f32> = Vector2 { x: self.velocity.x * dt, y: self.velocity.y * dt };
            rect.translate(velocity);

            // Bounds ?
            if let Ok(x) = self.limits_bounce.is_out_min_x(rect.x) {
                rect.x = x;
                self.velocity.x *= -1.;
            }
            if let Ok(x) = self.limits_bounce.is_out_max_x(rect.x + rect.w) {
                rect.x = x - rect.w;
                self.velocity.x *= -1.;
            }
            if let Ok(y) = self.limits_bounce.is_out_min_y(rect.y) {
                rect.y = y;
                self.velocity.y *= -1.;
            }
            if let Ok(y) = self.limits_bounce.is_out_max_y(rect.y + rect.h) {
                rect.y = y - rect.h;
                self.velocity.y *= -1.;
            }
        }
    }

    pub fn set_x(&mut self, x: f32) -> Result<(), String> {
        if let Some(mut rect) = self.rect {
            rect.x = x;
            return Ok(());
        } else {
            return Err("Rect not defined".to_string());
        }
    }

    pub fn set_y(&mut self, y: f32) -> Result<(), String> {
        if let Some(mut rect) = self.rect {
            rect.y = y;
            return Ok(());
        } else {
            return Err("Rect not defined".to_string());
        }
    }

    pub fn collide(&self, other_rect: &Option<Rect>) -> bool {
        if other_rect.is_none() {
            return false;
        }
        if let Some(rect) = self.rect {
          return rect.overlaps(&other_rect.unwrap());
        } else {
            return false;
        }

    }


}
