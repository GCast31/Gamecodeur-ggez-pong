/*
 ██████╗  ██████╗ ███████╗███████╗    ██████╗  ██████╗ ███╗   ██╗ ██████╗               ██████╗  █████╗ ██████╗ ██████╗ ██╗     ███████╗
██╔════╝ ██╔════╝ ██╔════╝╚══███╔╝    ██╔══██╗██╔═══██╗████╗  ██║██╔════╝               ██╔══██╗██╔══██╗██╔══██╗██╔══██╗██║     ██╔════╝
██║  ███╗██║  ███╗█████╗    ███╔╝     ██████╔╝██║   ██║██╔██╗ ██║██║  ███╗    █████╗    ██████╔╝███████║██║  ██║██║  ██║██║     █████╗  
██║   ██║██║   ██║██╔══╝   ███╔╝      ██╔═══╝ ██║   ██║██║╚██╗██║██║   ██║    ╚════╝    ██╔═══╝ ██╔══██║██║  ██║██║  ██║██║     ██╔══╝  
╚██████╔╝╚██████╔╝███████╗███████╗    ██║     ╚██████╔╝██║ ╚████║╚██████╔╝              ██║     ██║  ██║██████╔╝██████╔╝███████╗███████╗
 ╚═════╝  ╚═════╝ ╚══════╝╚══════╝    ╚═╝      ╚═════╝ ╚═╝  ╚═══╝ ╚═════╝               ╚═╝     ╚═╝  ╚═╝╚═════╝ ╚═════╝ ╚══════╝╚══════╝
                                                                                                                                        
 */

use ggez::{graphics::{Rect, Color, DrawParam, Drawable, self}, mint::Vector2};

use crate::common::Limits;


pub struct Paddle {
    rect: Option<Rect>,
    color: Color,
    limits: Limits,
}

impl Drawable for Paddle {
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

impl Default for Paddle {
    fn default() -> Self {
        Paddle { 
            color: Color::WHITE,
            rect: None,
            limits: Limits::default(),
        }
    }
}

#[allow(dead_code)]
impl Paddle {
    pub fn new() -> Self {
        Paddle { 
            ..Default::default()
        }
    }

    pub fn set_limit_min_x(mut self, limit: f32) -> Self {
        self.limits.min_x = Some(limit);
        self
    }

    pub fn set_limit_min_y(mut self, limit: f32) -> Self {
        self.limits.min_y = Some(limit);
        self
    }

    pub fn set_limit_max_x(mut self, limit: f32) -> Self {
        self.limits.max_x = Some(limit);
        self

    }
    pub fn set_limit_max_y(mut self, limit: f32) -> Self {
        self.limits.max_y = Some(limit);
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

    pub fn get_rect_ref(&self) -> &Option<Rect> {
        &self.rect
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        if let Some(rect) = &mut self.rect {
            rect.translate(Vector2 {
                x,
                y,
            });

            // Min position is set ?
            if let Ok(x) = self.limits.is_out_min_x(rect.x) {
                rect.x = x;
            }

            if let Ok(y) = self.limits.is_out_min_y(rect.y) {
                rect.y = y;
            }

            // Max position is set ?
            if let Ok(x) = self.limits.is_out_max_x(rect.x) {
                rect.x = x;
            }

            if let Ok(y) = self.limits.is_out_max_y(rect.y) {
                rect.y = y;
            }
        }
    }

}
