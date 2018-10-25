extern crate ggez;
extern crate fallingball;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use ggez::timer;

use fallingball::ball::{self,Ball};
use fallingball::bar::HorizontalBar;

#[derive(Debug)]
struct MainState {
    ball: Ball,
    is_start:bool,
}

impl MainState {
     fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let window_size = graphics::get_size(_ctx);
        let s = MainState{
            ball:Ball::new(
                MainState::get_ball_radius(_ctx),
                (window_size.0 as f32/2.0,window_size.1 as f32),
                (0f32,MainState::get_max_velocity(_ctx),ball::BALL_VELOCITY_MAX)),
            is_start:false,
        };
        Ok(s)
    }

    fn restore(&mut self,ctx:&Context){
        let window_size = graphics::get_size(ctx);
        self.ball.restore(
            MainState::get_ball_radius(ctx),
            (window_size.0 as f32/2.0,window_size.1 as f32),
            (0f32,MainState::get_max_velocity(ctx),ball::BALL_VELOCITY_MAX)
            );
        self.is_start = false;
    }

    fn get_ball_radius(ctx:&Context) ->f32{
        let window_size = graphics::get_size(ctx);
        window_size.0 as f32 / 80f32
    }

    fn get_max_velocity(ctx:&Context) ->f32{
        let window_size = graphics::get_size(ctx);
        window_size.1 as f32 / -1.8
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.is_start{
            let time_delta =  ((ggez::timer::get_delta(ctx)).subsec_millis() as f32)/1e3; 
            self.ball.update(time_delta);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_background_color(ctx, graphics::Color::new(1.0,1.0,1.0,1.0));
        self.ball.draw(ctx)?;
        graphics::present(ctx);
        timer::yield_now();
        Ok(())
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: event::MouseButton, x: i32, y: i32) {
        match button {
            event::MouseButton::Left => {
                let winsows_size = graphics::get_size(_ctx);
                self.ball.set_direction_vec(((x - winsows_size.0 as i32/2) as f32,(y - winsows_size.1 as i32) as f32)) ;         
                self.is_start = true;
            },
            event::MouseButton::Right => {self.restore(_ctx);},
            _ =>{},
        }
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("falling ball", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}