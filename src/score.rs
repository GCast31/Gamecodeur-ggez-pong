/*
 ██████╗  ██████╗ ███████╗███████╗    ██████╗  ██████╗ ███╗   ██╗ ██████╗               ███████╗ ██████╗ ██████╗ ██████╗ ███████╗
██╔════╝ ██╔════╝ ██╔════╝╚══███╔╝    ██╔══██╗██╔═══██╗████╗  ██║██╔════╝               ██╔════╝██╔════╝██╔═══██╗██╔══██╗██╔════╝
██║  ███╗██║  ███╗█████╗    ███╔╝     ██████╔╝██║   ██║██╔██╗ ██║██║  ███╗    █████╗    ███████╗██║     ██║   ██║██████╔╝█████╗  
██║   ██║██║   ██║██╔══╝   ███╔╝      ██╔═══╝ ██║   ██║██║╚██╗██║██║   ██║    ╚════╝    ╚════██║██║     ██║   ██║██╔══██╗██╔══╝  
╚██████╔╝╚██████╔╝███████╗███████╗    ██║     ╚██████╔╝██║ ╚████║╚██████╔╝              ███████║╚██████╗╚██████╔╝██║  ██║███████╗
 ╚═════╝  ╚═════╝ ╚══════╝╚══════╝    ╚═╝      ╚═════╝ ╚═╝  ╚═══╝ ╚═════╝               ╚══════╝ ╚═════╝ ╚═════╝ ╚═╝  ╚═╝╚══════╝
                                                                                                                                 
 */

use ggez::{graphics::{Text, TextFragment, Color, DrawParam, PxScale}, mint::Vector2};

pub struct Score {
    score: u32,
    pub text: Text,
    pub draw_param: DrawParam,
    size_in_pixels: Option<f32>,
    color: Color,
    pub x: f32,
    pub y: f32,
}

impl Score {
    pub fn new(x: f32, y: f32) -> Self {

        Score { 
            score: 0, 
            text: Text::default(),
            color: Color::WHITE,
            x,
            y,
            draw_param: DrawParam::default(),
            size_in_pixels: None,
        }

    }

    pub fn reset_score(&mut self) {
        self.score = 0;
        self.update();
    }

    pub fn build(mut self) -> Self {
        self.update();
        self
    }

    pub fn set_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn set_size_in_pixels(mut self, size: f32) -> Self {
        self.size_in_pixels = Some(size);
        self
    }

    fn update(&mut self) {
        self.draw_param = DrawParam::default().dest(Vector2 {
            x: self.x,
            y: self.y,
        });

        let scale: Option<PxScale> = 
                if let Some(size) = self.size_in_pixels {
                    Some(PxScale::from(size))
                } else {
                    None
                };

        let tf: TextFragment = TextFragment { 
            color: Some(self.color),
            scale,
            text: self.score.to_string(),
            ..Default::default()
        };

        self.text = Text::new(tf);
    }

    pub fn add(&mut self) {
        self.score += 1;
        self.update();
    }

}