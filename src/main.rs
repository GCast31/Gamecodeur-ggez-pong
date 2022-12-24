/*
 ██████╗  ██████╗ ███████╗███████╗    ██████╗  ██████╗ ███╗   ██╗ ██████╗ 
██╔════╝ ██╔════╝ ██╔════╝╚══███╔╝    ██╔══██╗██╔═══██╗████╗  ██║██╔════╝ 
██║  ███╗██║  ███╗█████╗    ███╔╝     ██████╔╝██║   ██║██╔██╗ ██║██║  ███╗
██║   ██║██║   ██║██╔══╝   ███╔╝      ██╔═══╝ ██║   ██║██║╚██╗██║██║   ██║
╚██████╔╝╚██████╔╝███████╗███████╗    ██║     ╚██████╔╝██║ ╚████║╚██████╔╝
 ╚═════╝  ╚═════╝ ╚══════╝╚══════╝    ╚═╝      ╚═════╝ ╚═╝  ╚═══╝ ╚═════╝ 
      
 By GCast31 : December 2022 (Formation Gamecodeur.com)
 */

 ///
 /// For playing :
 /// 
 /// => Player 1 : Up : "Z" / Down : "S" (AZERTY)
 /// => Player 2 : Up : "UP" / Down : "DOWN"
 ///
 /// Reset => "F5"

mod ball;
mod paddle;
mod common;
mod count_screen;
mod score;
mod streak_manager;

use ball::Ball;
use common::Limits;
use count_screen::CountScreen;
use ggez::conf::{WindowSetup, WindowMode};
use ggez::input::keyboard;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, DrawParam, Drawable, Rect};
use ggez::event::{self, EventHandler};
use paddle::Paddle;
use score::Score;
use streak_manager::{StreakManager, Streak};

pub const BALL_COLOR: Color = Color::WHITE;
pub const BALL_WIDTH: f32 = 10.;
pub const BALL_HEIGHT: f32 = 10.;
pub const BALL_VELOCITY_MIN_X: f32 = 180.;
pub const BALL_VELOCITY_MAX_X: f32 = 300.;
pub const BALL_VELOCITY_MIN_Y: f32 = 180.;
pub const BALL_VELOCITY_MAX_Y: f32 = 300.;

pub const COUNT_SCREEN_DELAY: f32 = 2.;
pub const COUNT_SCREEN_COUNTER_COLOR: Color = Color::GREEN;
pub const COUNT_SCREEN_COUNTER_SIZE: f32 = 64.;

pub const PADDLE_WIDTH: f32 = 12.;
pub const PADDLE_HEIGHT: f32 = 75.;
pub const PADDLE_LEFT_COLOR: Color = Color::BLUE;
pub const PADDLE_RIGHT_COLOR: Color = Color::RED;
pub const PADDLE_MOVEMENT: f32 = 260.;

pub const STREAK_LIFE: f32 = 1.;

pub const SCREEN_WIDTH: f32 = 600.;
pub const SCREEN_HEIGHT: f32 = 300.;

#[derive(PartialEq)]
enum GameState {
    CountScreen,
    Game,
}

struct PongGame {
    state: GameState,
    l_paddle: Paddle,
    r_paddle: Paddle,
    ball: Ball,
    l_score: Score,
    r_score: Score,
    streaks: StreakManager,
    count_screen: CountScreen,
}

fn main() {
    // Make a context
    let (ctx, event_loop) = 
        ContextBuilder::new("GGEZ Pong", "GCast31")
            .window_setup(WindowSetup {
                title: "GGEZ Pong".to_string(),
                ..Default::default()
            })
            .window_mode(WindowMode {
                height: SCREEN_HEIGHT,
                width: SCREEN_WIDTH, 
                ..Default::default()
            })
            .build()
            .expect("Could not create ggez context");
    
    // Make informations of game
    let mut my_game = PongGame {
        l_paddle: Paddle::new()
                    .set_rect(0., SCREEN_HEIGHT / 2. - PADDLE_HEIGHT / 2., PADDLE_WIDTH, PADDLE_HEIGHT)
                    .set_color(PADDLE_LEFT_COLOR)
                    .set_limit_min_y(0.)
                    .set_limit_max_y(SCREEN_HEIGHT - PADDLE_HEIGHT),
        r_paddle: Paddle::new()
                    .set_rect(SCREEN_WIDTH - PADDLE_WIDTH, SCREEN_HEIGHT / 2. - PADDLE_HEIGHT / 2., PADDLE_WIDTH, PADDLE_HEIGHT)
                    .set_color(PADDLE_RIGHT_COLOR)
                    .set_limit_min_y(0.)
                    .set_limit_max_y(SCREEN_HEIGHT - PADDLE_HEIGHT),
        ball: Ball::new()
            .set_default_rect(SCREEN_WIDTH / 2. - BALL_WIDTH / 2., SCREEN_HEIGHT / 2. - BALL_HEIGHT / 2., BALL_WIDTH, BALL_HEIGHT)
            .set_rect(SCREEN_WIDTH / 2. - BALL_WIDTH / 2., SCREEN_HEIGHT / 2. - BALL_HEIGHT / 2., BALL_WIDTH, BALL_HEIGHT)
            .set_color(BALL_COLOR)
            .set_limits_bounce(Limits {
                min_y: Some(0.),
                max_y: Some(SCREEN_HEIGHT),
                ..Default::default()
            })
            .set_limits_out(Limits {
                min_x: Some(0.),
                max_x: Some(SCREEN_WIDTH),
                ..Default::default()
            })
            .set_velocity_range(BALL_VELOCITY_MIN_X, BALL_VELOCITY_MAX_X, BALL_VELOCITY_MIN_Y, BALL_VELOCITY_MAX_Y, true)
            ,
        l_score:
            Score::new(SCREEN_WIDTH / 2. - 50.,10.)
                .set_color(PADDLE_LEFT_COLOR)
                .set_size_in_pixels(24.)
                .build(),
        r_score: 
            Score::new(SCREEN_WIDTH / 2. + 50., 10.)
                .set_color(PADDLE_RIGHT_COLOR)
                .set_size_in_pixels(24.)
                .build(),
        streaks: StreakManager::default(),
        state: GameState::CountScreen,
        count_screen: CountScreen {
                    rect: Rect {
                        x: 0.,
                        y: 0.,
                        h: SCREEN_HEIGHT,
                        w: SCREEN_WIDTH,
                    },
                    actual_delay: COUNT_SCREEN_DELAY,
                    delay_in_seconds: COUNT_SCREEN_DELAY,
                    ..Default::default()
                }
                .set_counter_color(COUNT_SCREEN_COUNTER_COLOR)
                .set_size_pixel_counter(Some(COUNT_SCREEN_COUNTER_SIZE))

    };

    my_game.ball.apply_default_rect();

    // Run
    event::run(ctx, event_loop, my_game);
}

