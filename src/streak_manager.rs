/*
 ██████╗  ██████╗ ███████╗███████╗    ██████╗  ██████╗ ███╗   ██╗ ██████╗               ███████╗████████╗██████╗ ███████╗ █████╗ ██╗  ██╗    ███╗   ███╗ █████╗ ███╗   ██╗ █████╗  ██████╗ ███████╗██████╗ 
██╔════╝ ██╔════╝ ██╔════╝╚══███╔╝    ██╔══██╗██╔═══██╗████╗  ██║██╔════╝               ██╔════╝╚══██╔══╝██╔══██╗██╔════╝██╔══██╗██║ ██╔╝    ████╗ ████║██╔══██╗████╗  ██║██╔══██╗██╔════╝ ██╔════╝██╔══██╗
██║  ███╗██║  ███╗█████╗    ███╔╝     ██████╔╝██║   ██║██╔██╗ ██║██║  ███╗    █████╗    ███████╗   ██║   ██████╔╝█████╗  ███████║█████╔╝     ██╔████╔██║███████║██╔██╗ ██║███████║██║  ███╗█████╗  ██████╔╝
██║   ██║██║   ██║██╔══╝   ███╔╝      ██╔═══╝ ██║   ██║██║╚██╗██║██║   ██║    ╚════╝    ╚════██║   ██║   ██╔══██╗██╔══╝  ██╔══██║██╔═██╗     ██║╚██╔╝██║██╔══██║██║╚██╗██║██╔══██║██║   ██║██╔══╝  ██╔══██╗
╚██████╔╝╚██████╔╝███████╗███████╗    ██║     ╚██████╔╝██║ ╚████║╚██████╔╝              ███████║   ██║   ██║  ██║███████╗██║  ██║██║  ██╗    ██║ ╚═╝ ██║██║  ██║██║ ╚████║██║  ██║╚██████╔╝███████╗██║  ██║
 ╚═════╝  ╚═════╝ ╚══════╝╚══════╝    ╚═╝      ╚═════╝ ╚═╝  ╚═══╝ ╚═════╝               ╚══════╝   ╚═╝   ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝    ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚═╝  ╚═╝
                                                                                                                                                                                                           
 */

use ggez::graphics::{Canvas, Rect, DrawParam, Drawable, self, Color};

pub struct Streak {
    pub rect: Rect,
    pub life: f32,
    pub color: Color,
}

impl Default for Streak {
    fn default() -> Self {
        Self {  
            rect: Rect::default(),
            life: 0.,
            color: Color::WHITE,
        }
    }
}

impl Drawable for Streak {
    fn dimensions(&self, _gfx: &impl ggez::context::Has<ggez::graphics::GraphicsContext>) -> Option<Rect> {
        Some(self.rect)
    }

    fn draw(&self, canvas: &mut Canvas, _param: impl Into<DrawParam>) {
        if self.life > 0. {

            canvas.draw(
                &graphics::Quad, 
                graphics::DrawParam::new()
                    .dest(self.rect.point())
                    .color(self.color)
                    .scale(self.rect.size()
                )
            );
        }
    }
}

impl Streak {
    fn update(&mut self, dt: f32) {
        self.color.a *= 0.8;
        self.life -= dt;
    }

    fn is_dead(&self) -> bool {
        self.life <= 0.
    }
}

pub struct StreakManager {
    streaks: Vec<Streak>,
}

impl Default for StreakManager {
    fn default() -> Self {
        Self { 
            streaks: Vec::new(), 
        }
    }
}

impl StreakManager {
   
    pub fn add_streak(&mut self, streak: Streak) {
        self.streaks.push(streak);
    }

    pub fn reset(&mut self) {
        self.streaks.clear();
    }

    pub fn update(&mut self, dt: f32) {
        for streak in self.streaks.iter_mut() {
            streak.update(dt);
        }
        self.streaks.retain(|x| !x.is_dead());
    }

    pub fn draw(&self, canvas: &mut Canvas) {

        let dp = DrawParam::default();

        for streak in self.streaks.iter() {
            canvas.draw(streak, dp);
        }
    }
}