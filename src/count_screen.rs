/*
 ██████╗  ██████╗ ███████╗███████╗    ██████╗  ██████╗ ███╗   ██╗ ██████╗                ██████╗ ██████╗ ██╗   ██╗███╗   ██╗████████╗    ███████╗ ██████╗██████╗ ███████╗███████╗███╗   ██╗
██╔════╝ ██╔════╝ ██╔════╝╚══███╔╝    ██╔══██╗██╔═══██╗████╗  ██║██╔════╝               ██╔════╝██╔═══██╗██║   ██║████╗  ██║╚══██╔══╝    ██╔════╝██╔════╝██╔══██╗██╔════╝██╔════╝████╗  ██║
██║  ███╗██║  ███╗█████╗    ███╔╝     ██████╔╝██║   ██║██╔██╗ ██║██║  ███╗    █████╗    ██║     ██║   ██║██║   ██║██╔██╗ ██║   ██║       ███████╗██║     ██████╔╝█████╗  █████╗  ██╔██╗ ██║
██║   ██║██║   ██║██╔══╝   ███╔╝      ██╔═══╝ ██║   ██║██║╚██╗██║██║   ██║    ╚════╝    ██║     ██║   ██║██║   ██║██║╚██╗██║   ██║       ╚════██║██║     ██╔══██╗██╔══╝  ██╔══╝  ██║╚██╗██║
╚██████╔╝╚██████╔╝███████╗███████╗    ██║     ╚██████╔╝██║ ╚████║╚██████╔╝              ╚██████╗╚██████╔╝╚██████╔╝██║ ╚████║   ██║       ███████║╚██████╗██║  ██║███████╗███████╗██║ ╚████║
 ╚═════╝  ╚═════╝ ╚══════╝╚══════╝    ╚═╝      ╚═════╝ ╚═╝  ╚═══╝ ╚═════╝                ╚═════╝ ╚═════╝  ╚═════╝ ╚═╝  ╚═══╝   ╚═╝       ╚══════╝ ╚═════╝╚═╝  ╚═╝╚══════╝╚══════╝╚═╝  ╚═══╝
                                                                                                                                                                                           
 */

use ggez::{graphics::{Color, Rect, Drawable, self, Text, PxScale, TextFragment, DrawParam}, mint::Vector2};

pub struct CountScreen {
    pub color: Color,
    pub rect: Rect,
    pub delay_in_seconds: f32,
    pub actual_delay: f32,
    pub actual_delay_u: i32,
    pub size_pixel_counter: Option<f32>,
    pub counter_color: Color,
    pub text: Text,
}

impl Default for CountScreen {
    fn default() -> Self {
        CountScreen { 
            color: Color { r: 0., g: 0., b: 0., a: 0.8 }, 
            rect: Rect::default(), 
            delay_in_seconds: 0., 
            actual_delay: 0., 
            actual_delay_u: 0,
            size_pixel_counter: None,
            text: Text::default(),
            counter_color: Color::WHITE,
        }
    }
}

#[allow(dead_code)]
impl CountScreen {
    pub fn reset(&mut self) {
        self.actual_delay = self.delay_in_seconds;
        self.actual_delay_u = 0;
    }

    pub fn set_counter_color(mut self, color: Color) -> Self {
        self.counter_color = color;
        self
    }

    pub fn set_size_pixel_counter(mut self, size: Option<f32>) -> Self {
        self.size_pixel_counter = size;
        self
    }

    pub fn update(&mut self, dt: f32) {

        if !self.is_finished() {
            self.actual_delay -= dt;

            let delay_u = self.actual_delay.trunc() as i32 + 1;

            // Change text only if delay change
            if self.actual_delay_u != delay_u {

                self.actual_delay_u = delay_u;

                let scale: Option<PxScale> = 
                if let Some(size) = self.size_pixel_counter {
                    Some(PxScale::from(size))
                } else {
                    None
                };
        
                let tf: TextFragment = TextFragment { 
                    color: Some(self.counter_color),
                    scale,
                    text: self.actual_delay_u.to_string(),
                    ..Default::default()
                };

                self.text = Text::new(tf);
            }
        }

 

    }

    pub fn is_finished(&self) -> bool {
        self.actual_delay <= 0.
    }
}

impl Drawable for CountScreen {
    fn dimensions(&self, _gfx: &impl ggez::context::Has<ggez::graphics::GraphicsContext>) -> Option<Rect> {
        Some(self.rect)
    }

    fn draw(&self, canvas: &mut ggez::graphics::Canvas, _param: impl Into<ggez::graphics::DrawParam>) {

        let (counter_x, counter_y) = (self.rect.w / 2., self.rect.h / 2. - 20.);

        canvas.draw(
            &graphics::Quad, 
            graphics::DrawParam::new()
                .dest(self.rect.point())
                .color(self.color)
                .scale(self.rect.size())
        );
        canvas.draw(
            &self.text, 
            DrawParam::default().dest(Vector2 {
                x: counter_x,
                y: counter_y,
            })
        );
    }
}