impl PongGame {
    /// Reset the game
    /// @Parm "score_zero" : true if scores is set to zero
    fn reset(&mut self, scores_zero: bool) {
        self.state = GameState::CountScreen;
        self.count_screen.reset();
        self.streaks.reset();

        if scores_zero {
            self.l_score.reset_score();
            self.r_score.reset_score();
        }
    } 
}

impl EventHandler for PongGame {

    /// Update games elements each frames
    fn update(&mut self, ctx: &mut Context) -> GameResult {

        let dt = ctx.time.delta().as_secs_f32();

        let l_rect = self.l_paddle.dimensions(ctx);
        let r_rect = self.r_paddle.dimensions(ctx);
        let b_rect = self.ball.dimensions(ctx);

        // F5-Restart game
        if ctx.keyboard.is_key_just_pressed(keyboard::KeyCode::F5) {
            self.reset(true);
            return Ok(());
        }

        // Paddles - change directions
        // Players can move padles on count screen
        if ctx.keyboard.is_key_pressed(keyboard::KeyCode::Z) {
            self.l_paddle.translate(0., -PADDLE_MOVEMENT * dt);
        }

        if ctx.keyboard.is_key_pressed(keyboard::KeyCode::S) {
            self.l_paddle.translate(0., PADDLE_MOVEMENT * dt);
        }

        if ctx.keyboard.is_key_pressed(keyboard::KeyCode::Up) {
            self.r_paddle.translate(0., -PADDLE_MOVEMENT * dt);
        }

        if ctx.keyboard.is_key_pressed(keyboard::KeyCode::Down) {
            self.r_paddle.translate(0., PADDLE_MOVEMENT * dt);
        }

        // Only when game is active
        if self.state != GameState::CountScreen {
            
            // Ball
            // => Move ball
            self.ball.apply_velocity(dt);

            // => Check if ball collide with paddles
            if self.ball.collide(&l_rect) {
                self.ball.velocity.x *= -1.;
                self.ball.set_x(l_rect.unwrap().x + l_rect.unwrap().w).unwrap();
            } else if self.ball.collide(&r_rect) {
                self.ball.velocity.x *= -1.;
                self.ball.set_x(r_rect.unwrap().x - b_rect.unwrap().w).unwrap();
                
            }
      
            // *** Streaks
            self.streaks.update(dt);
            self.streaks.add_streak(Streak {
                color: self.ball.get_color(),
                life: STREAK_LIFE, 
                rect: Rect {
                    x: b_rect.unwrap().x,
                    y: b_rect.unwrap().y,
                    h: b_rect.unwrap().h,
                    w: b_rect.unwrap().w,
                },
                ..Default::default()
            });

            // => Check if ball is out
            match self.ball.is_out() {
                common::Direction::Bottom => {},
                common::Direction::Top => {},
                common::Direction::Left => {
                    self.ball.reset(true, true);
                    self.r_score.add();
                    self.reset(false);
                },
                common::Direction::Right => {
                    self.ball.reset(true, true);
                    self.l_score.add();
                    self.reset(false);
                    
                },
                common::Direction::None => {},
            };
        }

        // When count screen is finish we can play
        if self.state == GameState::CountScreen {
            self.count_screen.update(dt);
            if self.count_screen.is_finished() {
                self.state = GameState::Game;
            }
        }


        Ok(())
    }

    /// Draw each elems at each frame
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
       
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        let dp = DrawParam::default();

        canvas.draw(&self.l_paddle, dp);
        canvas.draw(&self.r_paddle, dp);

        canvas.draw(&self.l_score.text, self.l_score.draw_param);
        canvas.draw(&self.r_score.text, self.r_score.draw_param);

        // While count screen is playing, ball is disabled
        if self.state == GameState::CountScreen {
          canvas.draw(&self.count_screen, dp);
        } else {
          canvas.draw(&self.ball, dp);
        }

        self.streaks.draw(&mut canvas);

        canvas.finish(ctx)
    }
}